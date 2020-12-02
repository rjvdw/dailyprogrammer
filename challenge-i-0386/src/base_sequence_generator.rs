pub struct BaseSequenceGenerator {
    s1_value: u64,
    s2_value: u64,
    value_toggle: bool,
}

impl BaseSequenceGenerator {
    pub fn new() -> BaseSequenceGenerator {
        BaseSequenceGenerator { s1_value: 0, s2_value: 1, value_toggle: false }
    }
}

impl Iterator for BaseSequenceGenerator {
    type Item = u64;

    fn next(&mut self) -> Option<Self::Item> {
        self.value_toggle = !self.value_toggle;
        if self.value_toggle {
            self.s1_value += 1;
            Some(self.s1_value)
        } else {
            self.s2_value += 2;
            Some(self.s2_value)
        }
    }
}