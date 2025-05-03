use super::Uvec;

impl Uvec {
    pub fn shl(&mut self, n: usize) {
        let shift = n % 64;
        let mut result: Vec<u64> = vec![0; n >> 6];
        let mut val: u128;
        let mut carry: u128 = 0;
        if shift != 0 {
            for &unit in self.unit.iter() {
                val    = (unit as u128) << shift | carry;
                result.push(val as u64);
                carry  = val >> 64
            }
            if carry != 0 {result.push(carry as u64)}
        } else {
            for &unit in self.unit.iter() {
                result.push(unit);
            }
        }
        self.unit = result;
    }
}