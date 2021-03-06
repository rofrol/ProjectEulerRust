#![crate_id = "prob0039"]
#![crate_id = "prob0039"]
#![crate_type = "rlib"]
#![crate_type = "rlib"]

extern crate collections;
extern crate num;
extern crate math;

use std::cmp;
use collections::HashMap;
use num::Integer;
use math::arith;

pub static EXPECTED_ANSWER: &'static str = "840";

pub fn solve() -> ~str {
    // a + b + c = 2m(m + n) <= L
    // 1 <= n <= L / 2m - m
    // if n == 1, a + b + c = 2m^2 + 2m <= L
    // m <= (sqrt(1 + L) - 1)/2
    let limit = 1000;
    let mut map = HashMap::<uint, uint>::new();

    for m in range(1, (arith::isqrt(1 + limit) - 1) / 2) {
        for n in range(1, cmp::min(1 + limit / (2 * m) - m, m)) {
            if (m - n) % 2 == 0 { continue }
            if m.gcd(&n) != 1 { continue }
            let (a, b, c) = (m * m - n * n, 2 * m * n, m * m + n * n);
            let s = a + b + c;
            for k in range(1, limit / s + 1) {
                let new_val = map.find(&(k * s)).map_or(1, |&v| v + 1);
                map.insert(k * s, new_val);
            }
        }
    }

    let (max_key, _max_val) = map.iter().max_by(|&(&_k, &v)| v).unwrap();
    max_key.to_str()
}
