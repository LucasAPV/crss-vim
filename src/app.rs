use crossterm::{
   QueueableCommand,
   cursor::{MoveDown, MoveLeft, MoveRight, MoveUp},
   event::{KeyCode, KeyEvent, KeyEventKind}, terminal::{Clear, ClearType},
};
use std::{
   fs::{self, OpenOptions},
   io::{Stdout, Write},
   process::exit,
};

pub static FILE_PATH: &str = "./teste.txt";

pub struct Text {
   pub value: String,
}

impl Text {
   pub fn new() -> Self {
      Self {
         value: String::new(),
      }
   }

   pub fn concat(&mut self, content: String) {
      self.value = self.value.clone() + &content;
   }

   pub fn retrive(&mut self, file_path: String) -> Self {
      Self {
         value: fs::read_to_string(file_path).unwrap(),
      }
   }
}

pub enum Modes {
   Normal,
   Insert,
   //GoTo
}

pub struct App {
   pub mode: Modes,
   pub line: u16,
   pub col: u16 
}

impl App {
   pub fn handle_normal_input(
      key_event: KeyEvent,
      app: &mut App,
      text: &Text,
      stdout: &mut &Stdout,
   ) -> Result<String, ()> {
      match key_event.code {
         KeyCode::Char('q') => {
               //lidar com salvar no arquivo
               let mut file = OpenOptions::new()
                  .append(true)
                  .open(FILE_PATH)
                  .expect("ERROR OPEN FILE");
               file.write(text.value.as_bytes()).expect("ERROR SAVING");

               stdout.queue(Clear(ClearType::All)).unwrap();
               stdout.flush().unwrap();

               exit(0);
               #[allow(unreachable_code)]
               Ok("o".to_string())
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
         _ => Ok(text.value.clone()),
      }
   }

   pub fn handle_insert_input(
      key_event: KeyEvent,
      app: &mut App,
      text: &mut Text,
      stdout: &mut &Stdout,
   ) {
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
                  text.value.pop();
               }
               KeyCode::Enter => {
                  text.concat("\n".to_string());
                  print!("\n");
                  stdout.flush().unwrap();
               }
               KeyCode::Down => {
                  stdout.queue(MoveDown(1)).unwrap();
                  stdout.flush().unwrap();
               },
               KeyCode::Up => {
                  stdout.queue(MoveUp(1)).unwrap();
                  stdout.flush().unwrap();
               },
               KeyCode::Left => {
                  stdout.queue(MoveLeft(1)).unwrap();
                  stdout.flush().unwrap();
               },
               KeyCode::Right => {
                  stdout.queue(MoveRight(1)).unwrap();
                  stdout.flush().unwrap();
               }
               KeyCode::Tab => {
                  print!("    ");
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
}
