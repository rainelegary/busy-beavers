use colored::Colorize;
use std::fmt::Display;
use regex::Regex;

use crate::turing_machine::TuringMachine;

impl TuringMachine {
    const DISPLAY_WIDTH: u8 = 101;

    pub fn show_tape_and_state(&mut self) {
        let hw = Self::DISPLAY_WIDTH / 2;
    
        Self::centered_text("▼".red(), None);
        for loc in self.head - (hw as isize) .. self.head + (hw as isize) {
            let (&mut ref mut vec, index) = self.vec_and_index(&loc);
            let mut symbol = 0;
            if index < vec.len() { symbol = vec[index]; }
            match symbol {
                0 => print!("{}", "▉".blue()),  // ◻ ◇ ▽ △ ○ █ ▉
                1 => print!("{}", "▉".green()), // ◼ ◆ ▼ ▲ ● █ ▉
                2 => print!("{}", "▉".yellow()),
                3 => print!("{}", "▉".red()),
                4 => print!("{}", "▉".cyan()),
                _ => panic!(),
            };
        }
        println!();
        Self::centered_text("▲".red(), None);
        println!();

        let current = &self.history[self.history.len() - 1];
        Self::centered_text(format!("{} ◇ {}", current.state, current.symbol), None);
        println!();
    }
    
    fn centered_text<T: Display>(text: T, focus: Option<u8>) {
        let hw = Self::DISPLAY_WIDTH / 2;
        
        let raw_text = text.to_string();
        let visible_text_length = Self::strip_ansi_codes(&raw_text).chars().count();
        let focus = match focus {
            Some(f) => f,
            None => visible_text_length as u8 / 2,
        };
        
        let leading_spaces = hw - focus;
        println!("{}{}", " ".repeat(leading_spaces as usize), text);
    }
    
    fn strip_ansi_codes(text: &str) -> String {
        let re = Regex::new(r"\x1b\[[0-9;]*m").unwrap();
        re.replace_all(text, "").to_string()
    }
}