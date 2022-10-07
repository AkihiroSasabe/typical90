use proconio::input;
use itertools::Itertools;
use std::cmp::{max, min};
use std::cmp::Ordering;
use std::collections::VecDeque;
use std::collections::BinaryHeap;
use proconio::marker::Chars;

// 51
// 15:04 開始
// 16:57 中断
// 19:57 再開
// 20:36 完了

fn main() {
    input! {
        n: usize,
        k: usize,
        p: usize,
        mut a: [usize; n]
    }

    let mut a0 = vec![];
    let mut a1 = vec![];

    // 配列aを2つに分割(いわゆる半分全列挙)
    // nが奇数だと、a1の方が1個多くなる可能性がある。
    for i in 0..n {
        if i < n/2 {
            a0.push(a[i]);
        }
        else {
            a1.push(a[i]);
        }
    }

    // 縦軸が個数、横軸が価格の配列取得。計算量は、ビット全探索なので、O(N/2 * 2^(N/2))
    let sum0 = get_pickupnum_pricesum_table(k, p, &a0);
    let sum1 = get_pickupnum_pricesum_table(k, p, &a1);
    

    let mut ans = 0;
    // 配列a0から選択する商品の数: pick_num_0
    for pick_num_0 in 0..(k+1) {
        let pick_num_1 = k - pick_num_0;
        // 選択した商品の数がpick_num_0のときに、ありえる価格price0
        for i in 0..sum0[pick_num_0].len() {
            let price0 = sum0[pick_num_0][i];
            let permit_price = p - price0;
            // 0 1 <2> 3 4 5
            let num = sum1[pick_num_1].upper_bound(&permit_price);
            ans += num;
        } 
    }
    println!("{}", ans);

    // 普通に組み合わせ全探索するとTLEする
    // a.sort();
    // let mut ans = 0;
    // saiki(0, k, p, 0, &mut ans, &a, n, 0);
    // println!("{}", ans);
}


// 縦軸が個数、横軸が価格の配列取得。計算量は、ビット全探索なので、O(N/2 * 2^(N/2))
fn get_pickupnum_pricesum_table(k: usize, p: usize, a0: &Vec<usize>) -> Vec<Vec<usize>> {
    let mut sum0 = vec![vec![]; k+1];
    // ビット全探索O(N*2^N)。このビットはn/2個の配列の中からどの商品をチョイスするかを各桁で表現している。
    for bit in 0..(1 << (a0.len())) {
        let mut price_sum = 0;
        let mut pick_num = 0;
        for digit in 0..(a0.len()) {
            if bit & 1 << digit != 0 {
                price_sum += a0[digit];
                pick_num += 1;
            }
        }
        if price_sum <= p {
            if pick_num > k {continue}
            sum0[pick_num].push(price_sum);
        }
    }
    for i in 0..sum0.len() {
        sum0[i].sort();
    }
    return sum0;
}


// fn saiki(select_num: usize, k: usize, p: usize, price_sum: usize, ans: &mut usize, a: &Vec<usize>, n: usize, start_index: usize) {
//     if select_num == k {
//         // println!("{}", start_index);
//         if price_sum <= p {
//             *ans += 1;
//         }
//         return;
//     }
//     for i in start_index..n {
//         if price_sum + a[i] > p {break}
//         saiki(select_num + 1, k, p, price_sum + a[i], ans, a, n, i+1);
//     }
// }



// lower_bound=Key★以★上★のインデックス、
// upper_bound=Key★よ★り★大きいインデックス
// sorted_list.lower_bound(&x)は、x以上となる最小のインデックスを返すが、x超えがリスト内に無いときは、sorted_list.len()を返すので注意
/// Equivalent to std::lowerbound and std::upperbound in c++
pub trait BinarySearch<T> {
    fn lower_bound(&self, x: &T) -> usize;
    fn upper_bound(&self, x: &T) -> usize;
}

impl<T: Ord> BinarySearch<T> for [T] {
    fn lower_bound(&self, x: &T) -> usize {
        let mut low = 0;
        let mut high = self.len();

        while low != high {
            let mid = (low + high) / 2;
            match self[mid].cmp(x) {
                Ordering::Less => {
                    low = mid + 1;
                }
                Ordering::Equal | Ordering::Greater => {
                    high = mid;
                }
            }
        }
        low
    }

    fn upper_bound(&self, x: &T) -> usize {
        let mut low = 0;
        let mut high = self.len();

        while low != high {
            let mid = (low + high) / 2;
            match self[mid].cmp(x) {
                Ordering::Less | Ordering::Equal => {
                    low = mid + 1;
                }
                Ordering::Greater => {
                    high = mid;
                }
            }
        }
        low
    }
}
