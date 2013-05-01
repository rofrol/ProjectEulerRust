use core::iterator::{ IteratorUtil };

use common::prime::{ Prime, num_of_divisors };
use common::extiter::{ Triangle, ExtIteratorUtil };
use common::problem::{ Problem };

pub static problem: Problem<'static> = Problem {
    id: 12,
    answer: "76576500",
    solver: solve
};

fn solve() -> ~str {
    let mut ps = Prime::new();
    return Triangle::new()
        .transform(|t| (t, num_of_divisors(t, &mut ps)))
        .skip_while(|&(_t, n)| n <= 500)
        .nth(0).first().to_str();
}
