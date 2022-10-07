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
        n: usize,
        k: usize,
        a: [usize; n]
    }

    // 尺取法（しゃくとり法）
    // https://github.com/E869120/kyopro_educational_90/blob/main/sol/034.cpp のように、スタート側をforループで回した法が実装が綺麗かも
    let mut s = 0;
    let mut e = 0;
    let mut kind_num = 1;
    // let mut freqeuncy: Vec<usize> = vec![0; 1_000_000_001]; // len=10^8は良いが、10^9だとメモリがデカくてREする
    let mut freqeuncy = HashMap::new(); // 可変長配列だとメモリがオーバーするのでHashMapでいく。
    freqeuncy.insert(a[e], 1);
    // freqeuncy[a[e]] += 1;
    let mut max_length = 1;
    let mut length = 1;
    while e < n-1 {
        // 末尾
        loop {
            e += 1;
            if e >= n {break}
            length += 1;
            // 含まれる要素数の確認
            if !freqeuncy.contains_key(&a[e]) {
            // if freqeuncy[a[e]] == 0 {
                kind_num += 1;
                freqeuncy.insert(a[e], 0);
            }
            *freqeuncy.get_mut(&a[e]).unwrap() += 1;
            // freqeuncy[a[e]] += 1;
            // k種類よりも大きくなったら一旦endの探索は終わり。
            if kind_num > k {
                break
            }
            // println!("e:{} s:{}, max_length:{}, length:{}", e, s, max_length, length);
            max_length = max(max_length, length);
        }
        // 先頭
        loop {
            s += 1;
            length -= 1;
            *freqeuncy.get_mut(&a[s-1]).unwrap() -= 1;
            // freqeuncy[a[s-1]] -= 1;
            // println!("e:{} s:{}, max_length:{}, length:{}", e, s, max_length, length);
            // 先頭を詰めたことで1種類失った場合
            if freqeuncy[&a[s-1]] == 0 {
            // if freqeuncy[a[s-1]] == 0 {
                kind_num -= 1;
                freqeuncy.remove(&a[s-1]);
                break
            }
        }
    }
    println!("{}", max_length);

}