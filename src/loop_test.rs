use crate::busy_beavers::BusyBeavers;
use crate::turing_machine::TuringMachine;

pub struct LoopStats {
    periodicity: usize,
    pc_lifetime: usize,
    pc_coverage: usize,
    pc_footprint: usize,
}

impl BusyBeavers {
    pub fn loop_test(&self, beaver: &mut TuringMachine) -> Option<LoopStats> {
        let periodicity = match self.get_periodicity(beaver) {
            Some(value) => value,
            None => return None,
        };

        let pc_lifetime = match (0..beaver.history.len()).rev().skip(periodicity).find(
            |&i| beaver.history[i] != beaver.history[i + periodicity]
        ){
            Some(i) => i + 1,
            None => 0,
        };

        let mut beaver_copy = TuringMachine::new(beaver.t_fn.clone());
        beaver_copy.run(pc_lifetime);

        Some(
            LoopStats {
                periodicity: periodicity,
                pc_lifetime: pc_lifetime,
                pc_coverage: beaver_copy.tape.len(),
                pc_footprint: beaver_copy.tape.footprint(),
            }
        )
    }

    fn get_periodicity(&self, beaver: &mut TuringMachine) -> Option<usize> {
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