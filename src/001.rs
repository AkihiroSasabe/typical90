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
        l: usize,
        k: usize,
        a: [usize; n]
    }

    // N+1個の羊羹の各長さ
    let mut length_list = vec![];
    length_list.push(a[0]);
    for i in 1..n {
        length_list.push(a[i] - a[i-1]);
    }
    length_list.push(l-a[n-1]);

    // 3 34
    // 1
    // 8 13 26
    // 8 5 13 8

    // 7 45
    // 2
    // 7 11 16 20 28 34 38
    // 7 4 5 |4 8| 6 4 

    // めぐる式2分探索2分探索 (羊羹の最低長さがmid以上になる)
    let mut low = 0;
    let mut high = l;
    while high - low > 1 {
        let mut mid = (low + high) / 2;
        let mut cut_count = 0;
        let mut cut_length = 0;
        // 羊羹を左端の切れ目からmid以上になるまで切っていく。
        for i in 0..(n+1) {
            cut_length += length_list[i];
            if cut_length >= mid {
                cut_count += 1;
                cut_length = 0;
            }
        }
        // 最低長さがmid以上が実現可能だったとき
        if cut_count >= k+1 {
            low = mid;
        }
        // 最低長さがmid以上が実現不可能だったとき
        else {
            high = mid;
        }
    }
    // 条件をギリギリ満たす最大の値を知りたいのでlowを返す
    println!("{}", low);


}