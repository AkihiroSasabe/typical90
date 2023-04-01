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
        m: usize,
    }
    let mut t = vec![];
    let mut a = vec![];
    for i in 0..n {
        input! {
            ti: usize,
            ai: [usize; ti],
        }
        t.push(ti);
        a.push(ai);
    }        
    input! {
        s: [usize; m]
    }
    let MODULO = 998_244_353;

    // 掃き出し法
    // 111111 ---(1)
    // 110111 ---(2)
    // V
    // 111111 ---(1)
    // 001000 ---(3) = (1) XOR (2)
    // V
    // 110111 ---(4) = (1) XOR (3)
    // 001000 ---(3)

    // n個のスイッチ, m個のパネル
    // a[i]をスイッチiが返す、パネルの表裏を表すbitに変換
    let mut bits = vec![];
    for i in 0..n {
        let mut bit = vec![0; m];
        for j in 0..a[i].len() {
            bit[a[i][j] - 1] = 1;
        }
        // println!("switch: {}, bits: {:06b}", i, bit);
        bits.push(bit);
    }

    // 掃き出し法 (各列が、1行を除いて全部0になるようにしていく)
    let mut get_switch_from_panel = vec![n; m]; // パネルiに対応するスイッチは get_switch_from_panel[i]. nなら無し
    let mut top_index = 0; // 次に掃き出しをするスイッチのインデックス
    // 各パネル(列)のループ
    for panel_i in 0..m {
        // println!("panel_i: {}", panel_i);

        // その列の1番上にある表(=1)のスイッチを探す(初期値n)
        let mut top_index_seen = false;
        // 各スイッチ(行)のループ
        for switch_j in top_index..n {
            // println!("before -> switch_j: {}, bits: {:06b}", switch_j, bits[switch_j]);

            // panel_iで1番上にある表のスイッチか判定
            if bits[switch_j][panel_i] == 1 {
                get_switch_from_panel[panel_i] = top_index;
                // 行を入れ替える
                // println!("swap {} <-> {}", switch_j, top_index);
                bits.swap(switch_j, top_index);
                top_index_seen = true;
                top_index += 1;
                break
            }
            // println!("after  -> switch_j: {}, bits: {:06b}", switch_j, bits[switch_j]);
        }
        if top_index_seen {
            // 自分以外の全スイッチのpanel_iの位置が0になるように掃き出す. 
            for switch_j in 0..n {
                if switch_j == top_index-1 {continue}
                // XORする
                if bits[switch_j][panel_i] == 1 {
                    for panel_temp in 0..m {
                        bits[switch_j][panel_temp] ^= bits[top_index-1][panel_temp];
                    }
                }
            }
        }
        
        // // debug
        // for switch_j in 0..n {
        //     println!("switch: {}, bits: {:06b}", switch_j, bits[switch_j]);
        // }
    }
    // println!("get_switch_from_panel: {:?}", get_switch_from_panel);

    // 貪欲に希望通りのパネルに近づけていく
    let mut ans = 1;
    let mut current_bit = vec![0; m];
    for panel_i in 0..m {
        // 希望のビットと現在のビットのpanel_iビット目が異なる場合、対応するスイッチを押す
        if s[panel_i] != current_bit[panel_i] {
            // panel_iビット目を返せるスイッチを取得
            let switch_j = get_switch_from_panel[panel_i];
            // 返せるビットがないときは、希望の押し方無し
            if switch_j == n {
                ans = 0;
                break
            }
            // スイッチを押す
            else {
                for panel_temp in 0..m {
                    current_bit[panel_temp] ^= bits[switch_j][panel_temp];
                }
            }
        }
    }

    // 掃き出し法の後に、bitが全て0のスイッチの数を数える
    let mut count_zero = 0;
    let mut all_zero_bit = vec![0; m];
    for i in 0..n {
        if bits[i] == all_zero_bit {
            count_zero += 1;
        }
    }
    // bitが全て0のスイッチは押しても押さなくても影響が無いので、押し方を2倍に増やせる
    for _ in 0..count_zero {
        ans = ans * 2 % MODULO;
    }
    println!("{}", ans);


    // 下記はvectorの代わりにbitを使って計算した。
    // m <= 300よりbit <= 1 << 300 ~ 10^90 でoverflow した。

    // // n個のスイッチ, m個のパネル
    // // a[i]をスイッチiが返す、パネルの表裏を表すbitに変換
    // let mut bits = vec![];
    // for i in 0..n {
    //     let mut bit = 0;
    //     for j in 0..a[i].len() {
    //         bit += 1 << (a[i][j] - 1);
    //     }
    //     // println!("switch: {}, bits: {:06b}", i, bit);
    //     bits.push(bit);
    // }
    // // 所望のM枚のパネルの状態
    // let mut desired_bit = 0;
    // for i in 0..m {
    //     desired_bit += s[i] << i;
    // }

    // // 掃き出し法
    // let mut get_switch_from_panel = vec![n; m]; // パネルiに対応するスイッチは get_switch_from_panel[i]. nなら無し
    // let mut top_index = 0;
    // // for i in 0..m {
    // // 各パネル(列)のループ
    // for panel_i in 0..m {
    //     // その列の1番上にある表(=1)のスイッチを探す(初期値n)
    //     let mut top_index_seen = false;
    //     // println!("panel_i: {}", panel_i);

    //     // 各スイッチ(行)のループ
    //     for switch_j in top_index..n {
    //         // println!("before -> switch_j: {}, bits: {:06b}", switch_j, bits[switch_j]);
    //         // panel_iで1番上にある表のスイッチか判定
    //         if 1 << panel_i & bits[switch_j] != 0 {
    //             get_switch_from_panel[panel_i] = top_index;
    //             // 行を入れ替える
    //             // println!("swap {} <-> {}", switch_j, top_index);
    //             bits.swap(switch_j, top_index);
    //             top_index_seen = true;
    //             top_index += 1;
    //             break
    //         }
    //         // println!("after  -> switch_j: {}, bits: {:06b}", switch_j, bits[switch_j]);
    //     }
    //     if top_index_seen {
    //         for switch_j in 0..n {
    //             if switch_j == top_index-1 {continue}
    //             // XORする
    //             if 1 << panel_i & bits[switch_j] != 0 {
    //                 bits[switch_j] ^= bits[top_index-1];
    //             }
    //         }
    //     }
        
    //     // // debug
    //     // for switch_j in 0..n {
    //     //     println!("switch: {}, bits: {:06b}", switch_j, bits[switch_j]);
    //     // }
    // }
    // // println!("get_switch_from_panel: {:?}", get_switch_from_panel);

    // // 貪欲に希望通りのパネルに近づけていく
    // let mut ans = 1;
    // let mut current_bit = 0;
    // for panel_i in 0..m {
    //     // 希望のビットと現在のビットのpanel_iビット目が異なる場合、対応するスイッチを押す
    //     if ((desired_bit ^ current_bit) & (1 << panel_i)) != 0 {
    //         // panel_iビット目を返せるスイッチを取得
    //         let switch_j = get_switch_from_panel[panel_i];
    //         // 返せるビットがないときは、希望の押し方無し
    //         if switch_j == n {
    //             ans = 0;
    //             break
    //         }
    //         // スイッチを押す
    //         else {
    //             current_bit ^= bits[switch_j];
    //         }
    //     }
    // }

    // // 掃き出し法の後に、bitが全て0のスイッチの数を数える
    // let mut count_zero = 0;
    // for i in 0..n {
    //     if bits[i] == 0 {
    //         count_zero += 1;
    //     }
    // }
    // // bitが全て0のスイッチは押しても押さなくても影響が無いので、押し方を2倍に増やせる
    // for _ in 0..count_zero {
    //     ans = ans * 2 % MODULO;
    // }
    // println!("{}", ans);
}