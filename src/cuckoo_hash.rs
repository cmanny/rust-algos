use std::vec::Vec;
use wu_hash::{WUHashBuilder, WUHash};


struct Nest<T> {
    hash_used: u32,
    obj: T
}

trait CuckooHashable {
    fn cuckoo_hash(&self) -> u64;
}

impl CuckooHashable for usize {
    fn cuckoo_hash(&self);
}

#[derive(Debug)]
pub struct CuckooHash<K, V> {
    internal_size: u64,
    wu_hash_1: WUHash,
    wu_hash_2: WUHash,
    elements: Vec<Nest<V>>,
}

impl<K, V> CuckooHash<K, V> {
    fn new() -> CuckooHash<K, V> {
        CuckooHash {
            internal_size: 100,
            wu_hash_1: WUHashBuilder::new().modulus(100).finalize().unwrap(),
            wu_hash_2: WUHashBuilder::new().modulus(100).finalize().unwrap(),
            elements: Vec::new()
        }
    }

    fn add(&self, key: K, obj: V) {

    }
}


#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn what(){
        let x = CuckooHash::new();
        ()
    }
}
