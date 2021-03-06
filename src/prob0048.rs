#![crate_id = "prob0048"]
#![crate_id = "prob0048"]
#![crate_type = "rlib"]
#![crate_type = "rlib"]

extern crate math;
use math::arith;

pub static EXPECTED_ANSWER: &'static str = "9110846700";

pub fn solve() -> ~str {
    let modulo  = 100_0000_0000;
    let mut sum = 0;
    for n in range(1u, 1000 + 1) {
        sum = (sum + arith::mod_pow(n, n, modulo)) % modulo;
    }
    return sum.to_str();
}
