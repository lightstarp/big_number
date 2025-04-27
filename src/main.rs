/*
    1010
01100100

  4
 32
 64

100



F128
+--------+
|i64     |
|mantissa|
+--------+
|i64     |
|power   |
+--------+

F128::add
[power near]---->cast big
     |       NO
     | YES
     v
 power fix
     |
     |
     v
add mantissa

F128::mul
pow = a_p + b_p
man = a_m * b_m

F128::pow
[b_p small]---->Error
     |       NO
     | YES
     v
pow_pow = b_p
pow_man = a_p * b_m
man     = a_m

F128::e_next
pow = b_p
man = a_p * b_m + log_10 a_m

F128::shift
pow = a_p + shift
man = a_m / shift


pow_pow = b_p
pow_man = a_p * b_m
man     = log_10 a_m

F128::arr
pow_pow = b_p
pow_man = a_p * b_m
man     = log_10 a_m


MultiFloat
+----------+    
|F128      |
|+--------+|
||i64     ||
||mantissa||
|+--------+|
||i64     ||
||power   ||
|+--------+|
+----------+
| u64      |
| e_count  |
+----------+

|0000|0000|0000|01100100    shift
|0000|0000|0000|1100100     shift
|0000|0000|0001|100100      shift
|0000|0000|0011|00100       shift
|0000|0000|0110|0100        shift
|0000|0001|0010|100         add0 -> 1001 & shift
|0000|0010|0100|00          shift
|0000|0100|1000|0           shift
|0000|1000|0110|            add0 -> 1011 & shift

*/

mod long;

fn main() {
    long::vec::test();
    /*
    let now = time::Instant::now();

    for j in 0..100 {
        for _i in 0..100 {
            let mut number: i8 = j;
            number = number / 17;
        };
    };
    println!("u8  {:?}", now.elapsed());

    for j in 0..100 {
        for _i in 0..100 {
            let mut number: i16 = j;
            number = number / 17;
        };
    };
    println!("u16 {:?}", now.elapsed());

    for j in 0..100 {
        for _i in 0..100 {
            let mut number: f32 = j as f32;
            number = number / 27.33713477766;
        };
    };
    println!("u32 {:?}", now.elapsed());

    for j in 0..100 {
        for _i in 0..100 {
            let mut number: f64 = j as f64;
            number = number / 27.33713466666;
        };
    };
    println!("u64 {:?}", now.elapsed());
    */
    /*
    let mut a: u64 = 1;
    println!("{}",format!("{:b}",a));
    for i in 0..8 {
        a *= 10;
        println!("{}",format!("{}",a));
        println!("{}",format!("{:b}",a));
    }
        */
}