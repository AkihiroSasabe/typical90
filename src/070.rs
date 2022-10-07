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
    let mut x = vec![];
    let mut y = vec![];
    for i in 0..n {
        input! {
            x_i: isize,
            y_i: isize,
        }
        x.push(x_i);
        y.push(y_i);
    }

    x.sort();
    y.sort();
    // let x_index = n / 2 - 1 +  
    let xc = x[n/2];
    let yc = y[n/2];

    // println!("xc, yc: {} {}", xc, yc);

    let mut distance = 0;
    for i in 0..n {
        distance += (xc  - x[i]).abs() + (yc - y[i]).abs();
    }
    
    println!("{}", distance);


}