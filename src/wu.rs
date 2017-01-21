extern crate rand;

use wu::rand::Rng;

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
    pub fn eval(&self, x: u64) -> u64 {
        ((self.a * x + self.b) % self.prime) % self.modulus
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

    pub fn finalize(&self) -> Option<WUHash> {
        if self.wh.modulus != 0 {
            Some(
                WUHash {
                    prime: self.wh.prime,
                    modulus: self.wh.modulus,
                    a: self.wh.a,
                    b: self.wh.b,
                }
            )
        } else {
            None
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn no_modulus() {
        let wh = WUHashBuilder::new()
                    .a(453534)
                    .b(5667546343)
                    .finalize();
        assert_eq!(wh.is_some(), false)
    }

    #[test]
    fn valid() {
        let wh = WUHashBuilder::new()
                    .modulus(10)
                    .finalize();
        assert_eq!(wh.is_some(), true)
    }
}
