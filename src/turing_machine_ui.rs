use crate::turing_machine::TuringMachine;

impl TuringMachine {
    const DISPLAY_WIDTH: u8 = 75;

    fn centered_text(&self, text: &String, focus: Option<u8>) {
        let hw = Self::DISPLAY_WIDTH / 2;

        let focus = match focus {
            Some(f) => f,
            None => text.len() as u8 / 2,
        };
        let leading_spaces = hw - focus;
        println!("{}{}", " ".repeat(leading_spaces as usize), text);
    }

    fn show_tape(&mut self) {
        let hw = Self::DISPLAY_WIDTH / 2;

        for loc in self.head - (hw as isize) .. self.head + (hw as isize) {
            let (&mut ref mut vec, index) = self.vec_and_index(&loc);
            let mut symbol = 0;
            if index < vec.len() { symbol = vec[index]; }
            match symbol {
                0 => print!("◻"),
                1 => print!("◼"),
                _ => unreachable!(),
            };
        }
        println!();
        
    }

    pub fn show(&mut self) {
        self.show_tape();
        self.centered_text(&String::from("^"), None);
        let symbol = self.get_symbol();
        self.centered_text(&format!("{} o {}", self.state, symbol), None);
        println!();
    }
}