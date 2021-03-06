#![crate_id = "prob0092"]
#![crate_id = "prob0092"]
#![crate_type = "rlib"]
#![crate_type = "rlib"]

extern crate num;

use std::slice;
use num::Integer;

pub static EXPECTED_ANSWER: &'static str = "8581146";

fn square_digit_sum(mut n: uint) -> uint {
    let mut sum = 0;
    while n > 0 {
        let (d, m) = n.div_rem(&10);
        sum += m * m;
        n = d;
    }
    return sum;
}

fn is_reach_89(n: uint, map: &mut [Option<bool>]) -> bool {
    if n >= map.len() {
        return is_reach_89(square_digit_sum(n), map);
    }

    match map[n] {
        Some(b) => { return b; }
        None => { }
    }

    let result = is_reach_89(square_digit_sum(n), map);
    map[n] = Some(result);
    return result;
}

pub fn solve() -> ~str {
    let limit = 10000000;
    let mut cnt = 0u;

    let vec_size = 81 * 7 + 1;
    let mut map = slice::from_elem(vec_size, None);
    map[0] = Some(false);
    map[1] = Some(false);
    map[89] = Some(true);
    for n in range(1u, limit + 1) {
        if is_reach_89(n, map) { cnt += 1; }
    }
    return cnt.to_str();
}
