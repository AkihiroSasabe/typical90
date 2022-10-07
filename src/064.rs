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
        mut a: [isize; n],
    }
    let mut l = vec![];
    let mut r = vec![];
    let mut v = vec![];

    for i in 0..q {
        input! {
            l_i: usize,
            r_i: usize,
            v_i: isize
        }
        l.push(l_i);
        r.push(r_i);
        v.push(v_i);
    }

    // let c = a[1-1] + 1;
    // println!("{}", c);

    // 不便さeの初期化
    let mut e_sum = 0;
    let mut e = vec![];
    for i in 0..(n-1) {
        let diff = a[i+1] - a[i];
        e.push(diff);
        e_sum += diff.abs();
    }

    for i in 0..q {
        // 変化後 - 変化前
        if l[i] != 1 {
            let diff = e[l[i] -2] + v[i];
            e_sum += diff.abs() - e[l[i] -2].abs();
            e[l[i] -2] = diff;
        }
        if r[i] != n {
            let diff = e[r[i] - 1] - v[i];
            e_sum += diff.abs() - e[r[i] - 1].abs();
            e[r[i] - 1] = diff;
        }
        println!("{}", e_sum);
    }

}