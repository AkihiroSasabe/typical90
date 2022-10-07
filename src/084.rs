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
        s: Chars
    }

    // ooxo
    // Success
    // l=1, r=3
    // l=2, r=3
    // l=3, r=4
    // Failed
    // l=1, r=2

    let mut run_length = vec![];
    let mut count: usize = 1;
    for i in 0..(n-1) {
        if s[i] != s[i+1] {
            run_length.push((s[i], count, i));
            count = 1;
        }
        else  {
            count += 1;
        }
    }
    run_length.push((s[n-1], count, n-1));

    // 余事象を数える
    let mut comprementary_event_num = 0;
    for i in 0..run_length.len() {
        if run_length[i].1 != 1 {
            // 長さnの区間から2個選ぶ = nC2 = n*(n-1)/2!
            comprementary_event_num += run_length[i].1 * (run_length[i].1 - 1) / 2;
        }
    }

    let answer = n * (n-1) / 2 - comprementary_event_num;
    println!("{}", answer);

    // println!("{:?}", run_length);
    // println!("{}", run_length[0].0);

}

