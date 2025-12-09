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
        line: 0,
        col: 0
    };

    stdout.queue(Clear(ClearType::All)).unwrap();
    stdout.flush()?;
    if s.value != "".to_string() {
        println!("{}", s.value);
        stdout.flush()?;
    }
    stdout.queue(MoveTo(0u16,0u16)).unwrap();
    stdout.flush()?;
    loop {
        match app.mode {
            Modes::Normal => match read()? {
                Event::Key(key_event) => {
                    if key_event.kind == KeyEventKind::Press {
                        text.value =
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