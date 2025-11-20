use crossterm::{
    QueueableCommand,
    cursor::{MoveDown, MoveLeft, MoveRight, MoveUp},
    event::{Event, KeyCode, KeyEventKind, read},
};
use std::{
    fs::File, io::{self, Write, stdout}, process::{exit}
};

enum Modes {
    Normal,
    Insert,
    //GoTo
}

struct App {
    mode: Modes,
}

fn main() -> io::Result<()> {
    //Poderia aceitar argumentos do user de qual file usar


    let mut stdout = stdout();
    let mut app = App { mode: Modes::Normal };
    loop {
        match app.mode {
            Modes::Normal => match read()? {
                Event::Key(key_event) => {
                    if key_event.kind == KeyEventKind::Press {
                        match key_event.code {
                            KeyCode::Char('q') => exit(0),
                            KeyCode::Char('h') => { stdout.queue(MoveLeft(1))?;     stdout.flush()?; }
                            KeyCode::Char('j') => { stdout.queue(MoveDown(1))?;     stdout.flush()?; }
                            KeyCode::Char('k') => { stdout.queue(MoveUp(1))?;       stdout.flush()?; }
                            KeyCode::Char('l') => { stdout.queue(MoveRight(1))?;    stdout.flush()?; }
                            KeyCode::Char('i') => { app.mode = Modes::Insert;               stdout.flush()?; }
                            KeyCode::Char('o') => { print!("\n"); app.mode = Modes::Insert; stdout.flush()?; }
                            KeyCode::Char('g') => { }
                            _                  => {}
                        }
                    }
                }
                _ => {}
            },
            Modes::Insert => match read()? {
                Event::Key(key_event) => {
                    if key_event.kind == KeyEventKind::Press {
                        match key_event.code {
                            KeyCode::Esc       => { app.mode = Modes::Normal; stdout.flush()?; },
                            KeyCode::Char(' ') => { 
                                print!(" "); stdout.flush()?; 
                            },
                            KeyCode::Backspace => { 
                                stdout.queue(MoveLeft(1))?;
                                print!(" ");
                                stdout.queue(MoveLeft(1))?;
                                stdout.flush()?;
                            },
                            KeyCode::Enter => {  
                                print!("\n"); stdout.flush()?; 
                            }
                            _ => {
                                let char = key_event.code;
                                let mut file = File::open("./teste.txt")?;
                                file.write_all(&char.to_string().as_bytes())?;
                                //Pegar um arquivo e colocar o dado nele
                                print!("{}", key_event.code); stdout.flush()?; 
                            }
                        }
                    }
                }
                _ => {}
            },
            // Modes::GoTo => {

            // }
        }
    }
}
