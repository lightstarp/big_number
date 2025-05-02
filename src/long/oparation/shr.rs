use super::Uvec;

impl Uvec {
    pub fn shr(&mut self, n: u64) {
        let mut val: u128;
        let mut carry: u128 = 0;
        let mut result: Vec<u64> = vec![];
        for &unit in self.unit.iter() {
            val    = (unit as u128) << n | carry;
            result.push(val as u64);
            carry  = val >> 64
        }
        self.unit = result;
    }
}