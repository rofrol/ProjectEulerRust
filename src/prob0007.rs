#![crate_id = "prob0007"]
#![crate_id = "prob0007"]
#![crate_type = "rlib"]
#![crate_type = "rlib"]

extern crate math;

use math::prime::Prime;

pub static EXPECTED_ANSWER: &'static str = "104743";

pub fn solve() -> ~str {
    return Prime::new().nth(10000).to_str();
}
