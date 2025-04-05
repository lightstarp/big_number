/*
    1010
01100100

  4
 32
 64

100


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

use std::{backtrace, io};

#[derive(Clone, Debug)]
struct SimpleNumber {
    value: u64,
}

impl SimpleNumber {
}

#[derive(Clone, Debug)]
struct BigNum {
    mantissa: u64,
    power: u64,
    e_count: u64,
}

impl BigNum {
    fn string(&self) -> String {
        format!("{:20}x{}",self.mantissa,self.power)
    }
    fn format(&self) -> String {
        //3.5269E11         log_2 ,10
        //0.4D104D2A2A4B04  log_10,2
        //0.4D104D427DE7FC1
        //  .   .   .   .
        let step1   : u32   = self.mantissa.ilog2();
        let step2   : u128  = (step1 as u128 + self.power as u128) * 0x04D104D427DE7FC1;
        let power   : u64   = (step2 >> 64) as u64;
        let mantissa: u64   = (step2      ) as u64;
        //step4
        format!("{:20}e{}",self.mantissa,self.power)
    }
    fn set(&mut self, num: u64) {
        *self = BigNum {
            mantissa: num,
            power: 0,
            e_count: 0
        };
    }
    fn add(&mut self,number: &BigNum) {
        self.mantissa = self.mantissa + number.mantissa;
    }
    fn mul(&mut self,number: &BigNum) {
        self.power     = self.power            + number.power;
        let mut result = self.mantissa as u128 * number.mantissa as u128;
        {
            let mut count = 0;
            let mut digit = 1;
            for i in 0..48 {
                if result <= digit * 10000000000000000 {
                    result = result / digit;
                    self.power = self.power + count;
                    break;
                };
                count = count + 1;
                digit = digit * 10;
            }
        }

        self.mantissa = result as u64;

        /*
        let zero_count = result.leading_zeros();
        let shift_count = if zero_count < 64 {
            64 - zero_count
        } 
        else {
            0
        };

        self.mantissa = (result >> shift_count) as u64;
        self.power = self.power + number.power + shift_count as u64;
        */
    }
}

fn build_bignum() -> BigNum {
    BigNum {
        mantissa: 0,
        power: 0,
        e_count: 0
    }
}

fn get_input() -> String {
    let mut word = String::new();
    io::stdin().read_line(&mut word).ok();
    return word.trim().to_string();
}

fn main() {
    /*
    let mut a: u64 = 1;
    println!("{}",format!("{:b}",a));
    for i in 0..8 {
        a *= 10;
        println!("{}",format!("{}",a));
        println!("{}",format!("{:b}",a));
    }
        */

    let mut a: BigNum = build_bignum();
    a.set(1);
    let mut b: BigNum = build_bignum();
    b.set(2);
    for i in 0..128 {
        a.mul(&b);
        println!("{}",a.format());
    }
}