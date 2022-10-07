use proconio::input;
use itertools::Itertools;
use std::cmp::{max, min};
use std::cmp::Ordering;
use std::collections::VecDeque;
use std::collections::BinaryHeap;
use proconio::marker::Chars;
fn main() {
    input! {
        q: usize,
        tx: [[usize; 2]; q]
    }
    // リングバッファ(Vecは末尾の追加/取り出しが速いため、スタックに向いているが、Dequeは先頭の追加/取り出しが早くキューにもなれる)
    let mut deque = VecDeque::new();
    // let mut deque = vec![]; // Vecでも実はACできる(もちろん実行時間は伸びる。subtask_1_1.txtで25msec -> 655msec)
    let mut paper = vec![];
    for i in tx {
        match i[0] {
            1 => deque.push_front(i[1]),    // 一番上に追加
            2 => deque.push_back(i[1]),     // 一番下に追加
            // 1 => deque.insert(0,i[1]),    // 一番上に追加(Vecでも実はACできる)
            // 2 => deque.push(i[1]),        // 一番下に追加(Vecでも実はACできる)
            3 => paper.push(deque[i[1]-1]), // 山札の上からx_i番目のカード
            _ => (),                        // 上記以外のときは`_`で示す。`()`は何もしない。
        }
    }
    for i in paper {
        println!("{}", i);
    }
}