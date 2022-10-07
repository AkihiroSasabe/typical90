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
        mut a: [usize; n],
        mut b: [usize; n],
        mut c: [usize; n],
    }

    let mut amari_num = vec![vec![0; 46]; 3];
    // 46の倍数
    for i in 0..n {
        a[i] = a[i] % 46;
        b[i] = b[i] % 46;
        c[i] = c[i] % 46;
        amari_num[0][a[i]] += 1;
        amari_num[1][b[i]] += 1;
        amari_num[2][c[i]] += 1;
    }

    let mut ans: usize = 0;
    for i in 0..46 {
        for j in 0..46 {
            for k in 0..46 {
                let amari = (i+j+k) % 46;
                if amari == 0 {
                    ans += amari_num[0][i] * amari_num[1][j] * amari_num[2][k];
                }
            }
        }
    }
    println!("{}", ans);


}