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

    // 最大公約数
    let gcd0 = gcd(a, (gcd(b, c)));
    let answer = (a / gcd0 - 1) + (b / gcd0 - 1) + (c / gcd0 - 1);    

    println!("{}", answer);

}

// ユークリッドの互除法で最大公約数を求める (Euclidean Algorithm)
// ユークリッドの互除法とは、x < y のとき、gcd(x, y)=gcd(x, y % x)
fn gcd(mut x: usize, mut y:usize) -> usize {
    if y <= x {
        let y_before = y;
        y = x;
        x = y_before;
    } 
    if y % x == 0 {
        return x
    }
    else {
        return gcd(x, y % x);
    }
}