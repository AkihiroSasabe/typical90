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
        cp: [[usize; 2]; n],
        q: usize,
        lr: [[usize; 2]; q]
    }

    // 累積和
    let mut cumulative_sum1 = vec![0;n];
    let mut cumulative_sum2 = vec![0;n];

    // 初期化
    if cp[0][0] == 1 {
        cumulative_sum1[0] = cp[0][1];
    }
    else {
        cumulative_sum2[0] = cp[0][1];
    }


    for i in 1..n {
        if cp[i][0] == 1 {
            cumulative_sum1[i] = cumulative_sum1[i-1] + cp[i][1];
            cumulative_sum2[i] = cumulative_sum2[i-1];    
        }
        else {
            cumulative_sum1[i] = cumulative_sum1[i-1];
            cumulative_sum2[i] = cumulative_sum2[i-1] + cp[i][1];
        }
    }
    // println!("{:?}", cumulative_sum1);
    // println!("{:?}", cumulative_sum2);

    for i in 0..q {
        let l = lr[i][0] - 1;
        let r = lr[i][1] - 1;

        let a: usize;
        let b: usize;

        if l != 0 {
            a = cumulative_sum1[r] - cumulative_sum1[l-1];
            b = cumulative_sum2[r] - cumulative_sum2[l-1];
        }
        else {
            a = cumulative_sum1[r];
            b = cumulative_sum2[r];
        }

        println!("{} {}", a, b);
    }
}