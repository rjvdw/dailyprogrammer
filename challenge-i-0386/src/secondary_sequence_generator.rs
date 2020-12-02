use crate::base_sequence_generator::BaseSequenceGenerator;

pub struct SecondarySequenceGenerator {
    next_value: u64,
    base_sequence: BaseSequenceGenerator,
    upper_bound: u64,
}

impl SecondarySequenceGenerator {
    pub fn new(upper_bound: u64) -> SecondarySequenceGenerator {
        SecondarySequenceGenerator { next_value: 1, base_sequence: BaseSequenceGenerator::new(), upper_bound }
    }
}

impl Iterator for SecondarySequenceGenerator {
    type Item = u64;

    fn next(&mut self) -> Option<Self::Item> {
        if self.next_value <= self.upper_bound {
            let i = self.next_value;
            self.next_value += self.base_sequence.next().unwrap();
            Some(i)
        } else {
            None
        }
    }
}