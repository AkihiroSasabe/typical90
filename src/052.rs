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
        a: [[usize; 6]; n]
    }

    let modulo = 1_000_000_000 + 7;
    let mut answer = 1;

    for i in 0..n {
        let mut sum = 0;
        for j in 0..6 {
            sum = sum + a[i][j];
        }
        answer = (answer * sum)  % modulo;
    }
    println!("{}", answer % modulo);

    // a11 a12
    // a21 a22

    // a11*a21 + a11*a22
    // a12*a21 + a12*a22
    


}