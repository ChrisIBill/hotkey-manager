extern crate termion;

use std::io::{stdin, stdout, Write};
use termion::event::{Event, Key, MouseEvent};
use termion::input::{MouseTerminal, TermRead};
use termion::raw::IntoRawMode;
// use rusqlite::{Connection, Result};

// #[derive(Debug)]
// struct Hotkey {
//     id: i32,
//     keys: Vec<String>,
//     description: String,
// }
//
// #[derive(Debug)]
// struct KeyboardKey {
//     id: i32,
//     name: String,
//     children: Vec<Key>,
//     description: String,
// }
fn main() {
    let stdin = stdin();
    let mut stdout = MouseTerminal::from(stdout().into_raw_mode().unwrap());

    write!(
        stdout,
        "{}{}q to exit. Click, click, click!",
        termion::clear::All,
        termion::cursor::Goto(1, 1)
    )
    .unwrap();
    stdout.flush().unwrap();
    // let mut stdout = stdout().into_raw_mode().unwrap();
    // write!(stdout, r#"{}{}ctrl + q to exit, ctrl + h to print "Hello world!", alt + t to print "termion is cool""#, termion::cursor::Goto(1, 1), termion::clear::All)
    //         .unwrap();
    // stdout.flush().unwrap();

    for c in stdin.events() {
        write!(
            stdout,
            "{}{}",
            termion::cursor::Goto(1, 1),
            termion::clear::All
        )
        .unwrap();

        match c.unwrap() {
            Event::Key(Key::Char('q')) => break,
            Event::Key(Key::Char('h')) => println!("Hello, world!"),
            Event::Key(Key::Char('t')) => println!("termion is cool"),
            // Event::Mouse(_) => todo!(),
            Event::Unsupported(_) => todo!(),
            Event::Mouse(me) => match me {
                MouseEvent::Press(_, x, y) => {
                    println!("{}x", termion::cursor::Goto(x, y));
                }
                _ => (),
            },
            _ => {}
        }

        // stdout.flush().unwrap();
    }
}
