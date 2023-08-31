mod helperfunctions;

use crossterm::{
    execute,
    style::{Color, SetBackgroundColor, SetForegroundColor},
    terminal, ExecutableCommand,
};
use helperfunctions::*;
use inflector::Inflector;
use moon_calc::Moon;
use num2words::Num2Words;
use pancurses::{endwin, initscr, newwin, noecho, Input};
use std::io;
use std::io::{stdout, Write};
use std::thread::sleep;
use std::time::Duration;
use std::time::SystemTime;
use textwrap::{fill, wrap};

const DEFTEXTSPEED: u64 = 50;

fn main() {
    let mut window = initscr();
    window.resize(20, 30);
    noecho();
    let moon = Moon::new(SystemTime::now());
    cprintout(
        &format!(
            "{} days ago the moon was invented.  The old venerable girl has been through a lot at our hands. Names and mixed metaphors, mined for moon-milk and meaning.\n\nPerhaps we took her for granted, as we tend to do. Perhaps we only realized we did when the doctors told us it's terminal: the moon is rusting and there's nothing we can do about it.\n\nYou see,\n       the earth spins electrons, swirled out painfully into the burning void of the universe. Is it really our fault if there's side effects? If bystanders get caught in our glorious magnetism?",
            Num2Words::new(moon.julian_date)
                .to_words()
                .unwrap()
                .to_sentence_case()
        ),
        DEFTEXTSPEED,
        400, &['.', '?', '!'], true, &window, 30);
}
