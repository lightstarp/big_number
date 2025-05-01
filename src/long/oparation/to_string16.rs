use super::Uvec;

impl Uvec {
    pub fn to_string16(&self) -> String {
        /*val_listにある0~9までの整数をstringにして戻り値として返す */
        let mut c: String = Default::default();
        for s in self.unit.iter().rev() {
            c.push_str(&format!("{:x}",s));
        }
        c
    }
}