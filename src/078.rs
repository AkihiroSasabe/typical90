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
        m: usize,
    }

    let mut graph = vec![vec![]; n];

    for _ in 0..m {
        input! {
            a: usize,
            b: usize
        }
        graph[a-1].push(b-1);
        graph[b-1].push(a-1);
    }
    
    let mut answer = 0;
    for i in 0..n {
        let mut over_i_num = 0;
        for j in graph[i].iter() {
            if i > *j {
                over_i_num += 1;
            }
        }
        if over_i_num == 1 {
            answer += 1;
        }
    }
    println!("{}", answer);
}

