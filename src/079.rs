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
        mut a: [[isize; w]; h],
        mut b: [[isize; w]; h],
    }
    let mut count = 0;
    for i in 0..h {
        for j in 0..w {

            let diff = b[i][j] - a[i][j];
            if i == h-1 && diff != 0 {
                println!("No");
                return;
            }
            if j == w-1 && diff != 0 {
                println!("No");
                return;
            }
            if diff != 0 {
                count += diff.abs();
                a[i][j] += diff;
                a[i][j+1] += diff;
                a[i+1][j] += diff;
                a[i+1][j+1] += diff;
            }
            
        }
    }
    println!("Yes");
    println!("{}", count);

}