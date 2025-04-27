#[derive(Debug)]
struct Uvec {
    value: Vec<u64>,
}

impl Uvec {
    fn create(n: u64) -> Uvec {
        let tmp: Vec<u64> = vec![n];
        Uvec{
            value: tmp
        }
    }
    fn add(a: &Uvec, b: &Uvec) -> Uvec{
        
        let len = std::cmp::max(a.value.len(), b.value.len());

        let mut value: u128 = 0;
        let mut carry: u64 = 0;
        let mut result: Uvec = Uvec{ value: vec![0; len] };
        for i in 0..len {
            value = if a.value.len() <= i {0u128} else {a.value[i] as u128} 
                  + if b.value.len() <= i {0u128} else {b.value[i] as u128};
            
            result.value[i] = value as u64 + carry as u64;
            carry = (value >> 64) as u64

        }
        if value != 0 {
            result.value.push(value as u64);
        }
        println!("{:?}",result);
        result
    }
    fn mul(a: &Uvec, b: &Uvec) -> Uvec{

        let mut value: u128 = 0;
        let mut carry: u128 = 0;
        let mut result: Uvec = Uvec{ value: vec![0; a.value.len() + b.value.len() - 1] };

        for i in 0..a.value.len() {
            for j in 0..b.value.len() {
                value =(a.value[i] as u128
                      * b.value[j] as u128)
                      + carry
                      + result.value[i+j] as u128;
                result.value[i+j] = value as u64;  /*valueの繰り上がりではない部分をresultに入れる*/
                carry = value >> 64;               /*valueの繰り上がりの部分をcarryに入れる*/
            }
        }
        if carry != 0 {
            result.value.push(carry as u64);
        }
        println!("{:?}",result.value);
        result
    }
}

struct Ivec {
    value: u64,
}

pub fn test() {
    let mut a: Uvec = Uvec::create(1534222);
    let mut b: Uvec = Uvec::create(15722);
    let mut c: Uvec = Uvec::create(25);

    a = Uvec::mul(&a, &b);
    a = Uvec::mul(&a, &b);
    a = Uvec::mul(&a, &b);
    a = Uvec::mul(&a, &b);
    a = Uvec::mul(&a, &b);
    a = Uvec::mul(&a, &b);
    a = Uvec::mul(&a, &b);
    a = Uvec::mul(&a, &b);
    a = Uvec::mul(&a, &b);
    a = Uvec::mul(&a, &b);
    a = Uvec::mul(&a, &b);
    a = Uvec::mul(&a, &b);
}