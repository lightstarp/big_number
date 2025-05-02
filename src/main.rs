mod long;

fn main() {
    example1();
    example2();
    example3();
}

fn example1() {
    /*2^1024を求めて出力する関数*/
    let mut result   = long::create_set(1);
    let     multiple = long::create_set(2);
    for _i in 0..1024 {
        result = long::mul(&result,&multiple);
    }
    println!("Example1: 2^1024 is");
    println!("{}",result.to_string());
    println!("");
}

fn example2() {
    /*100の階乗を求めて出力する関数*/
    let mut result   = long::create_set(1);
    let mut multiple = long::create_set(100);
    let     remove   = long::create_set(1);
    for _i in 0..100 {
        result   = long::mul(&result  ,&multiple);
        multiple = long::sub(&multiple,&remove  );
    }
    println!("Example2: factorial of 100 is");
    println!("{}",result.to_string());
    println!("");
}

fn example3() {
    /*大きい数のビットシフトを求めて出力する関数*/
    let mut number   = long::create();
    number.push(123456789);
    number.push(987654321);
    number.push(333333333);

    let mut result = number.clone();
    result.shl(16);

    println!("Example3: bitshift is");
    println!("{}",number.to_string2());
    println!("{}",result.to_string2());
    println!("");
}