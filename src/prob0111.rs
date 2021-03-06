#![crate_id = "prob0111"]
#![crate_id = "prob0111"]
#![crate_type = "rlib"]
#![crate_type = "rlib"]

extern crate data;
extern crate math;

use std::num;
use data::extiter::Comb;
use math::numconv;
use math::prime::Prime;

pub static EXPECTED_ANSWER: &'static str = "612407567715";

pub fn solve() -> ~str {
    let n = 10u;
    let prime = Prime::new();
    let mut total = 0;
    for d in range(0u, 10) {
        for m in range(0, n + 1).rev() {
            let mut sum = 0;
            for other_ds in range(0, num::pow(9u, n - m)) {
                let other_ds = {
                    let mut ds = numconv::to_digits(other_ds, 9).rev().collect::<~[uint]>();
                    while ds.len() < n - m { ds.unshift(0); }
                    for i in ds.mut_iter() { if *i >= d { *i += 1; } }
                    ds
                };

                for set in Comb::new(n - m, n) {
                    let first = if set.contains(&0) { other_ds[0] } else { d };
                    if first == 0 { continue; }

                    let mut j = 0;
                    let mut num = 0;
                    for i in range(0, n) {
                        num = num * 10 + if set.contains(&i) {
                            j += 1;
                            other_ds[j - 1]
                        } else {
                            d
                        };
                    }
                    if prime.contains(num) { sum += num; }
                }
            }
            if sum > 0 {
                total += sum;
                break;
            }
        }
    }
    total.to_str()
}
