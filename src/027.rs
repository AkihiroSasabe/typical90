use proconio::input;
use itertools::Itertools;
use std::cmp::{max, min};
use std::cmp::Ordering;
use std::collections::VecDeque;
use std::collections::BinaryHeap;
use std::collections::HashMap;
use proconio::marker::Chars;

fn main() {
    input! {
        n: usize,
        s: [String; n]
    }

    let mut hash_map = HashMap::new();
    for i in 0..n {
        if hash_map.contains_key(&s[i]) {
            continue
        }
        hash_map.insert(&s[i], i);
        println!("{}", i+1);
    }
}