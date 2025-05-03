use super::Uvec;

impl Uvec {
    pub fn shr(&mut self, n: usize) {
        let shift = n % 64;
        let unit_shift = n >> 6;
        let mut result: Vec<u64> = vec![0; self.unit.len() - unit_shift];
        let mut val: u128;
        let mut carry: u128 = 0;
        let mut i = self.unit.len();
        if shift != 0 {
            for &unit in self.unit.iter().rev() {
                i -= 1;
                if i < unit_shift {break;};
                val    = (unit as u128) << (64 - shift) | carry;
                result[i - unit_shift] = (val >> 64) as u64; /*valの上位64bitをresultに入れる*/
                carry  = val  << 64                          /*valの下位64bitをresultに入れる*/

            }
            //if carry != 0 {result.push(carry as u64)} /*右にシフトするとき溢れた数は捨てる*/
        } else {
            for &unit in self.unit.iter().rev() {
                i -= 1;
                if i < unit_shift {break;};
                result[i - unit_shift] = unit;
            }
        }
        self.unit = result;
    }
}