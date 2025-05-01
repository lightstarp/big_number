use super::Uvec;

struct UvecScope<'a> {
    main: &'a Uvec,
    first: usize,
    last: usize,
}

impl UvecScope<'_> {
    fn split(&self, first: usize, last: usize) -> UvecScope {
        UvecScope { main: &self.main, first, last }
    }
    fn create(main: &Uvec) -> UvecScope {
        UvecScope { main, first: 0, last: main.unit.len() }
    }
}

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


fn mul_u(a: &UvecScope, b: u64) -> Uvec {

    let mut val: u128;
    let mut carry: u128 = 0;
    let mut result: Uvec = Uvec{ unit: vec![] };

    for &item in a.main.unit.iter() {
        val = item as u128 * b as u128 + carry;
        result.unit.push(val as u64);   /*valの繰り上がりではない部分をresultに入れる*/
        carry = val >> 64;              /*valの繰り上がりの部分をcarryに入れる*/
    }
    if carry != 0 {
        result.unit.push(carry as u64);
    }

    result
}

//  fn sub(a: &mut [u64], b: &mut [u64]) -> [u64] {
//      let a_iter = a.iter();
//      let b_iter = b.iter();
//  }

//fn karatsuba(a: &Uvec, b: &Uvec) -> Uvec{
//  let result = karatsuba_loop(&UvecScope::create(a), &UvecScope::create(b));
//  result.main
//}
/*
fn karatsuba_loop(a: &UvecScope, b: &UvecScope) -> UvecScope {

    if a.last == 1 {
        //return mul_u(&b, a[0]);
    }
    if b.last == 1 {
        //return mul_u(&a, b[0]);
    }
    let a_split = a.last >> 1;
    let b_split = b.last >> 1;

    let r0 = karatsuba_loop(&a.split(0, a_split),&b.split(0, b_split));
    let r2 = karatsuba_loop(&a.split(a_split, a.last),&b.split(b_split, b.last));

    r0 + r2 - karatsuba_loop(
        &a.split(0      , a_split) - &b.split(b_split, b.last)
       ,&a.split(a_split, a.last)  - &b.split(0      , b_split));

    r0
}
    */