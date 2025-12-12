use crossterm::{
    QueueableCommand, cursor::MoveTo, event::{Event, KeyEventKind, read}, terminal::{Clear, ClearType}
};
use std::io::{self, Write, stdout};

//pub mod App;
#[allow(non_snake_case)]
pub mod App;
use App::{
    App as application, 
    Text, 
    FILE_PATH, 
    Modes
};

fn main() -> io::Result<()> {
    let mut text = Text::new();
    let s = text.retrive(FILE_PATH.to_string());
    let mut stdout = stdout();

    let mut app = application {
        mode: Modes::Normal,
        row: 0,
        col: 0,
    };

    stdout.queue(Clear(ClearType::All))?;
    stdout.flush()?;

    stdout.queue(MoveTo(0u16,0u16)).unwrap();
    stdout.flush()?;

    if !s.lines.is_empty() {
        for line in s.lines {
            if line != "".to_string(){
                print!("{}", line);
            }
            stdout.flush()?;
        }
    }
    stdout.queue(MoveTo(0u16,0u16)).unwrap();
    stdout.flush()?;
    loop {
        match app.mode {
            Modes::Normal => match read()? {
                Event::Key(key_event) => {
                    if key_event.kind == KeyEventKind::Press {
                            application::
                                handle_normal_input
                                    (
                                        key_event, &mut app, 
                                        &text, &mut &stdout
                                    )
                                .unwrap();
                    }
                }
                _ => {}
            },
            Modes::Insert => match read()? {
                Event::Key(key_event) => {
                    application::
                        handle_insert_input
                            (
                                key_event, &mut app, 
                                &mut text, &mut &stdout
                            );
                }
                _ => {}
            },
        }
    }
}