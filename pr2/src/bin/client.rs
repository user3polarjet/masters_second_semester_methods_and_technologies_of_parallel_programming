use crossterm::{
    cursor, execute,
    style::{self, Stylize},
    terminal,
};
use std::io::{Write, stdout};
use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};
use tokio::net::TcpStream;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    print!("Enter your unique nickname: ");
    stdout().flush()?;
    let mut nickname = String::new();
    std::io::stdin().read_line(&mut nickname)?;

    let stream = TcpStream::connect("127.0.0.1:8080").await?;
    let (reader, mut writer) = stream.into_split();
    let mut reader = BufReader::new(reader);
    let mut stdin_reader = BufReader::new(tokio::io::stdin());

    writer.write_all(nickname.as_bytes()).await?;
    let mut auth_resp = String::new();
    reader.read_line(&mut auth_resp).await?;

    if auth_resp.trim() == "TAKEN" {
        println!(
            "{}",
            "Error: Nickname is active. Disconnecting.".red().bold()
        );
        return Ok(());
    }

    execute!(
        stdout(),
        terminal::Clear(terminal::ClearType::All),
        cursor::MoveTo(0, 0)
    )?;

    println!("{}", " --- Distributed Chat System --- ".bold().cyan());
    println!(
        "{}",
        format!("Logged in as: {}", nickname.trim())
            .italic()
            .dark_grey()
    );
    println!("{}", "Commands:".yellow());
    println!(
        "{}",
        "  <text>                - Sends to 'global' group".dark_grey()
    );
    println!(
        "{}",
        "  /join <group>         - Join a new or existing group".dark_grey()
    );
    println!("{}", "  /leave <group>        - Leave a group".dark_grey());
    println!(
        "{}",
        "  /list <group>         - View active members in a group".dark_grey()
    );
    println!(
        "{}",
        "  /g <group> <text>     - Message a specific group".dark_grey()
    );
    println!(
        "{}",
        "  /msg <user> <text>    - Send a private direct message".dark_grey()
    );
    println!("{}", "=".repeat(60).dark_grey());

    loop {
        execute!(
            stdout(),
            cursor::MoveToColumn(0),
            terminal::Clear(terminal::ClearType::CurrentLine),
            style::Print(" > ".green().bold())
        )?;
        stdout().flush()?;

        let mut server_msg = String::new();
        let mut user_input = String::new();

        tokio::select! {
            res = reader.read_line(&mut server_msg) => {
                if res.unwrap_or(0) == 0 { break; }

                execute!(stdout(), cursor::MoveToColumn(0), terminal::Clear(terminal::ClearType::CurrentLine))?;

                let msg = server_msg.trim();
                if msg.starts_with("*** System:") {
                    println!("{}", msg.yellow());
                } else if msg.starts_with("[Private") {
                    println!("{}", msg.magenta().bold());
                } else if msg.starts_with("[Group:") {
                    // Differentiate global from other groups visually
                    if msg.starts_with("[Group: global]") {
                        println!("{}", msg.white());
                    } else {
                        println!("{}", msg.cyan());
                    }
                } else {
                    println!("{}", msg);
                }
            }
            res = stdin_reader.read_line(&mut user_input) => {
                if res.is_ok() && !user_input.trim().is_empty() {
                    writer.write_all(user_input.as_bytes()).await?;
                    execute!(stdout(), cursor::MoveUp(1), cursor::MoveToColumn(0), terminal::Clear(terminal::ClearType::CurrentLine))?;
                }
            }
        }
    }

    execute!(
        stdout(),
        cursor::MoveToColumn(0),
        terminal::Clear(terminal::ClearType::CurrentLine)
    )?;
    println!("{}", "Disconnected from server.".red());
    Ok(())
}
