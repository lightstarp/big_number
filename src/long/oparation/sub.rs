use super::Uvec;
use std::num::Wrapping;

pub fn sub(a: &Uvec,b: &Uvec) -> Uvec {
    let len = std::cmp::max(a.unit.len(), b.unit.len());
    let mut val: u128 = 0;
    let mut carry: u128 = 0;
    let mut is_zero: Vec<bool> = vec![false; len];
    let mut result: Uvec = Uvec{ unit: vec![0; len] };
    for i in 0..len {
        val =(Wrapping(if a.unit.len() <= i {0u128} else {a.unit[i] as u128})
            -(Wrapping(if b.unit.len() <= i {0u128} else {b.unit[i] as u128})
            + Wrapping(carry))).0;
        result.unit[i] = val as u64;
        is_zero[i]     = val as u64 == 0;
        carry = 0xFFFF & (Wrapping(!(val >> 64)) + Wrapping(1)).0;
    }
    if carry != 0 {
        panic!("PANIC!! sub function overflow");
    }
    for &item in is_zero.iter().rev() {
        if item {
            result.unit.pop();
        } else {
            break;
        }
    }
    result
}