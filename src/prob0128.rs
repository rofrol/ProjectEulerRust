//! Problem 128 (https://projecteuler.net/problem=128)
//!
//! # 定義
//!
//! 最内のリングを 0 周目のリングとする。
//! `r` 周目のリングの要素数を `a(r)` 、
//! `r` 週目のリングの、真北から数えて `m` 番目のセルの数値を `b(r, m)` とそれぞれおくと、
//!
//! ``` {.math .notrust}
//! a(0) = 1
//! a(r) = 6r if r > 0
//!
//! b(0, 0) = 1
//! b(r, m) = \sum_{k=0}^{r-1} a(k) + 1 + m
//!         = 1 + 6 + 12 + ... + 6(r-1) + 1 + m
//!         = 2 + 3r(r-1) + m
//! ```
//!
//! となる。利便性のため、任意の `m` について、`b(r, m)` は以下を満たすとする。
//!
//! ``` {.math .notrust}
//! b(r, m) = b(r, m + a(r)) if m < 0 or m >= 6r
//! ```
//!
//! また、上記式より以下を得る。
//!
//! ``` {.math .notrust}
//! b(r+1, m_a) - b(r, m_b) = 6r + m_a - m_b
//! b(r, m_a) - b(r, m_b)   = m_a - m_b
//! b(r, m_a) - b(r-1, m_b) = 6(r-1) + m_a - m_b
//! ```
//!
//! _r_ 周目のリング中のセルは、リングのどの部分に属しているかで以下のように分類する。
//!
//! * 角 - `m = rk` を満たすセル。 `r+1` 周目のセル3つと、`r-1` 周目のセル 1 つと隣接する。
//!        `r > 0` の周に存在。
//! * 辺 - 角ではないセル。`r+1` 周目のセルと、 `r-1` 周目のセルそれぞれに 2 つに隣接する。
//!        `r > 1` の周に存在。
//!
//! # 解析
//!
//! ## `r=0` の場合
//!
//! 周囲のセルの数値との差は `1`. `2`, `3`, `4`, `5`, `6` であり、
//! `2`, `3`, `5` が素数なので、 `PD(1) = 3` となる。
//!
//! ## 辺の場合
//!
//! `r` 周目の辺に属するセルの場合、`r+1` 周目の連続したセル2つおよび
//! `r-1` 週目の連続したセル2つと隣接する。
//!
//! ### `m \neq 6r-1` の場合
//
//! `m \neq 6r-1` の場合、連続した 2 つセルとの差 2 つは、いずれかは必ず偶数となる。
//! よって、これら 4 つのセルの数値との差のうち素数となるのは最大 2 つである。
//! すなわち、辺のセルについては `PD(n)` が 3 となることはない。
//!
//! ### `m = 6r-1` の場合
//!
//! `m = 6r-1` の場合、隣接するセルの数値との差は以下である。
//!
//! * `b(r+1, 6r+4) - b(r, 6r-1)   = 6r+5`
//! * `b(r+1, 6r+5) - b(r, 6r-1)   = 6(r+1)`
//! * `b(r, 6r-1)   - b(r, 6r-2)   = 1`
//! * `b(r, 6r-1)   - b(r, 0)      = 6r-1`
//! * `b(r, 6r-1)   - b(r-1, 0)    = 12r-7`
//! * `b(r, 6r-1)   - b(r-1, 6r-7) = 6r`
//!
//! `6(r+1)`, `1`, `6r` は素数ではないため、`PD(n) = 3` となるセルは、
//! `6r+5`, `6r-1`, `12r-7` が素数でなければならない。
//!
//! なお、 `b(r, 6r-1) = 3r^2 + 3r + 1` である。
//!
//! ## 角の場合
//!
//! 角のセルに隣接するセルの数値は、以下の6種類である。ここで、 `k = m/r` である。
//!
//! * `b(r+1, rk-1+k)`
//! * `b(r+1, rk+k)`
//! * `b(r+1, rk+1+k)`
//! * `b(r, rk-1)`
//! * `b(r, rk+1)`
//! * `b(r-1, rk-k)`
//!
//! ### `k > 0` の場合
//!
//! `k > 0` の場合、セルと隣接するセルの数値との差は以下である。
//!
//! * `b(r+1, rk-1+k) - b(r, rk)     = 6r + k - 1`
//! * `b(r+1, rk+k)   - b(r, rk)     = 6r + k`
//! * `b(r+1, rk+1+k) - b(r, rk)     = 6r + k + 1`
//! * `b(r, rk)       - b(r, rk-1)   = 1`
//! * `b(r, rk+1)     - b(r, rk)     = 1`
//! * `b(r, rk)       - b(r-1, rk-k) = 6(r-1) + k`
//!
//! `k` が偶数の場合、`6r+k`, `6(r-1)+k` は偶数となる。
//! `k` が奇数の場合、`6r+k-1`, `6r+k+1` は偶数となる。
//! よって、`k > 1` の場合、`PD(3)` となることはない。
//!
//! ### `k = 0` の場合
//!
//! `k = 0` の場合、セルと隣接するセルの数値との差は以下である。
//!
//! * `b(r+1, -1) - b(r, 0)   = b(r+1, 6r+5) - b(r, 0) = 12r + 5`
//! * `b(r+1, 0)  - b(r, 0)   = 6r`
//! * `b(r+1, 1)  - b(r, 0)   = 6r + 1`
//! * `b(r, -1)   - b(r, 0)   = b(r, 6r-1) - b(r, 0) = 6r - 1`
//! * `b(r, 1)    - b(r, 0)   = 1`
//! * `b(r, 0)    - b(r-1, 0) = 6r(r-1)`
//!
//! `6r`, `6r(r-1)`, `1` は素数ではないため、`PD(n) = 3` となるためには、
//! `12r+5`, `6r+1`, `6r-1` が素数でなければならない。
//!
//! なお、 `b(r, 0) = 3r^2 - 3r + 2` である。
//!
//! # 解法
//!
//! `r > 0` について、以下すべてが素数の場合、 `PD(n) = 3` となる。
//!
//! ``` {.math .notrust}
//! 12r+5, 6r+1, 6r-1
//! ```
//!
//! `r > 1` について、以下すべてが素数の場合、 `PD(n) = 3` となる。
//!
//! ``` {.math .notrust}
//! 6r-1, 6r-5, 12r-7
//! ```
//!
//! `r=0` から順番にこれらを満たす数をカウントする。

