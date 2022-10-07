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
        w: [usize; n],
        b: [usize; n]
    }

    // スプレイグ・グランディの定理を使って解く。解説は下記がわかりやすい。
    // https://algo-logic.info/combinatorial-games/
    // // Grundy数
    // 定義: 
    // 終了状態 Pf での Grundy 数 = 0
    // ある状態 P での Grundy数 = mex(「Pから到達可能な状態 P’ のGrundy数」の集合 )
    // g(wi,bi) = mex({wi,biから到達可能な状態(wi_next, bi_next)のg(wi_next, bi_next)を集めた集合}). 
    // mex関数は、集合を受け取って、集合に含まれない最も小さい非負整数を返す関数.
    // g(0,0) = mex({})=0
    // g(0,1) = mex({})=0
    // g(0,2) = mex({g(0,1)})=mex({0})=1
    // g(0,3) = mec({g(0,2)})=mex({1})=0
    let INF = 1 << 60;
    let max_len = 51 * 51;
    let mut xor_sum = 0;
    let wi_bi_max = 51*51;
    let mut grundy_memo = vec![vec![INF; wi_bi_max]; wi_bi_max];
    // (1)不偏ゲームを(n個の)部分不偏ゲームに分解
    for i in 0..n {
        // (2)それぞれの部分ゲームに対して、その状況での Grundy数を計算
        let grundy_num = get_grundy(w[i], b[i], &mut grundy_memo, max_len);
        // (3)計算したGrundy数全てに対しての XOR を取る
        // "^"はXOR(排他的論理和)を表す。0+0=0, 0+1=1, 1+0=1, 1+1=0。繰り上がりの無い足し算。
        xor_sum ^= grundy_num;
        // println!("grundy_num:{} xor_sum:{}", grundy_num, xor_sum);
    }
    // (4)XOR sum の値が、non-zero のとき先手必勝となり、0のとき後手必勝となる
    if xor_sum == 0 {
        println!("Second");
    }
    else {
        println!("First");
    }

}

fn get_grundy(w_i: usize, b_i: usize, grundy_memo: &mut Vec<Vec<usize>>, max_len: usize) -> usize {
    let mut set = HashMap::new();
    let INF = 1 << 60;

    if w_i ==0 && b_i <= 1 {
        grundy_memo[w_i][b_i] = 0;
        return 0;
    }
    else {
        if w_i >= 1 {
            let next_b_i = b_i + w_i;
            let next_w_i = w_i - 1;
            // println!("w_i, b_i, next_w_i, next_b_i: {} {} {} {}", w_i, b_i, next_w_i, next_b_i);
            let mut grungy_num = grundy_memo[next_w_i][next_b_i];
            if grungy_num == INF  {
                grungy_num = get_grundy(next_w_i, next_b_i, grundy_memo, max_len);
            }
            set.insert(grungy_num, 1);
        }
        if b_i >= 2 {
            for k in 1..(b_i/2 + 1) {
                let next_b_i = b_i - k;
                let next_w_i = w_i;
                let mut grungy_num = grundy_memo[next_w_i][next_b_i];
                if grungy_num == INF  {
                    grungy_num = get_grundy(next_w_i, next_b_i, grundy_memo, max_len);
                }
                set.insert(grungy_num, 1);
            }
        }
    }
    // println!("set: {:?}", set);
    let grungy_num = mex(&set, max_len);
    grundy_memo[w_i][b_i] = grungy_num;
    return grungy_num
}

// 最小除外数: Minimum excludent(集合に含まれない最小の非負整数を返す)
fn mex(set: &HashMap<usize, usize>, max_len: usize) -> usize {
    let mut answer = 0;
    for i in 0..max_len {
        if !set.contains_key(&i) {
            answer = i;
            break
        }
    }
    answer
}