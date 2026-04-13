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
    print!("Enter your nickname: ");
    stdout().flush()?;
    let mut nickname = String::new();
    std::io::stdin().read_line(&mut nickname)?;

    let mut stream = TcpStream::connect("127.0.0.1:8080").await?;
    let (reader, mut writer) = stream.split();
    let mut reader = BufReader::new(reader);
    let mut stdin_reader = BufReader::new(tokio::io::stdin());

    writer.write_all(nickname.as_bytes()).await?;

    let mut auth_resp = String::new();
    reader.read_line(&mut auth_resp).await?;
    if auth_resp.trim() == "TAKEN" {
        println!(
            "{}",
            "Error: Nickname is already in use. Try another one."
                .red()
                .bold()
        );
        return Ok(());
    }

    // 2. UI Initialization Phase
    execute!(
        stdout(),
        terminal::Clear(terminal::ClearType::All),
        cursor::MoveTo(0, 0)
    )?;

    println!(
        "{}",
        " --- Distributed Parallel Chat System --- ".bold().cyan()
    );
    println!(
        "{}",
        format!("Logged in as: {}", nickname.trim())
            .italic()
            .dark_grey()
    );
    println!("{}", "=".repeat(50).dark_grey());

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

                execute!(
                    stdout(),
                    cursor::MoveToColumn(0),
                    terminal::Clear(terminal::ClearType::CurrentLine)
                )?;

                if server_msg.starts_with("***") {
                    println!("{}", server_msg.trim().yellow());
                } else {
                    print!("{}", "Msg | ".magenta().bold());
                    println!("{}", server_msg.trim());
                }
                println!("{}", "-".repeat(30).dark_grey());
            }

            res = stdin_reader.read_line(&mut user_input) => {
                if res.is_ok() && !user_input.trim().is_empty() {
                    writer.write_all(user_input.as_bytes()).await?;

                    execute!(
                        stdout(),
                        cursor::MoveUp(1),
                        cursor::MoveToColumn(0),
                        terminal::Clear(terminal::ClearType::CurrentLine)
                    )?;

                    print!("{}", "You | ".blue().bold());
                    println!("{}", user_input.trim());
                    println!("{}", "-".repeat(30).dark_grey());
                }
            }
        }
    }

    execute!(
        stdout(),
        terminal::Clear(terminal::ClearType::All),
        cursor::MoveTo(0, 0)
    )?;
    println!("Disconnected from server.");
    Ok(())
}
