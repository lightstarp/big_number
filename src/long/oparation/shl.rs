use super::Uvec;

impl Uvec {
    pub fn shl(&mut self, n: usize) {

        let mut result: Vec<u64> = vec![0; n >> 6];
        let mut val: u128;
        let mut carry: u128 = 0;
        let shift = n % 64;
        for &unit in self.unit.iter() {
            val    = (unit as u128) << shift | carry;
            result.push(val as u64);
            carry  = val >> 64
        }
        self.unit = result;
    }
}