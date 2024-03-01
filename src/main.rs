use std::io;
use rusqlite::{Connection, Result};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let connection = Connection::open("notes.db")?;
    connection.execute(
        "CREATE TABLE IF NOT EXISTS notes (
            id INTEGER PRIMARY KEY,
            body TEXT NOT NULL UNIQUE
        )",
        [],
    )?;


    let mut running = true ;
    while running == true {
        let mut buffer = String::new();
        io::stdin().read_line(&mut buffer)?;
        let trimmed_body = buffer.trim();
        let cmd_split = trimmed_body.split_once(" ");
        let mut cmd = trimmed_body;
        let mut message = "";
        if cmd_split != None {
            cmd = cmd_split.unwrap().0;
            message = cmd_split.unwrap().1;
        }
        if cmd == "/del" {
            let id = message;
            connection.execute("DELETE FROM notes WHERE id = (?1)", [id])?;
        } else if cmd == "/edit" {
            let message_split = message.split_once(" ").unwrap();
            let id = message_split.0;
            let body = message_split.1;
            connection.execute("UPDATE notes SET body = (?1) WHERE id = (?2)", [body, id])?;
        }else if trimmed_body == "" {
            running = false;
        } else if trimmed_body == "/list" {
            let mut stmt = connection.prepare("SELECT id, body FROM notes")?;
            let mut rows = stmt.query(rusqlite::params![])?;
            while let Some(row) = rows.next()? {
                let id: i32 = row.get(0)?;
                let body: String = row.get(1)?;
                println!("{} {}", id, body.to_string());
            }
        } else {
            connection.execute("INSERT INTO notes (body) VALUES (?1)", [trimmed_body])?;
        }
    }
    Ok(())
}
