extern crate rand;

use wu::rand::Rng;
use num::bigint::{BigInt};
use num::traits::ToPrimitive;
use std::error;
use std::error::Error;
use std::fmt;


#[derive(Debug)]
pub enum WUHashError {
    ZeroModulus,
}

impl error::Error for WUHashError {
    fn description(&self) -> &str {
        match *self {
            WUHashError::ZeroModulus => "The table size (modulus) is zero!"
        }
    }
}

impl fmt::Display for WUHashError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            WUHashError::ZeroModulus => write!(f, "WUHashError: {}", self.description())
        }
    }
}

#[derive(Debug)]
pub struct WUHash {
    prime: u64,
    modulus: u64,
    a: u64,
    b: u64,
}

#[derive(Debug)]
pub struct WUHashBuilder {
    wh: WUHash,
}

impl WUHash {
    pub fn eval(&self, x: u64) -> usize {
        ((
            (BigInt::from(self.a) * BigInt::from(x) + BigInt::from(self.b))
            % BigInt::from(self.prime)).to_u64().unwrap()
            % self.modulus)
            as usize
    }
}

impl WUHashBuilder {
    pub fn new() -> WUHashBuilder {
        WUHashBuilder{
            wh: WUHash {
                prime: 18446744073709551557,
                modulus: 0,
                a: rand::thread_rng().gen_range(1, 18446744073709551557),
                b: rand::thread_rng().gen_range(1, 18446744073709551557),
            }
        }
    }

    pub fn prime(&mut self, prime: u64) -> &mut WUHashBuilder {
        self.wh.prime = prime;
        self
    }

    pub fn modulus(&mut self, r: u64) -> &mut WUHashBuilder {
        self.wh.modulus = r;
        self
    }

    pub fn a(&mut self, a: u64) -> &mut WUHashBuilder {
        self.wh.a = a;
        self
    }

    pub fn b(&mut self, b: u64) -> &mut WUHashBuilder {
        self.wh.b = b;
        self
    }

    pub fn finalize(&self) -> Result<WUHash, WUHashError> {
        if self.wh.modulus != 0 {
            Ok(WUHash {
                prime: self.wh.prime,
                modulus: self.wh.modulus,
                a: self.wh.a,
                b: self.wh.b,
            })
        } else {
            Err(WUHashError::ZeroModulus)
        }

    }
}

#[cfg(test)]
mod test {
    use super::*;


    #[test]
    #[should_panic]
    fn no_modulus() {
        WUHashBuilder::new()
                    .finalize()
                    .unwrap();
    }

    #[test]
    fn valid() {
        WUHashBuilder::new()
                    .modulus(10)
                    .finalize()
                    .unwrap();
    }

    #[test]
    fn distributed() {
        const TABLE_SIZE: u64 = 1000;
        let wh = WUHashBuilder::new()
                    .modulus(TABLE_SIZE)
                    .finalize()
                    .unwrap();

        let mut ys: [u64; TABLE_SIZE as usize] = [0; TABLE_SIZE as usize];

        for i in 0..TABLE_SIZE {
            ys[wh.eval(i)] += 1
        }

        for i in 0..TABLE_SIZE {
            println!("{} has {}", i, ys[i as usize]);
            if ys[i as usize] > TABLE_SIZE / 100 {
                panic!("WUHash is not distributed")
            }
        }
    }
}
