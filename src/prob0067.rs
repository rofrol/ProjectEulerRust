#![crate_id = "prob0067"]
#![crate_id = "prob0067"]
#![crate_type = "rlib"]
#![crate_type = "rlib"]

use std::{cmp, slice};
use std::io::{BufferedReader, File};

pub static EXPECTED_ANSWER: &'static str = "7273";

pub fn solve() -> ~str {
    let mut br = BufferedReader::new(
        File::open(&Path::new("files/triangle.txt")).ok().expect("file not found."));

    let triangle = br.lines()
        .filter_map(|line| line.ok())
        .filter(|line| !line.is_empty())
        .map(|line| line.words().filter_map(from_str::<uint>).collect::<~[uint]>())
        .collect::<~[~[uint]]>();
    let init = triangle.init();
    let last = triangle.last().unwrap();
    init.rev_iter().fold(last.to_owned(), |prev, elem| {
            slice::from_fn(elem.len(), |i| {
                    elem[i] + cmp::max(prev[i], prev[i + 1])
                })
        })[0].to_str()
}

