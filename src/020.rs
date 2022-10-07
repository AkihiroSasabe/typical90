use proconio::input;
use itertools::Itertools;
use std::cmp::{max, min};
use std::cmp::Ordering;
use std::collections::VecDeque;
use std::collections::BinaryHeap;
use proconio::marker::Chars;
fn main() {
    input! {
        a: usize,
        b: usize,
        c: usize
    }
    
    // log2(a) < b*log2(c) ですか?
    if a < c.pow(b as u32) {
        println!("Yes");
    }
    else {
        println!("No");
    }

    // // c.pow(b as u32)は以下のように計算してもよい
    // let x = 1;
    // for _ in 0..b {
    //     x *= c;
    // }

}