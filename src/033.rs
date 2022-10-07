use proconio::input;
use itertools::Itertools;
use std::cmp::{max, min};
use std::cmp::Ordering;
use std::collections::VecDeque;
use std::collections::BinaryHeap;
use proconio::marker::Chars;

// #.#.#
// .....
// #.#.#

// #.
// ..
// #.

// #.#.
// ....
// #.#.

// #.#
// ...
// #.#
// ...

// #.#.#.#.
// ########

fn main() {
    input! {
        h: usize,
        w: usize
    }

    if (h ==1 || w == 1) {
        println!("{}", h * w);
        return
    }
    let mut answer = (h / 2 + h % 2) * (w/2 + w%2);
    println!("{}", answer);
}