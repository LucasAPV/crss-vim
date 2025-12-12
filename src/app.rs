use crossterm::{
   QueueableCommand,
   cursor::{MoveDown, MoveLeft, MoveRight, MoveTo, MoveUp},
   event::{KeyCode, KeyEvent, KeyEventKind}, terminal::{Clear, ClearType},
};
use std::{
   fs::{self, OpenOptions},
   io::{Stdout, Write},
   process::exit,
};

pub static FILE_PATH: &str = "./teste.txt";

pub struct Text {
   pub lines: Vec<String>,
}

impl Text {
   pub fn new() -> Self {
      Self {
         lines: vec![String::new()],
      }
   }

   pub fn insert_char(&mut self, row: usize, col: usize, ch: char){
      while self.lines.len() <=row {
         self.lines.push(String::new());
      }

      let line = &mut self.lines[row];
      while line.len() < col {
         line.push(' ');
      }
      line.insert(col, ch);
   }

   pub fn delete_char(&mut self, row: usize, col: usize) -> bool{
      if row >= self.lines.len() {
         return false;
      }

      if col > 0 && col <self.lines[row].len(){
         self.lines[row].remove(col - 1);
         return true;
      }

      false
   }

   pub fn insert_new_line(&mut self, row: usize, col: usize){
      if row >= self.lines.len() {
         self.lines.push(String::new());
         return;
      }

      let current_line = &self.lines[row];
      let remaining = if col < current_line.len() {
         current_line[col..].to_string()
      }else {
         String::new()
      };

      self.lines[row].truncate(col);
      self.lines.insert(row + 1, remaining);
   }

   pub fn retrive(&mut self, file_path: String) -> Self {
      match fs::read_to_string(&file_path) {
            Ok(content) => {
               let v: Vec<String> = if content.is_empty() {
                  vec![String::new()]
               } else {
                  content.lines().map(|s| s.to_string()).collect()
               };
               Self { lines: v }
            }
            Err(_) => Self::new(),
      }
   }

   pub fn save(&self, file_path: String) -> std::io::Result<()>{
      let mut file = OpenOptions::new()
            .write(true)
            .truncate(true)
            .create(true)
            .open(file_path)?;

      for (i, line) in self.lines.iter().enumerate() {
            file.write_all(line.as_bytes())?;
            if i < self.lines.len() - 1 {
               file.write_all(b"\n")?;
            }
      }

      Ok(())
   }
}


pub enum Modes {
   Normal,
   Insert,
}

pub struct App {
   pub mode   : Modes,
   pub row    : usize,
   pub col    : usize,
}

impl App {
   pub fn handle_normal_input(
      key_event: KeyEvent,
      app      : &mut App,
      text     : &Text,
      stdout   : &mut &Stdout,
   ) -> Result<(), ()> {
      match key_event.code {
         KeyCode::Char('q') => {
               //lidar com salvar no arquivo
               text.save(String::from(FILE_PATH)).expect("ERROR SAVING");
               stdout.queue(Clear(ClearType::All)).unwrap();
               stdout.flush().unwrap();
               exit(0);
         }
         KeyCode::Char('h') => {
            if app.col > 0 {
               app.col -= 1;
               stdout.queue(MoveLeft(1)).unwrap();
               stdout.flush().unwrap();   
            }
            Ok(())
         }
         KeyCode::Char('j') => {
            if app.row < text.lines.len() - 1 {
                  app.row += 1;
                  if app.col > text.lines[app.row].len() {
                        app.col = text.lines[app.row].len();
                     }

                  stdout.queue(MoveTo(app.col as u16, app.row as u16)).unwrap();
                  stdout.flush().unwrap();
               }
               Ok(())
         }
         KeyCode::Char('k') => {
               if app.row > 0 {
                  app.row -= 1;
                  stdout.queue(MoveUp(1)).unwrap();
                  
                  // Ajusta col se a nova linha for mais curta
                  if app.col > text.lines[app.row].len() {
                     app.col = text.lines[app.row].len();
                  }
                  
                  stdout.queue(MoveTo(app.col as u16, app.row as u16)).unwrap();
                  stdout.flush().unwrap();
               }
               Ok(())
         }
         KeyCode::Char('l') => {
               if app.col < text.lines[app.row].len() {
                  app.col += 1;
                  stdout.queue(MoveRight(1)).unwrap();
                  stdout.flush().unwrap();
               }
               Ok(())
         }
         KeyCode::Char('i') => {
               app.mode = Modes::Insert;
               stdout.flush().unwrap();
               Ok(())
         }
         KeyCode::Char('o') => {
               app.col = text.lines[app.row].len();
               app.row += 1;
               
               stdout.queue(MoveTo(0, app.row as u16)).unwrap();
               print!("\n");
               stdout.flush().unwrap();
               
               app.mode = Modes::Insert;
               Ok(())
         }
         _ => {Ok(())},
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
               KeyCode::Char(ch) => {
                  text.insert_char(app.row, app.col, ch);
                  app.col += 1;
                  print!("{}", ch);
                  stdout.flush().unwrap();
               }
               KeyCode::Backspace => {
                  if text.delete_char(app.row, app.col) {
                        app.col -= 1;
                        stdout.queue(MoveLeft(1)).unwrap();
                        print!(" ");
                        stdout.queue(MoveLeft(1)).unwrap();
                        stdout.flush().unwrap();
                  }
               }
               KeyCode::Enter => {
                  text.insert_new_line(app.row, app.col);
                  app.row += 1;
                  app.col = 0;
                  print!("\n");
                  stdout.flush().unwrap();
               }
               KeyCode::Down => {
                  if app.row < text.lines.len() - 1 {
                        app.row += 1;
                        stdout.queue(MoveDown(1)).unwrap();
                        stdout.flush().unwrap();
                  }
               },
               KeyCode::Up => {
                  if app.row > 0 {
                        app.row -= 1;
                        stdout.queue(MoveUp(1)).unwrap();
                        stdout.flush().unwrap();
                  }
               },
               KeyCode::Left => {
                  if app.col > 0 {
                        app.col -= 1;
                        stdout.queue(MoveLeft(1)).unwrap();
                        stdout.flush().unwrap();
                  }
               },
               KeyCode::Right => {
                  if app.col < text.lines[app.row].len() {
                        app.col += 1;
                        stdout.queue(MoveRight(1)).unwrap();
                        stdout.flush().unwrap();
                  }
               }
               KeyCode::Tab => {
                  for _ in 0..4 {
                        text.insert_char(app.row, app.col, ' ');
                        app.col += 1;
                  }
                  print!("    ");
                  stdout.flush().unwrap();
               }
               _ => { }
         }
      }
   }
}
