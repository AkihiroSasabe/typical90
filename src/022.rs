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

// ユークリッドの互除法で最小公倍数を求める
// ユークリッドの互除法とは、x < y のとき、gcd(x, y)=gcd(x, y%x)
fn gcd(mut x: usize, mut y:usize) -> usize {
    if y <= x {
        let y_pre = y.clone();
        y = x.clone();
        x = y_pre;
    } 
    let amari = y % x;
    if amari == 0 {
        return x
    }
    else {
        y = y % x;
        return gcd(x, y);
    }

}