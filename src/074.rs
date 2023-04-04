use proconio::input;
use itertools::Itertools;
use std::cmp::{max, min};
use std::cmp::Ordering;
use std::collections::VecDeque;
use std::collections::BinaryHeap;
use std::hash::Hash;
use proconio::marker::Chars;
use std::collections::HashMap;

fn main() {
    // 2023-04-02 20:30
    input! {
        n: usize,
        s: Chars
    }

    let mut s_num = vec![3; n];
    for i in 0..n {
        s_num[i] = 
        if s[i] == 'a' {0}
        else if s[i] == 'b' {1}
        else if s[i] == 'c' {2}
        else {3}
    }

    // usizeにしないと、i32がアサインされてオーバーフローするので注意！
    let mut ans: usize = 0;
    for i in 0..n {
        ans += s_num[i] * (1 << i);
    }
    println!("{}", ans);


    // 考察用にN=4のSで最大操作回数がどうなるか実験
    // experiment();

    // 下の結果を見れば、法則性 (ans = Σ[i=0->i=n-1] s_num[i] * 2^i) に気づける
    // 結果
    // s=[0, 0, 0, 0], potential = 0
    // s=[1, 0, 0, 0], potential = 1
    // s=[2, 0, 0, 0], potential = 2
    // V==== ===== ===== ====V
    // s=[0, 1, 0, 0], potential = 2
    // s=[1, 1, 0, 0], potential = 3
    // s=[2, 1, 0, 0], potential = 4
    // V==== ===== ===== ====V
    // s=[0, 2, 0, 0], potential = 4
    // s=[1, 2, 0, 0], potential = 5
    // s=[2, 2, 0, 0], potential = 6
    // V==== ===== ===== ====V
    // s=[0, 0, 1, 0], potential = 4
    // s=[1, 0, 1, 0], potential = 5
    // s=[2, 0, 1, 0], potential = 6
    // s=[0, 1, 1, 0], potential = 6
    // s=[1, 1, 1, 0], potential = 7
    // s=[2, 1, 1, 0], potential = 8
    // s=[0, 2, 1, 0], potential = 8
    // s=[1, 2, 1, 0], potential = 9
    // s=[2, 2, 1, 0], potential = 10
    // V==== ===== ===== ====V
    // s=[0, 0, 2, 0], potential = 8
    // s=[1, 0, 2, 0], potential = 9
    // s=[2, 0, 2, 0], potential = 10
    // s=[0, 1, 2, 0], potential = 10
    // s=[1, 1, 2, 0], potential = 11
    // s=[2, 1, 2, 0], potential = 12
    // s=[0, 2, 2, 0], potential = 12
    // s=[1, 2, 2, 0], potential = 13
    // s=[2, 2, 2, 0], potential = 14
    // V==== ===== ===== ====V
    // s=[0, 0, 0, 1], potential = 8
    // s=[1, 0, 0, 1], potential = 9
    // s=[2, 0, 0, 1], potential = 10
    // s=[0, 1, 0, 1], potential = 10
    // s=[1, 1, 0, 1], potential = 11
    // s=[2, 1, 0, 1], potential = 12
    // s=[0, 2, 0, 1], potential = 12
    // s=[1, 2, 0, 1], potential = 13
    // s=[2, 2, 0, 1], potential = 14
    // s=[0, 0, 1, 1], potential = 12
    // s=[1, 0, 1, 1], potential = 13
    // s=[2, 0, 1, 1], potential = 14
    // s=[0, 1, 1, 1], potential = 14
    // s=[1, 1, 1, 1], potential = 15
    // s=[2, 1, 1, 1], potential = 16
    // s=[0, 2, 1, 1], potential = 16
    // s=[1, 2, 1, 1], potential = 17
    // s=[2, 2, 1, 1], potential = 18
    // s=[0, 0, 2, 1], potential = 16
    // s=[1, 0, 2, 1], potential = 17
    // s=[2, 0, 2, 1], potential = 18
    // s=[0, 1, 2, 1], potential = 18
    // s=[1, 1, 2, 1], potential = 19
    // s=[2, 1, 2, 1], potential = 20
    // s=[0, 2, 2, 1], potential = 20
    // s=[1, 2, 2, 1], potential = 21
    // s=[2, 2, 2, 1], potential = 22
    // V==== ===== ===== ====V
    // s=[0, 0, 0, 2], potential = 16
    // s=[1, 0, 0, 2], potential = 17
    // s=[2, 0, 0, 2], potential = 18
    // s=[0, 1, 0, 2], potential = 18
    // s=[1, 1, 0, 2], potential = 19
    // s=[2, 1, 0, 2], potential = 20
    // s=[0, 2, 0, 2], potential = 20
    // s=[1, 2, 0, 2], potential = 21
    // s=[2, 2, 0, 2], potential = 22
    // s=[0, 0, 1, 2], potential = 20
    // s=[1, 0, 1, 2], potential = 21
    // s=[2, 0, 1, 2], potential = 22
    // s=[0, 1, 1, 2], potential = 22
    // s=[1, 1, 1, 2], potential = 23
    // s=[2, 1, 1, 2], potential = 24
    // s=[0, 2, 1, 2], potential = 24
    // s=[1, 2, 1, 2], potential = 25
    // s=[2, 2, 1, 2], potential = 26
    // s=[0, 0, 2, 2], potential = 24
    // s=[1, 0, 2, 2], potential = 25
    // s=[2, 0, 2, 2], potential = 26
    // s=[0, 1, 2, 2], potential = 26
    // s=[1, 1, 2, 2], potential = 27
    // s=[2, 1, 2, 2], potential = 28
    // s=[0, 2, 2, 2], potential = 28
    // s=[1, 2, 2, 2], potential = 29
    // s=[2, 2, 2, 2], potential = 30

    // 実験
    // 000: 0回
    // 100: 1回
    // 200: 2回
    
    // 010: 2回 
    // 100
    // 000

    // 001: 4回
    // 110
    // ...



}

