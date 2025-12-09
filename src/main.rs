use crossterm::{
    QueueableCommand,
    cursor::{MoveDown, MoveLeft, MoveRight, MoveUp},
    event::{Event, KeyCode, KeyEvent, KeyEventKind, read},
};
use std::{
    fs::OpenOptions,
    io::{self, Stdout, Write, stdout},
    process::exit,
};

static FILE_PATH: &str = "./teste.txt";

struct Text {
    value: String
}

impl Text {
    pub fn new() -> Self{
        Self{
            value: String::new()
        }
    }

    pub fn concat(&mut self, content: String){
        self.value = self.value.clone() + &content;
    } 
}

enum Modes {
    Normal,
    Insert,
    //GoTo
}

struct App {
    mode: Modes,
}

fn handle_normal_input(key_event: KeyEvent, app: &mut App, text: &Text, stdout:&mut &Stdout) -> Result<String,()>{
    match key_event.code {
                            KeyCode::Char('q') => {
                                //lidar com salvar no arquivo
                                let mut file = OpenOptions::new()
                                    .append(true)
                                    .open(FILE_PATH)
                                    .expect("ERROR OPEN FILE");
                                    file.write(text.value.as_bytes())
                                    .expect("ERROR SAVING");
                                // _ = fs::write(FILE_PATH, TEXT_TO_BE_SAVED.as_bytes());

                                exit(0);
                            }
                            KeyCode::Char('h') => {
                                stdout.queue(MoveLeft(1)).unwrap();
                                stdout.flush().unwrap();
                                Ok(text.value.clone())
                            }
                            KeyCode::Char('j') => {
                                stdout.queue(MoveDown(1)).unwrap();
                                stdout.flush().unwrap();
                                Ok(text.value.clone())
                            }
                            KeyCode::Char('k') => {
                                stdout.queue(MoveUp(1)).unwrap();
                                stdout.flush().unwrap();
                                Ok(text.value.clone())
                            }
                            KeyCode::Char('l') => {
                                stdout.queue(MoveRight(1)).unwrap();
                                stdout.flush().unwrap();
                                Ok(text.value.clone())
                            }
                            KeyCode::Char('i') => {
                                app.mode = Modes::Insert;
                                stdout.flush().unwrap();
                                Ok(text.value.clone())
                            }
                            KeyCode::Char('o') => {
                                print!("\n");
                                app.mode = Modes::Insert;
                                stdout.flush().unwrap();
                                Ok(text.value.clone())
                            }
                            _ => {Ok(text.value.clone())}
                        }

}

fn handle_insert_input(key_event: KeyEvent, app: &mut App, text: &mut Text, stdout:&mut &Stdout){
    if key_event.kind == KeyEventKind::Press {
                        match key_event.code {
                            KeyCode::Esc => {
                                app.mode = Modes::Normal;
                                stdout.flush().unwrap();
                            }
                            KeyCode::Char(' ') => {
                                text.concat(" ".to_string());
                                print!(" ");
                                stdout.flush().unwrap();
                            }
                            KeyCode::Backspace => {
                                stdout.queue(MoveLeft(1)).unwrap();
                                print!(" ");
                                stdout.queue(MoveLeft(1)).unwrap();
                                stdout.flush().unwrap();
                            }
                            KeyCode::Enter => {
                                text.concat("\n".to_string());
                                print!("\n");
                                stdout.flush().unwrap();
                            }
                            _ => {
                                text.concat(key_event.code.to_string());
                                print!("{}", key_event.code);
                                stdout.flush().unwrap();
                            }
                        }
                    }
}

fn main() -> io::Result<()> {
    //Poderia aceitar argumentos do user de qual file usar
    let mut text = Text::new();

    let stdout = stdout();
    let mut app = App {
        mode: Modes::Normal,
    };
    loop {
        match app.mode {
            Modes::Normal => match read()? {
                Event::Key(key_event) => {
                    if key_event.kind == KeyEventKind::Press {
                        text.value = handle_normal_input(key_event, &mut app, &text, &mut &stdout).unwrap();
                    }
                }
                _ => {}
            },
            Modes::Insert => match read()? {
                Event::Key(key_event) => {
                    handle_insert_input(key_event, &mut app, &mut text, &mut &stdout);
                }
                _ => {}
            },
        }
    }
}