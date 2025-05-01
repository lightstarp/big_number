use super::Uvec;

impl Uvec {
    pub fn pop(&mut self) -> u64 {
        self.unit.pop().expect("REASON")
    }
}