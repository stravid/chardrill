use rand::seq::SliceRandom;
use rand::thread_rng;
use std::io;
use std::io::{stdout, Stdout, Write};
use std::time::Instant;
use termion::event::Key;
use termion::input::TermRead;
use termion::raw::{IntoRawMode, RawTerminal};

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

    let _stdout: RawTerminal<Stdout> = stdout().into_raw_mode().unwrap();

    loop {
        print_characters(&sequence, index);
        let key = get_next_key();

        match key {
            Key::Esc => {
                break;
            }
            key => {
                actual_presses += 1;

                if key == termion::event::Key::Char(sequence[index]) {
                    index += 1;
                    target_presses += 1;
                }

                if index == sequence.len() {
                    break;
                }

                print_characters(&sequence, index);
            }
        }
    }

    print_result(&start, target_presses, actual_presses);
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
            print!(
                "{}{}{}{}{}",
                termion::color::Fg(termion::color::Black),
                termion::color::Bg(termion::color::White),
                character,
                termion::color::Fg(termion::color::Reset),
                termion::color::Bg(termion::color::Reset),
            );
        } else {
            print!(
                "{}",
                character
            );
        }
    }

    flush();
}

fn clear() {
    print!(
        "{}{}{}",
        termion::clear::All,
        termion::cursor::Hide,
        termion::cursor::Goto(1, 1)
    );
    flush();
}

fn flush() {
    match io::stdout().flush() {
        Ok(_) => {}
        Err(error) => println!("Terminal Error: {}", error),
    }
}

fn get_next_key() -> Key {
    match io::stdin().lock().keys().next() {
        None => panic!("Terminal Error: No key pressed"),
        Some(key) => match key {
            Ok(key) => key,
            Err(error) => panic!("Terminal Error: {}", error),
        },
    }
}
