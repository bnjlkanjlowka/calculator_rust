use std::io::{stdin, stdout, Write};
use std::{env, process};
use termion::clear;
use termion::cursor::{self, DetectCursorPos};
use termion::event::Key;
use termion::input::TermRead;
use termion::raw::IntoRawMode;

fn is_debug() -> bool {
    env::var("DEBUG").is_ok()
}

pub fn input(input: &mut String) {
    let stdin = stdin();
    let mut stdout = stdout().into_raw_mode().unwrap();

    let x: u16;
    let y: u16;

    if !is_debug() {
        (x, y) = stdout.cursor_pos().unwrap();
    } else {
        x = 1;
        y = 1;
    }

    let mut cursor_position: usize = x as usize;
    cursor::Goto(cursor_position as u16, y);
    stdout.flush().unwrap();

    for key in stdin.keys() {
        match key.unwrap() {
            Key::Char('\n') => break,
            Key::Char(chr) => match chr {
                '0'..='9' => {
                    input.insert(cursor_position - 1, chr);
                    cursor_position += 1;
                }
                '+' | '-' | '*' | '/' | '^' => {
                    input.insert(cursor_position - 1, chr);
                    cursor_position += 1;
                }
                '(' | ')' => {
                    input.insert(cursor_position - 1, chr);
                    cursor_position += 1;
                }
                'q' => process::exit(0),
                _ => {}
            },
            Key::Ctrl('c') => process::exit(0),
            Key::Left => {
                if cursor_position > 1 {
                    cursor_position -= 1;
                }
            }
            Key::Right => {
                if cursor_position <= input.len() {
                    cursor_position += 1;
                }
            }
            Key::Backspace => {
                if cursor_position > 1 {
                    input.remove(cursor_position - 2);
                    cursor_position -= 1;
                }
            }
            _ => {}
        }
        write!(
            stdout,
            "\r{}{}{}",
            clear::CurrentLine,
            input,
            cursor::Goto(cursor_position as u16, y)
        )
        .unwrap();
        stdout.flush().unwrap();
    }
    write!(stdout, "\r\n").unwrap();
    stdout.flush().unwrap();
}
