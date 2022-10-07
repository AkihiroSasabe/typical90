use proconio::input;
use itertools::Itertools;
use std::cmp::{max, min};
use std::cmp::Ordering;
use std::collections::VecDeque;
use std::collections::BinaryHeap;
use proconio::marker::Chars;
fn main() {
    input! {
        n: usize,
        k: isize,
        a: [isize; n],
        b: [isize; n]
    }

    let mut diff_sum = 0;
    for i in 0..n {
        diff_sum += (a[i] - b[i]).abs();
    }
    // println!("{}", diff_sum);
    if diff_sum > k {
        println!("No");
        return
    }
    if (diff_sum - k) % 2 == 0 {
        println!("Yes");
    }
    else {
        println!("No");
    }
}