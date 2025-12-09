use crossterm::{
    event::{Event, KeyEventKind, read},
};
use std::{
    io::{self, Write, stdout},
};

mod app;
use app::*;
fn main() -> io::Result<()> {
    let mut text = Text::new();
    let s = text.retrive(FILE_PATH.to_string());
    let mut stdout = stdout();
    let mut app = app::App {
        mode: Modes::Normal,
    };

    if s.value != "".to_string() {
        println!("{}", s.value);
        stdout.flush()?;
    }

    loop {
        match app.mode {
            Modes::Normal => match read()? {
                Event::Key(key_event) => {
                    if key_event.kind == KeyEventKind::Press {
                        text.value =
                            app::App::handle_normal_input(key_event, &mut app, &text, &mut &stdout)
                                .unwrap();
                    }
                }
                _ => {}
            },
            Modes::Insert => match read()? {
                Event::Key(key_event) => {
                    app::App::handle_insert_input(key_event, &mut app, &mut text, &mut &stdout);
                }
                _ => {}
            },
        }
    }
}