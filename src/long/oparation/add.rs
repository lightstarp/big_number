use super::Uvec;

pub fn add(a: &Uvec,b: &Uvec) -> Uvec {
    let len = std::cmp::max(a.unit.len(), b.unit.len());
    let mut val: u128 = 0;
    let mut carry: u128 = 0;
    let mut result: Uvec = Uvec{ unit: vec![0; len] };
    for i in 0..len {
        val = if a.unit.len() <= i {0u128} else {a.unit[i] as u128} 
            + if b.unit.len() <= i {0u128} else {b.unit[i] as u128}
            + carry;
        result.unit[i] = val as u64;
        carry = val >> 64;
    }
    if carry != 0 {
        result.unit.push(val as u64);
    }
    result
}