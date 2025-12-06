use crossterm::{
    QueueableCommand,
    cursor::{MoveDown, MoveLeft, MoveRight, MoveUp},
    event::{Event, KeyCode, KeyEventKind, read},
};
use std::{
    fs::OpenOptions,
    io::{self, Write, stdout},
    process::exit,
};

static FILE_PATH: &str = "./teste.txt";
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
    let mut text_to_be_saved: String = String::new();

    let mut stdout = stdout();
    let mut app = App {
        mode: Modes::Normal,
    };
    loop {
        match app.mode {
            Modes::Normal => match read()? {
                Event::Key(key_event) => {
                    if key_event.kind == KeyEventKind::Press {
                        match key_event.code {
                            KeyCode::Char('q') => {
                                //lidar com salvar no arquivo
                                let mut file = OpenOptions::new()
                                    .append(true)
                                    .open(FILE_PATH)
                                    .expect("ERROR OPEN FILE");
                                file.write(text_to_be_saved.as_bytes())
                                    .expect("ERROR SAVING");
                                // _ = fs::write(FILE_PATH, text_to_be_saved.as_bytes());

                                exit(0);
                            }
                            KeyCode::Char('h') => {
                                stdout.queue(MoveLeft(1))?;
                                stdout.flush()?;
                            }
                            KeyCode::Char('j') => {
                                stdout.queue(MoveDown(1))?;
                                stdout.flush()?;
                            }
                            KeyCode::Char('k') => {
                                stdout.queue(MoveUp(1))?;
                                stdout.flush()?;
                            }
                            KeyCode::Char('l') => {
                                stdout.queue(MoveRight(1))?;
                                stdout.flush()?;
                            }
                            KeyCode::Char('i') => {
                                app.mode = Modes::Insert;
                                stdout.flush()?;
                            }
                            KeyCode::Char('o') => {
                                print!("\n");
                                app.mode = Modes::Insert;
                                stdout.flush()?;
                            }
                            _ => {}
                        }
                    }
                }
                _ => {}
            },
            Modes::Insert => match read()? {
                Event::Key(key_event) => {
                    if key_event.kind == KeyEventKind::Press {
                        match key_event.code {
                            KeyCode::Esc => {
                                app.mode = Modes::Normal;
                                stdout.flush()?;
                            }
                            KeyCode::Char(' ') => {
                                text_to_be_saved.push(' ');
                                print!(" ");
                                stdout.flush()?;
                            }
                            KeyCode::Backspace => {
                                stdout.queue(MoveLeft(1))?;
                                print!(" ");
                                stdout.queue(MoveLeft(1))?;
                                stdout.flush()?;
                            }
                            KeyCode::Enter => {
                                text_to_be_saved.push('\n');
                                print!("\n");
                                stdout.flush()?;
                            }
                            _ => {
                                text_to_be_saved.push(key_event.code.as_char().expect("ERROR"));
                                print!("{}", key_event.code);
                                stdout.flush()?;
                            }
                        }
                    }
                }
                _ => {}
            },
        }
    }
}