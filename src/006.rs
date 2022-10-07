use proconio::input;
use itertools::Itertools;
use std::cmp::{max, min};
use std::cmp::Ordering;
use std::collections::VecDeque;
use std::collections::BinaryHeap;
use proconio::marker::Chars;

// 19:39
fn main() {
    input! {
        n: usize,
        k: usize,
        s: Chars
    }

    let mut answers = vec!['z'; k];
    let lowercase: Vec<char> = "abcdefghijklmnopqrstuvwxyz".chars().collect();


    // dp[alpabet][s_i] = alpabetがs_i移行で最も左にあるもの。
    //     a   t   c   o   d   e   r
    // a   0   -   -   -   -   -   -
    // b   -   -   -   -   -   -   - 
    // c   2   2   2   -   -   -   -
    // d   4   4   4   4   4   -   -
    // e   5   5   5   5   5   5   -

    let mut first_char_index_map = vec![vec![n; n]; 26];
    for i in 0..26 {
        let mut current_index = n;
        for j in 0..n {
            if s[n-1-j] == lowercase[i] {
                current_index = n-1-j;
            }
            first_char_index_map[i][n-1-j] = current_index;
        }
    }
    // for i in 0..26 {
    //     println!("{:?}", first_char_index_map[i]);
    // }

    let mut answers = vec![];
    let mut parts_index = 0;
    let mut s_index = 0;
    let mut break_flag = false;

    while s_index < n {
        for i in 0..26 {
            // println!("{}, first_char_index_map[i][s_index]: {}", i, first_char_index_map[i][s_index]);
            if first_char_index_map[i][s_index] == n {
                continue
            }
            
            // 挿入出来る条件
            if k - parts_index <= n - first_char_index_map[i][s_index] {
                // println!("push! {} {}", s_index, s[first_char_index_map[i][s_index]]);
                answers.push(s[first_char_index_map[i][s_index]]);
                parts_index += 1;
                s_index = first_char_index_map[i][s_index] + 1;
                if parts_index == k {
                    break_flag = true;
                    break
                }
                break
            }
        }

        if break_flag {break}
    }
    // println!("{:?}", answers);

    // // 尺取法的なノリで解く。confirmed_indexが虫の尻。nが虫の頭。
    // // 現在、確定しようとしている部分文字列のインデックス
    // let mut parts_index = 0;
    // // 文字列Sの中から調査するインデックスの先頭
    // let mut confirmed_index = 0;
    // while parts_index != k {
    //     // println!("parts_index: {}", parts_index);
    //     for i in confirmed_index..n {
    //         // その文字を入れられるか?
    //         if n - i < k - parts_index {continue}
    //         if s[i] < answers[parts_index] {
    //             answers[parts_index] = s[i];
    //             confirmed_index = i+1;
    //         }
    //     }
    //     // println!("{}", answers[parts_index]);
    //     parts_index += 1;
    // }
    // // println!("{:?}", answers);

    for i in 0..k {
        print!("{}", answers[i]);
    }

}
