use std::collections::{HashMap, HashSet};
use std::sync::Arc;
use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};
use tokio::net::TcpListener;
use tokio::sync::{Mutex, mpsc};

struct AppState {
    clients: HashMap<String, mpsc::Sender<String>>,
    groups: HashMap<String, HashSet<String>>,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let listener = TcpListener::bind("127.0.0.1:8080").await?;
    println!("Server started on 127.0.0.1:8080");

    let state = Arc::new(Mutex::new(AppState {
        clients: HashMap::new(),
        groups: HashMap::new(),
    }));

    state
        .lock()
        .await
        .groups
        .insert("global".to_string(), HashSet::new());

    loop {
        let (socket, _addr) = listener.accept().await?;
        let state = Arc::clone(&state);

        tokio::spawn(async move {
            let (reader, mut writer) = socket.into_split();
            let mut reader = BufReader::new(reader);
            let mut nickname = String::new();

            if reader.read_line(&mut nickname).await.unwrap_or(0) == 0 {
                return;
            }
            let nickname = nickname.trim().to_string();

            let (tx, mut rx) = mpsc::channel::<String>(100);

            {
                let mut st = state.lock().await;
                if st.clients.contains_key(&nickname) {
                    let _ = writer.write_all(b"TAKEN\n").await;
                    return;
                }

                st.clients.insert(nickname.clone(), tx.clone());
                st.groups
                    .get_mut("global")
                    .unwrap()
                    .insert(nickname.clone());

                let sys_msg = format!("*** System: {} joined global ***\n", nickname);
                for member in st.groups.get("global").unwrap() {
                    if member != &nickname {
                        if let Some(member_tx) = st.clients.get(member) {
                            let _ = member_tx.send(sys_msg.clone()).await;
                        }
                    }
                }
            }
            let _ = writer.write_all(b"OK\n").await;

            tokio::spawn(async move {
                while let Some(msg) = rx.recv().await {
                    if writer.write_all(msg.as_bytes()).await.is_err() {
                        break;
                    }
                }
            });

            let mut line = String::new();
            while let Ok(n) = reader.read_line(&mut line).await {
                if n == 0 {
                    break;
                }
                let input = line.trim();
                if input.is_empty() {
                    continue;
                }

                let mut st = state.lock().await;
                let parts: Vec<&str> = input.splitn(3, ' ').collect();
                let cmd = parts[0];

                match cmd {
                    "/join" if parts.len() == 2 => {
                        let group = parts[1].to_string();
                        st.groups
                            .entry(group.clone())
                            .or_insert_with(HashSet::new)
                            .insert(nickname.clone());
                        let sys_msg = format!("*** System: {} joined {} ***\n", nickname, group);

                        for member in st.groups.get(&group).unwrap() {
                            if let Some(member_tx) = st.clients.get(member) {
                                let _ = member_tx
                                    .send(if member == &nickname {
                                        format!("*** System: You joined {} ***\n", group)
                                    } else {
                                        sys_msg.clone()
                                    })
                                    .await;
                            }
                        }
                    }
                    "/leave" if parts.len() == 2 => {
                        let group = parts[1];
                        if group == "global" {
                            let _ = tx
                                .send(
                                    "*** System: You cannot leave the global group ***\n"
                                        .to_string(),
                                )
                                .await;
                        } else {
                            let mut left_successfully = false;
                            if let Some(members) = st.groups.get_mut(group) {
                                left_successfully = members.remove(&nickname);
                            }
                            if left_successfully {
                                let _ = tx
                                    .send(format!("*** System: You left {} ***\n", group))
                                    .await;
                                let sys_msg =
                                    format!("*** System: {} left {} ***\n", nickname, group);

                                if let Some(members) = st.groups.get(group) {
                                    for member in members.iter() {
                                        if let Some(member_tx) = st.clients.get(member) {
                                            let _ = member_tx.send(sys_msg.clone()).await;
                                        }
                                    }
                                }
                            }
                        }
                    }
                    "/list" if parts.len() == 2 => {
                        let group = parts[1];
                        if let Some(members) = st.groups.get(group) {
                            let users: Vec<String> = members.iter().cloned().collect();
                            let _ = tx
                                .send(format!(
                                    "*** System: Users in {}: {} ***\n",
                                    group,
                                    users.join(", ")
                                ))
                                .await;
                        } else {
                            let _ = tx
                                .send(format!("*** System: Group {} does not exist ***\n", group))
                                .await;
                        }
                    }
                    "/g" if parts.len() == 3 => {
                        let group = parts[1];
                        let msg = parts[2];
                        if let Some(members) = st.groups.get(group) {
                            if members.contains(&nickname) {
                                let formatted =
                                    format!("[Group: {}] {}: {}\n", group, nickname, msg);
                                for member in members.iter() {
                                    if let Some(member_tx) = st.clients.get(member) {
                                        let _ = member_tx.send(formatted.clone()).await;
                                    }
                                }
                            } else {
                                let _ = tx
                                    .send(format!(
                                        "*** System: You are not a member of {} ***\n",
                                        group
                                    ))
                                    .await;
                            }
                        }
                    }
                    "/msg" if parts.len() == 3 => {
                        let target = parts[1];
                        let msg = parts[2];
                        if target == nickname {
                            let _ = tx.send("*** System: You cannot send a private message to yourself ***\n".to_string()).await;
                        } else if let Some(target_tx) = st.clients.get(target) {
                            let _ = target_tx
                                .send(format!("[Private from {}] {}\n", nickname, msg))
                                .await;
                            let _ = tx.send(format!("[Private to {}] {}\n", target, msg)).await;
                        } else {
                            let _ = tx
                                .send(format!("*** System: User {} is not online ***\n", target))
                                .await;
                        }
                    }
                    _ => {
                        let msg = input;
                        let formatted = format!("[Group: global] {}: {}\n", nickname, msg);
                        if let Some(members) = st.groups.get("global") {
                            for member in members.iter() {
                                if let Some(member_tx) = st.clients.get(member) {
                                    let _ = member_tx.send(formatted.clone()).await;
                                }
                            }
                        }
                    }
                }
                line.clear();
            }

            let mut st = state.lock().await;
            st.clients.remove(&nickname);

            let mut affected_groups = Vec::new();
            for (group_name, members) in st.groups.iter_mut() {
                if members.remove(&nickname) {
                    affected_groups.push(group_name.clone());
                }
            }

            for group in affected_groups {
                let sys_msg = format!(
                    "*** System: {} left {} (Disconnected) ***\n",
                    nickname, group
                );
                if let Some(members) = st.groups.get(&group) {
                    for member in members.iter() {
                        if let Some(tx) = st.clients.get(member) {
                            let _ = tx.send(sys_msg.clone()).await;
                        }
                    }
                }
            }
        });
    }
}
