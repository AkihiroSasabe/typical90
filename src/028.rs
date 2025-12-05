use proconio::input;
use itertools::Itertools;
use std::cmp::{max, min};
use std::cmp::Ordering;
use std::collections::VecDeque;
use std::collections::BinaryHeap;
use proconio::marker::Chars;
fn main() {
    input! {
        n: usize
    }
    let mut x_mins = vec![];
    let mut y_mins = vec![];
    let mut x_maxs = vec![];
    let mut y_maxs = vec![];
    for i in 0..n {
        input! {
            lx_i: usize,
            ly_i: usize,
            rx_i: usize,
            ry_i: usize,
        }
        x_mins.push(lx_i);
        y_mins.push(ly_i);
        x_maxs.push(rx_i-1);
        y_maxs.push(ry_i-1);
    }
/*  
// ◆いもす法
// 複数領域の重なり度合いを調べるのに便利な手法。
// 区間の始点に+1、区間の終点の一個外に-1で初期化し、累積和を取る。
// ◆二次元いもす法
// 2次元マップ上の各マス毎に、N個の長方形が何個重なっているかをO(N+HW)で計算する手法。
// 二次元いもす法は、以下4ステップで処理する。

// [step1] 0で初期化
let mut imos: Vec<Vec<isize>> = vec![vec![0; w+1]; h+1]; // 累積和を取るときに、実際のマップの幅より1大きいと実装が楽。

// [step2] 4つ角に±1を加算
for i in 0..n {
    let y_min = y_mins[i];
    let y_max = y_maxs[i];
    let x_min = x_mins[i];
    let x_max = x_maxs[i];
    
    imos[y_min][x_min] += 1; // 長方形の左上(領域内)の1マスに+1
    imos[y_min][x_max+1] -= 1; // 長方形の右上(領域内)の右(領域外)の1マスに-1
    imos[y_max+1][x_min] -= 1; // 長方形の左下(領域内)の下(領域外)の1マスに-1
    imos[y_max+1][x_max+1] += 1; // 長方形の右下(領域内)の右下(領域外)の1マスに+1
}

// [step3] 横方向に累積和を取る
for i in 0..h {
    for j in 0..(w-1) {
        imos[i][j+1] += imos[i][j];
    }
}

// [step4] 縦方向に累積和を取る
for i in 0..(h-1) {
    for j in 0..w {
        imos[i+1][j] += imos[i][j];
    }
}

// 例 
// (H,W)=(4,3)のマップで
// (y_min, x_min, y_max, x_max) = (0, 0, 2, 1) の長方形が1個ある場合

// [step1] 0でマップ全体を初期化
// [  0,  0,  0]
// [  0,  0,  0]
// [  0,  0,  0]
// [  0,  0,  0]

// [step2] 4つ角に±1を加算
// [ +1,  0, -1]
// [  0,  0,  0]
// [  0,  0,  0]
// [ -1,  0, +1] 

// [step3] 横方向に累積和を取る
// [ +1, +1,  0]
// [  0,  0,  0]
// [  0,  0,  0]
// [ -1, -1,  0] 

// [step4] 縦方向に累積和を取る
// (y_min, x_min, y_max, x_max) = (0, 0, 2, 1) で与えられる長方形領域の重なり度合いが得られる。
// [  1,  1,  0]
// [  1,  1,  0]
// [  1,  1,  0]
// [  0,  0,  0]
*/

    // マップ全体の大きさ
    let h = 1001;
    let w = 1001;
    // let h = 5;
    // let w = 5;

    // [step1] 0で初期化
    let mut imos: Vec<Vec<isize>> = vec![vec![0; w+1]; h+1]; // 累積和を取るときに、実際のマップの幅より1大きいと実装が楽。
    // いもす法
    // [step2] N個の長方形の4つ角に、それぞれ±1を加算
    for i in 0..n {
        let y_min = y_mins[i];
        let y_max = y_maxs[i];
        let x_min = x_mins[i];
        let x_max = x_maxs[i];
        
        imos[y_min][x_min] += 1; // 長方形の左上(領域内)の1マスに+1
        imos[y_min][x_max+1] -= 1; // 長方形の右上(領域内)の右(領域外)の1マスに-1
        imos[y_max+1][x_min] -= 1; // 長方形の左下(領域内)の下(領域外)の1マスに-1
        imos[y_max+1][x_max+1] += 1; // 長方形の右下(領域内)の右下(領域外)の1マスに+1
    }
    // [step3] 横方向に累積和を取る
    for i in 0..h {
        for j in 0..(w-1) {
            imos[i][j+1] += imos[i][j];
        }
    }
    // [step4] 縦方向に累積和を取る
    for i in 0..(h-1) {
        for j in 0..w {
            imos[i+1][j] += imos[i][j];
        }
    }
    // 重なり度合いのデバッグ
    // for i in 0..h {
    //     for j in 0..w {
    //         print!("{} ", imos[i][j]);
    //     }
    //     println!();
    // }

    //overlap_counts[k] := k枚の長方形に覆われているマスの数
    let mut overlap_counts = vec![0; n+1];
    for i in 0..h {
        for j in 0..w {
            let overlap = imos[i][j] as usize;
            overlap_counts[overlap] += 1;
        }
    }
    for i in 1..n+1 {
        println!("{}", overlap_counts[i]);
    }
}
