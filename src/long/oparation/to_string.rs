use super::Uvec;

impl Uvec {
    pub fn to_string(&self) -> String {
        let mut val_list: Vec<u8> = vec![0];
        let mut carry: u8;
        for &unit in self.unit.iter().rev() {
            for i in 0..64 {
                carry = (unit >> 63 - i) as u8 & 0b00000001;
                for val_mut in val_list.iter_mut() {
                    let mut val = *val_mut;
                    if val >= 5 { val = val + 3; }
                    val = val << 1 | carry;
                    carry = val >> 4;
                    *val_mut = val & 0b00001111;
                }
                if carry == 1 { val_list.push(1); }
            }
        }
        /*val_listにある0~9までの整数をstringにして戻り値として返す */
        let mut c: Vec<char> = Vec::with_capacity(val_list.len());
        for val in val_list {
            c.push((val + 48) as char);
        }
        c.iter().rev().collect()
    }
}