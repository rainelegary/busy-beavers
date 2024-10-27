use crate::busy_beavers::BusyBeavers;

impl BusyBeavers {
    pub fn loop_test(&mut self, beaver_id: usize) -> Option<usize> {
        let beaver = &mut self.beavers[beaver_id];
        let last_index = beaver.history.len() - 1;
        for periodicity in 1_usize..=beaver.history.len() / 2 {
            // test for state cycle
            if (0..periodicity).any(
                |offset| beaver.history[last_index - offset] 
                != beaver.history[last_index - periodicity - offset]
            ) { continue }

            // test for zero net delta (dormant)
            if (0..periodicity).map(
                |offset| beaver.t_fn[&beaver.history[last_index - offset]].delta
            ).sum::<i8>() == 0 {
                return Some(periodicity);
            }

            // test for new coverage (propagating)
            let tape_length = beaver.tape.len();
            beaver.run(periodicity);
            if beaver.tape.len() > tape_length {
                return Some(periodicity);
            }
        }
        None
    }
}