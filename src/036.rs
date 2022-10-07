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
        q: usize,
    }
    let mut x = vec![];
    let mut y = vec![];
    let mut x_rot = vec![vec![]; n];
    let mut y_rot = vec![vec![]; n];
    // [X_ROT] = [cos45  -sin45] [X] = [X*cos45 - Y*sin45] = 1/root(2)[X - Y]
    // [Y_ROT] = [sin45   cos45] [Y]   [X*sin45 + Y*cos45]            [X + Y]
    for i in 0..n {
        input! {
            x_i: isize,
            y_i: isize,
        }
        x.push(x_i);
        y.push(y_i);
        x_rot[i] = vec![x_i - y_i, i as isize];
        y_rot[i] = vec![x_i + y_i, i as isize];
    }
    let mut q_list = vec![];
    for i in 0..q {
        input! {
            q_i: usize,
        }
        q_list.push(q_i - 1);
    }
    x_rot.sort();
    y_rot.sort();

    let max_x_0 = x_rot[0][1] as usize;
    let max_x_n = x_rot[n-1][1] as usize;
    let max_y_0 = y_rot[0][1] as usize;
    let max_y_n = y_rot[n-1][1] as usize;
    let max_candidate = vec![max_x_0, max_x_n, max_y_0, max_y_n];

    for i in q_list.iter() {
        let mut distance = 0;
        for j in max_candidate.iter() {
            distance = max(distance, (x[*i] - x[*j]).abs() + (y[*i] - y[*j]).abs());
        }
        println!("{}", distance);
    }

}