use super::Uvec;


impl Uvec {
    pub fn push(&mut self, a: u64) {
        self.unit.push(a);
    }
}