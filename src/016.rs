use proconio::input;
use itertools::Itertools;
use std::cmp::{max, min};
use std::cmp::Ordering;
use std::collections::VecDeque;
use std::collections::BinaryHeap;
use proconio::marker::Chars;

fn main() {
    input! {
        n: isize,
        a: isize,
        b: isize,
        c: isize
    }

    let mut coins = vec![a, b, c];
    coins.sort();

    let max_num = 10000;
    let mut answer = max_num;
    for i in 0..max_num {
        for j in 0..(max_num-i) {
            let nokori = n - (i * coins[2] + j * coins[1]);
            if nokori < 0 {continue}
            let k = nokori / coins[0];
            let amari = nokori % coins[0];
            if amari == 0 && i + j + k < max_num{
                // println!("i, j, k: {} {} {}", i, j, k);
                answer = min(answer, i+j+k);
           }
        }
    }
    println!("{}", answer);


}

