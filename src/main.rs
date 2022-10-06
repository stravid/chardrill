use rand::seq::SliceRandom;
use rand::thread_rng;
use std::io;
use std::io::{stdout, Stdout, Write};
use std::time::Instant;
use crossterm::cursor::{Hide, MoveTo};
use crossterm::event::{Event, KeyCode, read};
use crossterm::event::Event::Key;
use crossterm::{execute, style, terminal};
use crossterm::terminal::{Clear, ClearType};
use crossterm::style::{style, Color, Attribute, Stylize};

fn main() {
    let set = vec![
        'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r',
        's', 't', 'u', 'v', 'w', 'x', 'y', 'z', '0', '1', '2', '3', '4', '5', '6', '7', '8', '9',
        '`', '~', '!', '@', '#', '$', '%', '^', '&', '*', '(', ')', '-', '_', '=', '+', '\\', '|',
        '[', '{', ']', '}', ';', ':', '\'', '"', ',', '<', '.', '>', '/', '?',
    ];
    let mut index = 0;
    let start = Instant::now();
    let mut target_presses = 0;
    let mut actual_presses = 0;
    let mut sequence = [&set[..], &set[..], &set[..], &set[..], &set[..]].concat();

    sequence.shuffle(&mut thread_rng());

    let t = terminal::enable_raw_mode();
    match t {
        Ok(_) => {}
        Err(error) => {panic!("terminal error: {:?}", error)}
    }

    print_characters(&sequence, index);

    loop {
        let key = get_next_key();

        match key {
            KeyCode::Esc => {
                break;
            }
            KeyCode::Char(char) => {
                actual_presses += 1;

                if char == sequence[index] {
                    index += 1;
                    target_presses += 1;
                    print_characters(&sequence, index);
                }

                if index == sequence.len() {
                    break;
                }
            }
            _ => {}
        }
    }

    print_result(&start, target_presses, actual_presses);

    // Wait for a keypress before exiting the program
    get_next_key();
}

fn print_result(start: &Instant, target_presses: usize, actual_presses: usize) {
    let end = Instant::now();
    let duration = end.duration_since(*start);
    clear();
    println!("{} seconds\r", duration.as_secs());

    let percentage = match target_presses {
        0 => actual_presses as f32 * 100.0,
        _ => (actual_presses as f32 / target_presses as f32 - 1.0) * 100.0,
    };

    println!("{} % error rate", percentage);
}

fn print_characters(characters: &[char], highlight_index: usize) {
    clear();

    for (index, character) in characters.iter().enumerate() {
        if index == highlight_index {
            execute!(
                stdout(),
                style::SetForegroundColor(Color::Black),
                style::SetBackgroundColor(Color::White),
                style::Print(character),
                style::ResetColor,
            );
        } else {
            execute!(
                stdout(),
                style::SetForegroundColor(Color::White),
                style::SetBackgroundColor(Color::Black),
                style::Print(character),
                style::ResetColor,
            );
        }
    }
}

fn clear() {
    execute!(
        stdout(),
        Clear(ClearType::All),
        Hide,
        MoveTo(0, 0)
    );
}

fn get_next_key() -> KeyCode {
    loop {
        match read() {
                Ok(event) => match event {
                    Key(key) => { return key.code }
                    _ => {}
                },
                Err(error) => panic!("Terminal Error: {}", error),
        }
    }
}
