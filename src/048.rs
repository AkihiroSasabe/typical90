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
        k: usize,
    }
    let mut bba = vec![];
    for i in 0..n {
        input!{
            a_i: usize,
            b_i: usize,
        }
        bba.push(b_i);
        bba.push(a_i - b_i);
    }
    bba.sort();
    bba.reverse();
    let mut answer = 0;
    for i in 0..k {
        // println!("{}", bba[i]);
        answer += bba[i];
    }
    println!("{}", answer);
    
}