// Sの状態stateに対して、左からtarget文字目に操作を行って新しい状態を返す関数: 
// S i ​ = b である i (1≤i≤N) を 1 つ選び、 S i ​ を a に変更した後、 S 1 ​ ,S 2 ​ ,⋯,S i−1 ​ を変化させる。
// S i ​ = c である i (1≤i≤N) を 1 つ選び、 S i ​ を b に変更した後、 S 1 ​ ,S 2 ​ ,⋯,S i−1 ​ を変化させる。
fn change(mut state: Vec<usize>, target: usize, modulo: usize) -> Vec<usize> {
    state[target] = (state[target] + modulo - 1) % modulo;
    for i in 0..target {
        state[i] = (state[i] + 1) % modulo;
    }
    return state
}

// 任意のSについて、最大操作回数を出力する関数
// 既に計算済みのSになるまで変形を再帰的に繰り返す
fn get_state_num(hash: &mut HashMap<Vec<usize>, usize>, state: Vec<usize>, modulo: usize) -> usize {
    // 既に計算済みの場合は、それを返す
    if hash.contains_key(&state) {
        return *(hash.get(&state).unwrap())
    }

    let mut state_num = 0;

    // 各桁について操作をしていく
    for i in 0..state.len() {
        // その桁が'a'なら操作は出来ないのでパス
        if state[i] != 0 {
            let new_state = change(state.clone(), i, modulo);
            let new_state_num = get_state_num(hash, new_state, modulo);
            state_num = max(state_num, new_state_num + 1);
        }
    }

    // 計算が終わったら結果を登録
    hash.insert(state, state_num);
    return state_num
}

// 考察用の実験
fn experiment() {
    // N=4のとき、Sの全状態に対して操作の最大回数をprintする関数

    // key: Sのある状態, val: 操作の最大回数
    let mut hash: HashMap<Vec<usize>, usize> = HashMap::new();

    // hashの初期化. Sが全部'a'の時の状態(S="aaaa")は、最大回数が0.
    let s_all_a = vec![0,0,0,0];
    hash.insert(s_all_a, 0);

    // 特に注目したい状態
    let p1 = vec![0,1,0,0];
    let p2 = vec![0,2,0,0];
    let p3 = vec![0,0,1,0];
    let p4 = vec![0,0,2,0];
    let p5 = vec![0,0,0,1];
    let p6 = vec![0,0,0,2];

    let modulo = 3;
    let mut state = vec![0,0,0,0];
    for s3 in 0..3 {
        for s2 in 0..3 {
            for s1 in 0..3 {
                for s0 in 0..3 {
                    state[0] = s0;
                    state[1] = s1;
                    state[2] = s2;
                    state[3] = s3;
                    if state == p1 || state == p2 || state == p3 || state == p4 || state == p5 || state == p6 {
                        println!("V==== ===== ===== ====V");
                    }
                    let state_num = get_state_num(&mut hash, state.clone(), modulo);
                    println!("s={:?}, potential = {}", state, state_num);
                }
            }
        }
    }
}


