use super::Uvec;

pub fn mul(a: &Uvec,b: &Uvec) -> Uvec {
    let mut val: u128;
    let mut carry: u128 = 0;
    let mut result: Uvec = Uvec{ unit: vec![0; a.unit.len() + b.unit.len() - 1] };
    for i in 0..a.unit.len() {
        for j in 0..b.unit.len() {
            val =(a.unit[i] as u128
                  * b.unit[j] as u128)
                  + carry
                  + result.unit[i+j] as u128;
            result.unit[i+j] = val as u64;  /*valの繰り上がりではない部分をresultに入れる*/
            carry = val >> 64;              /*valの繰り上がりの部分をcarryに入れる*/
        }
    }
    if carry != 0 {
        result.unit.push(carry as u64);
    }
    result
}