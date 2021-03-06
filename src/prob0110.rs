#![crate_id = "prob0110"]
#![crate_id = "prob0110"]
#![crate_type = "rlib"]
#![crate_type = "rlib"]

extern crate collections;
extern crate math;
use collections::priority_queue::PriorityQueue;
use math::prime::Prime;

pub static EXPECTED_ANSWER: &'static str = "9350130049860600";

struct Elem (uint, ~[uint]);

impl Eq for Elem {
    fn eq(&self, other: &Elem) -> bool {
        let &Elem(s, _) = self;
        let &Elem(o, _) = other;
        s == o
    }
}
impl Ord for Elem {
    fn lt(&self, other: &Elem) -> bool {
        let &Elem(s, _) = self;
        let &Elem(o, _) = other;
        s.gt(&o)
    }
}


pub fn solve() -> ~str {
    let limit = 4000000;

    let prime = Prime::new();
    let mut queue = PriorityQueue::new();
    queue.push(Elem(2u, ~[1u]));

    loop {
        let Elem(n, pairs) = queue.pop();
        let num_sol = (pairs.iter().fold(1, |n, &i| n * (2 * i + 1)) + 1) / 2;
        if num_sol > limit {
            return n.to_str();
        }
        if pairs.len() == 1 || pairs[pairs.len() - 1]  < pairs[pairs.len() - 2] {
            let mut new_pairs = pairs.clone();
            new_pairs[pairs.len() - 1] += 1;
            queue.push(Elem(n * prime.nth(pairs.len() - 1), new_pairs));
        }
        let mut pairs = pairs;
        pairs.push(1);
        queue.push(Elem(n * prime.nth(pairs.len() - 1), pairs));
    }
}
