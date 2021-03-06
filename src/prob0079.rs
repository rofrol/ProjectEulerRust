#![crate_id = "prob0079"]
#![crate_id = "prob0079"]
#![crate_type = "rlib"]
#![crate_type = "rlib"]

extern crate collections;

use std::char;
use std::io::{BufferedReader, File};
use std::hash::Hash;
use collections::{HashMap, HashSet};

pub static EXPECTED_ANSWER: &'static str = "73162890";

struct Relation<T> {
    num_prec: uint,
    succ: HashSet<T>
}

impl<T: Hash + TotalEq> Relation<T> {
    fn new() -> Relation<T> { Relation { num_prec: 0, succ: HashSet::new() } }
}

struct Relations<T> {
    top: HashMap<T, Relation<T>>
}

impl<T: Hash + TotalEq + Clone> Relations<T> {
    fn new() -> Relations<T> { Relations { top: HashMap::new() } }

    fn set_dependant(&mut self, prec: T, succ: T) {
        if !self.top.contains_key(&prec) {
            self.top.insert(prec.clone(), Relation::new());
        }
        if !self.top.contains_key(&succ) {
            self.top.insert(succ.clone(), Relation::new());
        }

        let mut contained = true;
        match self.top.find_mut(&prec) {
            Some(s) => {
                if !s.succ.contains(&succ) {
                    s.succ.insert(succ.clone());
                    contained = false;
                }
            }
            None => { fail!() }
        }
        if !contained {
            match self.top.find_mut(&succ) {
                Some(p) => { p.num_prec += 1; }
                None => { fail!(); }
            }
        }
    }

    fn find_all_not_preceded(&self) -> ~[T] {
        self.top
            .iter()
            .filter(|&(_k, v)| v.num_prec == 0)
            .map(|(k, _v)| k.clone())
            .collect()
    }

    fn delete_and_find(&mut self, prec: T) -> ~[T] {
        let mut result = ~[];
        self.top.pop(&prec).map(|p| {
            for s in p.succ.iter() {
                match self.top.find_mut(s) {
                    Some(y) => {
                        y.num_prec -= 1;
                        if y.num_prec == 0 {
                            result.push(s.clone());
                        }
                    }
                    None => {}
                }
            }
        });
        return result;
    }
}

fn tsort<T: Hash + TotalEq + Clone>(rels: &mut Relations<T>) -> ~[T] {
    let mut sorted = ~[];
    let mut queue = rels.find_all_not_preceded();
    while !queue.is_empty() {
        let prec = queue.shift().unwrap();
        sorted.push(prec.clone());
        queue.push_all(rels.delete_and_find(prec));
    }
    return sorted;
}


pub fn solve() -> ~str {
    let mut br = BufferedReader::new(
        File::open(&Path::new("files/keylog.txt")).ok().expect("file not found."));

    let mut rels = Relations::new();
    for line in br.lines().filter_map(|line| line.ok()) {
        let ds = line.chars()
            .filter_map(|c| char::to_digit(c, 10)).collect::<~[uint]>();
        for i in range(1, ds.len()) {
            rels.set_dependant(ds[i - 1], ds[i]);
        }
    }
    tsort(&mut rels).iter().map(|d| d.to_str()).collect::<~[~str]>().concat()
}
