extern crate rand;

use wu::rand::Rng;
use std::num::Wrapping;

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
        (((self.b.wrapping_add(self.a.wrapping_mul(x))) % self.prime) % self.modulus) as usize
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

    pub fn finalize(&self) -> WUHash {
        if self.wh.modulus != 0 {
            WUHash {
                prime: self.wh.prime,
                modulus: self.wh.modulus,
                a: self.wh.a,
                b: self.wh.b,
            }
        } else {
            panic!("WUHash modulus 0, will not continue");
        }

    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    #[should_panic]
    fn no_modulus() {
        let wh = WUHashBuilder::new()
                    .finalize();
    }

    #[test]
    fn valid() {
        let wh = WUHashBuilder::new()
                    .modulus(10)
                    .finalize();
    }

    #[test]
    fn distributed() {
        const TABLE_SIZE: u64 = 1000;
        let wh = WUHashBuilder::new()
                    .modulus(TABLE_SIZE)
                    .finalize();

        let mut ys: [u64; TABLE_SIZE as usize] = [0; TABLE_SIZE as usize];

        for i in 0..TABLE_SIZE {
            ys[wh.eval(i)] += 1
        }

        for i in 0..TABLE_SIZE {
            if ys[i as usize] > TABLE_SIZE / 100 {
                panic!("WUHash is not distributed")
            }
        }
    }
}
