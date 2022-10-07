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
    // 考察
    // 一番左は(である必要がある。
    // 一番右は}である必要がある。
    // 一番右にある(を探す。それの対は、その(から一番近くにいる)である。

    
    // 0:(, 1:)とする
    for bits in 0..(1 << n) {
        // '('の対となる')'を既に見つけているか?
        let mut unused = vec![true; n];
        // bits内の全ての'('と')'のペアが存在するか?
        let mut all_match_flag = true;
        // n桁中、何桁の'('と')'のペアを見つけたか。
        let mut match_count = 0;
        for j in 0..n {
            // 右から順に"("を探す
            if !((bits & 1 << j) != 0) {
                // j番目の'('の対となる')'があるか?
                let mut match_flag = false;
                // その"("から最も近い")"を探す
                for k in 1..(j+1) {
                    if unused[j-k] && (bits & 1 << j-k) != 0 {
                        unused[j-k] = false;
                        match_flag = true;
                        match_count += 2;
                        break
                    }
                }
                if !match_flag {
                    all_match_flag = false;
                    break;
                }
            }
        }
        if all_match_flag && match_count == n {
            bits_to_brackets(bits, n);
        }
    }

}

fn bits_to_brackets(bits: usize, n:usize) {
    let mut answer_i = String::from("");
    for i in 0..n {
        if ((bits & 1 << i) != 0) {
            answer_i = format!("){}", answer_i);
        }
        else {
            answer_i = format!("({}", answer_i);
        }
    }
    println!("{}", answer_i);
}

// debug用
fn xth_digit(bits: usize, x: usize) {
    if bits & (1 << x) != 0 {
        println!("{}th is 1", x);
    }
    else {
        println!("{}th is 0", x);
    }
}