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
    // solve_1st_time(n, s); // 2022-06-24
    solve_2nd_time(n, s); // 2025-10-11
}

fn solve_2nd_time(n: usize, s: Vec<char>) {
    // 2025-10-11 Run Length Encoding の関数のテストを兼ねて実装

    // 解法
    // o と x が両方含まれる
    // 全事象 - 余事象 = 全事象 - (o だけの事象) - (x だけの事象)

    // 全事象 := n + n-1 + ... + 1 = n*(n+1)/2
    let whole_event = n * (n + 1) / 2;

    let rle = run_length_encoding(&s);

    // 余事象を数える
    let mut complementary_event = 0;
    for (ch, len) in rle {
        complementary_event += len * (len + 1) / 2;
    }
    let ans = whole_event - complementary_event;
    println!("{}", ans);

}

fn solve_1st_time(n: usize, s: Vec<char>) {
    // 2022-06-24 22:27:57 初提出

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

fn run_length_encoding<T: PartialEq + Clone>(s: &Vec<T>) -> Vec<(T, usize)> {
    // 配列sを、ランレングス圧縮する関数.
    // rle := (要素, 連続長さ) の配列を返す

    let mut rle: Vec<(T, usize)> = vec![];
    let mut count: usize = 0;
    for i in 0..s.len() {
        count += 1;
        if i + 1 == s.len() || s[i] != s[i + 1] {
            rle.push((s[i].clone(), count));
            count = 0;
        }
    }
    return rle
}
