fn run_pipes(num: u32) {
    let start = std::time::Instant::now();
    
    let mut child = std::process::Command::new("python3")
        .arg("aux_process.py")
        .arg("pipe")
        .stdin(std::process::Stdio::piped())
        .stdout(std::process::Stdio::piped())
        .spawn()
        .unwrap();

    let mut stdin = child.stdin.take().unwrap();
    std::io::Write::write_all(&mut stdin, std::format!("{}\n", num).as_bytes()).unwrap();
    std::mem::drop(stdin);

    let mut stdout = child.stdout.take().unwrap();
    let mut response = std::string::String::new();
    std::io::Read::read_to_string(&mut stdout, &mut response).unwrap();
    
    child.wait().unwrap();
    std::println!("Pipes: {} -> {} ({}ms)", num, response.trim(), start.elapsed().as_millis());
}

fn run_sockets(num: u32) {
    let start = std::time::Instant::now();
    let listener = std::net::TcpListener::bind("127.0.0.1:0").unwrap();
    let port = listener.local_addr().unwrap().port();

    let mut child = std::process::Command::new("python3")
        .arg("aux_process.py")
        .arg("socket")
        .arg(std::format!("{}", port))
        .spawn()
        .unwrap();

    let (mut stream, _) = listener.accept().unwrap();
    std::io::Write::write_all(&mut stream, std::format!("{}\n", num).as_bytes()).unwrap();

    let mut response = std::string::String::new();
    std::io::Read::read_to_string(&mut stream, &mut response).unwrap();

    child.wait().unwrap();
    std::println!("Sockets: {} -> {} ({}ms)", num, response.trim(), start.elapsed().as_millis());
}

fn run_shared_file(num: u32) {
    let start = std::time::Instant::now();
    let file_path = "shared_data.txt";
    
    std::fs::write(file_path, std::format!("{}", num)).unwrap();

    let mut child = std::process::Command::new("python3")
        .arg("aux_process.py")
        .arg("file")
        .arg(file_path)
        .spawn()
        .unwrap();

    child.wait().unwrap();

    let response = std::fs::read_to_string(file_path).unwrap();
    std::println!("File: {} -> {} ({}ms)", num, response.trim(), start.elapsed().as_millis());
    std::fs::remove_file(file_path).unwrap();
}

fn main() {
    let num = (std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_nanos() % 1000) as u32;
    
    run_pipes(num);
    run_sockets(num);
    run_shared_file(num);
}
