#[link(name = "prob0058", vers = "0.0")];
#[crate_type = "lib"];

extern mod common;

use common::prime;

pub static expected_answer: &'static str = "26241";

pub fn solve() -> ~str {
    let mut side = 1;
    let mut num_prime = 0;
    let mut num_total = 1;

    loop {
        side += 2;
        let rb = side * side;
        let lb = rb - side + 1;
        let lt = lb - side + 1;
        let rt = lt - side + 1;
        if prime::contains(lb) { num_prime += 1; }
        if prime::contains(lt) { num_prime += 1; }
        if prime::contains(rt) { num_prime += 1; }
        num_total += 4;
        if num_prime * 10 < num_total { break; }
    }
    return side.to_str();
}
