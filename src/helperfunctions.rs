use crossterm::{cursor, ExecutableCommand};
use pancurses::{endwin, initscr, noecho, Input, Window};
use std::io::{stdin, stdout, Write};
use std::thread::sleep;
use std::time::Duration;
use textwrap::{fill, wrap, Options};

pub fn clear() {
    print!("{esc}c", esc = 27 as char);
}

pub fn printout(text: &str, delay: u64, pause: u64, pauseat: &[char], wait: bool) {
    for c in text.chars() {
        print!("{c}");
        stdout().flush().unwrap();
        if pauseat.contains(&c) {
            sleep(Duration::from_millis(pause))
        } else {
            sleep(Duration::from_millis(delay))
        };
    }
    if wait == true {
        wait_input()
    }
}

pub fn cprintout(
    text: &str,
    delay: u64,
    pause: u64,
    pauseat: &[char],
    wait: bool,
    window: &Window,
    wrap_point: usize,
) {
    for c in fill(text, wrap_point).chars() {
        window.addch(c);
        window.refresh();
        if pauseat.contains(&c) {
            sleep(Duration::from_millis(pause))
        } else {
            sleep(Duration::from_millis(delay))
        };
    }
    if wait == true {
        cwait_input(&window)
    }
}

pub fn cwait_input(window: &Window) {
    stdout()
        .execute(cursor::SetCursorStyle::BlinkingBlock)
        .unwrap();
    loop {
        match window.getch() {
            None => (),
            _ => break,
        }
    }
    stdout()
        .execute(cursor::SetCursorStyle::DefaultUserShape)
        .unwrap();
}

pub fn wait_input() {
    let mut input = String::new();
    stdout()
        .execute(cursor::SetCursorStyle::BlinkingUnderScore)
        .unwrap();
    stdin().read_line(&mut input).unwrap();
    stdout()
        .execute(cursor::SetCursorStyle::DefaultUserShape)
        .unwrap();
}
