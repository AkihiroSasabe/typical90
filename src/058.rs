use proconio::input;
use itertools::Itertools;
use std::cmp::{max, min};
use std::cmp::Ordering;
use std::collections::VecDeque;
use std::collections::BinaryHeap;
use proconio::marker::Chars;
use std::collections::HashMap;

fn main() {
    input! {
        mut n: usize,
        k: usize
    }

    // 10^5 - 1 より小さい
    // 10^5 - 1 = 99_999
    let mut x = n.clone();
    let mut y = 0;
    let mut z = 0;
    let mut seen = vec![false; 100_000];
    let mut hash_map = HashMap::new(); // (数字x, 初登場時のイテレーション番号)
    let mut cycle_list = vec![]; // 初登場順に数字xが挿入される。
    let mut second_time = 0; // 2回目に同じxが登場したときのイテレーション番号

    for i in 0..k {
        y = 0;
        z = 0; 
        z += x;
        for _ in 0..6 {
            y += x % 10;
            x /= 10;
        }
        // 99_999 + 9 + 9 + 9 + 9 + 9 = -1 + 45 = 44;

        z = (z + y) % 100_000;
        x = z.clone();
        // println!("y: {}, x: {}", y, x);
        if seen[x] {
            // println!("seen!");
            break
        }
        seen[x] = true;
        hash_map.insert(x, i);
        cycle_list.push(x);
        second_time += 1;
    }

    // 0 1 2 3
    let first_time = hash_map[&x]; 
    // println!("1st: {}", first_time);      // 初回登場時
    // println!("2nd: {}", second_time);       // 2回目登場時

    let period = second_time - first_time;
    // println!("period: {}", period);    // 周期
    // println!("{}",  cycle_list[first_time]);
    let answer = cycle_list[first_time + (k - 1 - first_time) % period];

    println!("{}", answer);
    

}