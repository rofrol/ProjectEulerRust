#![crate_id = "prob0102"]
#![crate_id = "prob0102"]
#![crate_type = "rlib"]
#![crate_type = "rlib"]

use std::io::{BufferedReader, File};

pub static EXPECTED_ANSWER: &'static str = "228";

type Point = (int, int);
type Triangle = (Point, Point, Point);

enum Side { L, R, C }

#[inline(alaways)]
fn sub((ax, ay): Point, (bx, by): Point) -> Point { (ax - bx, ay - by) }

#[inline(always)]
fn get_normal(a: Point, b: Point) -> Point {
    let (dx, dy) = sub(b, a);
    return (-dy, dx);
}

#[inline(alawys)]
fn inner_prod((ax, ay): Point, (bx, by): Point) -> int { ax * bx + ay * by }

fn get_side(a: Point, b: Point, p: Point) -> Side {
    let norm = get_normal(b, a);
    return match inner_prod(norm, sub(p, a)).signum() {
        1  => L,
        0  => C,
        -1 => R,
        _  => fail!()
    };
}

fn is_inside((a, b, c): Triangle, p: Point) -> bool {
    match (get_side(a, b, p), get_side(b, c, p), get_side(c, a, p)) {
        (L, L, L) | (L, L, C) | (L, C, L) | (L, C, C)
        | (C, L, L) | (C, L, C) | (C, C, L) | (C, C, C)
        | (R, R, R) | (R, R, C) | (R, C, R) | (R, C, C)
        | (C, R, R) | (C, R, C) | (C, C, R) => true,
        _ => false
    }
}

pub fn solve() -> ~str {
    let mut br = BufferedReader::new(
        File::open(&Path::new("files/triangles.txt")).ok().expect("file not found."));

    let mut cnt = 0u;
    for line in br.lines().filter_map(|line| line.ok()) {
        let ns = line
            .trim()
            .split(',')
            .filter_map(from_str::<int>)
            .collect::<~[int]>();
        let ps = ((ns[0], ns[1]), (ns[2], ns[3]), (ns[4], ns[5]));
        if is_inside(ps, (0, 0)) { cnt += 1; }
    }
    cnt.to_str()
}
