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
    }

    let mut l = vec![];
    let mut r = vec![];

    for i in 0..n {
        input! {
            l_i: usize,
            r_i: usize,
        }
        l.push(l_i);
        r.push(r_i);
    }

    let mut expectation = 0.0;
    for i in 0..n {
        for j in i+1..n {
            let denominator =  (r[i] - l[i] + 1) * (r[j] - l[j] + 1);
            let mut numerator = 0;
            for x_i in l[i]..(r[i]+1) {
                for x_j in l[j]..r[j]+1 {
                    if x_i > x_j {
                        numerator += 1;
                    }
                }
            }
            expectation += numerator as f64 / denominator as f64;
        }
    }
    println!("{}", expectation);


}