use proconio::input;
use itertools::Itertools;
use std::cmp::{max, min};
use std::cmp::Ordering;
use std::collections::VecDeque;
use std::collections::BinaryHeap;
use proconio::marker::Chars;
fn main() {
    input! {
        h: usize,
        w: usize,
        a: [[usize; w]; h]
    }

    let mut colmun_sum = vec![0; w];
    let mut row_sum = vec![0; h];
    for i in 0..h {
        for j in 0..w{
            colmun_sum[j] += a[i][j];
            row_sum[i] += a[i][j];
        }
    }

    let mut answer = vec![vec![0; w]; h];
    for i in 0..h {
        for j in 0..w{
            answer[i][j] = colmun_sum[j] + row_sum[i] - a[i][j];
            print!("{} ", answer[i][j]);
        }
        println!("");
    }
    // println!("{:?}", a);
}