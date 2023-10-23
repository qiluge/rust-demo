use ethers::prelude::*;
use std::time::{SystemTime, UNIX_EPOCH};

fn main() {
    let mut f = 0f64;
    let mut start = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_nanos();
    for i in 0..1000 {
        f = 99.99 * 11.11;
    }
    let mut end = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_nanos();
    println!("{}", end - start);
    let mut r = U256::zero();
    let u = U256::from(9999)*(U256::from(1_000_000_000_000_000_000i64));
    let d = U256::from(1111)*(U256::from(1_000_000_000_000_000_000i64));
    start = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_nanos();
    for i in 0..1000 {
        r = d * u;
    }
    end = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_nanos();
    println!("{}", end - start);
}