#![crate_id = "prob0128"]
#![crate_id = "prob0128"]
#![crate_type = "rlib"]
#![crate_type = "rlib"]

extern crate math;

use std::mem;
use math::prime::Prime;

pub static EXPECTED_ANSWER: &'static str = "14516824220";

#[deriving(Eq, Show)]
struct PdTriple {
    n: uint,
    r: uint,
    triple: (uint, uint, uint)
}

struct PdTriples {
    r: uint,
    next: Option<PdTriple>
}

impl PdTriples {
    #[inline]
    fn new() -> PdTriples { PdTriples { r: 0, next: None } }
}

impl Iterator<PdTriple> for PdTriples {
    #[inline]
    fn next(&mut self) -> Option<PdTriple> {
        if self.next.is_some() {
            return mem::replace(&mut self.next, None)
        }

        let r = self.r;
        self.r += 1;

        if r == 0 { return Some(PdTriple { n: 1, r: 0, triple: (2, 3, 5) }) }
        if r > 1 {
            let n = 3*r*r + 3*r + 1;
            self.next = Some(PdTriple { n: n, r: r, triple: (6*r-1, 6*r+5, 12*r-7) });
        }
        let n = 3*r*r - 3*r + 2;
        Some(PdTriple { n: n, r: r, triple: (12*r+5, 6*r+1, 6*r-1) })
    }
}

struct Pd3Nums {
    iter: PdTriples,
    ps: Prime
}

impl Pd3Nums {
    #[inline]
    fn new() -> Pd3Nums { Pd3Nums { iter: PdTriples::new(), ps: Prime::new() } }
}

impl Iterator<uint> for Pd3Nums {
    #[inline]
    fn next(&mut self) -> Option<uint> {
        loop {
            let PdTriple { n, triple: (a, b, c), ..} = self.iter.next().unwrap();
            if self.ps.contains(a) && self.ps.contains(b) && self.ps.contains(c) {
                return Some(n)
            }
        }
    }
}

pub fn solve() -> ~str {
    Pd3Nums::new().nth(2000 - 1).unwrap().to_str()
}

#[cfg(test)]
mod tests {
    use super::{PdTriple, PdTriples, Pd3Nums};
    use std::iter::AdditiveIterator;

    fn a(r: uint) -> uint { if r == 0 { 1 } else { 6 * r } }
    fn b(r: uint, m: uint) -> uint {
        if r == 0 {
            assert_eq!(0, m);
            return 1
        }
        assert!(m < 6 * r);
        range(0, r).map(a).sum() + 1 + m
    }

    #[test]
    fn test_a() {
        assert_eq!(1, a(0));
        assert_eq!(6, a(1));
        assert_eq!(12, a(2));
    }

    #[test]
    fn test_b() {
        assert_eq!(1, b(0, 0));
        let mut n = 2;
        for r in range(1u, 10) {
            for m in range(0u, a(r)) {
                assert_eq!(n, b(r, m));
                n += 1;
            }
        }
    }

    #[test]
    fn pd_triples() {
        let mut it = PdTriples::new();
        assert_eq!(Some(PdTriple { n: b(0, 0), r: 0, triple: (2u, 3u, 5u)}), it.next());
        let n = b(1, 0);
        assert_eq!(Some(PdTriple { n: n, r: 1, triple: (b(2, 11) - n,
                                                        b(2, 1)  - n,
                                                        b(1, 5)  - n)}),
                   it.next());

        for r in range(2u, 100) {
            let n = b(r, 0);
            assert_eq!(Some(PdTriple { n: n, r: r, triple: (b(r+1, 6*r+5) - n,
                                                            b(r+1, 1)     - n,
                                                            b(r,   6*r-1) - n) }),
                       it.next());

            let n = b(r, 6*r-1);
            assert_eq!(Some(PdTriple { n: n, r: r, triple: (n - b(r,0),
                                                            b(r+1, 6*r+4) - n,
                                                            n - b(r-1, 0))}),
                       it.next());
        }
    }

    #[test]
    fn pd3_nums() {
        let mut it = Pd3Nums::new();
        assert_eq!(Some(1), it.next());

        let mut it = Pd3Nums::new();
        assert_eq!(Some(271), it.nth(9));
    }
}
