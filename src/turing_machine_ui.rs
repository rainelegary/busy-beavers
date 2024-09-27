use crate::turing_machine::TuringMachine;

impl TuringMachine {
    const DISPLAY_WIDTH: u8 = 75;

    fn centered_text(&self) {
        let w = Self::DISPLAY_WIDTH;
        let hw = w / 2;
        
    }

    fn show_tape(&mut self) {
        let w = Self::DISPLAY_WIDTH;
        let hw = w / 2;

        for i in &self.head - hw as isize .. &self.head + hw as isize {
            let (&mut ref mut vec, index) = self.vec_and_index(&i);
            print!("{}", vec[index]);
        }
    }

    pub fn show(&mut self) {
        println!("");


        println!("{:?}", &self.tape.0);
        println!("{:?}", &self.tape.1);
        println!("state: {}", &self.state);
        println!("symbol: {}", &self.get_symbol());
        println!("");
    }
}