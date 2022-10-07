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
        mut a: [usize; n]
    }

    // n: 300_000
    // A: 1 2 3 3 2 1
    // B: 1 2 3 2 1


    // 最長増加部分列(LIS: Longest Increasing Subsequence): i<jでa[i]<a[j]
    let lis_long_list = longest_increasing_subsequence(&a);

    // 最長減少部分列LDS
    a.reverse();
    let mut lds_long_list = longest_increasing_subsequence(&a);
    lds_long_list.reverse();
    
    let mut ans = 0;
    for i in 0..n {
        let lis_long = lis_long_list[i];
        let lds_long = lds_long_list[i];
        // println!("lis_long: {}, lds_long: {}", lis_long, lds_long);
        ans = max(ans, lis_long + lds_long - 1);
    }

    // for i in 0..n {
    //     println!("{:?}", dp_lis[i]);
    // }

    println!("{}", ans);

}


// 最長増加部分列(LIS: Longest Increasing Subsequence)を求める。
// LISとは、全てのi<jでa[i]<a[j]を満たす数列aの部分列のこと。
// 蟻本p63~65参考: O(NlogN)で解ける(2分探索を使う)
// 本問題を解くために、一部修正している。
fn longest_increasing_subsequence(a: &Vec<usize>) -> Vec<usize> {
    // 例 
    // n, aは下記の通り。
    // 5
    // 4 2 3 1 5

    // dpの遷移: 
    // 4 N N N N    a[0]まで使えるときのdp
    // 2 N N N N    a[1]まで使えるときのdp
    // 2 3 N N N    a[2]まで使えるときのdp
    // 1 3 N N N    a[3]まで使えるときのdp
    // 1 3 5 N N    a[4]まで使えるときのdp
    // 最終的に、最長増加部分列は[1,3,5]となり、長さは3となる。(NはINFINITEで無視する)

    // また、入力の数列aを反転させると、最長減少部分文字列を得られる。
    // 5 1 3 2 4
    // dpの遷移: 
    // 5 N N N N   a[0]まで使えるときのdp
    // 1 N N N N   a[1]まで使えるときのdp
    // 1 3 N N N   a[2]まで使えるときのdp
    // 1 2 N N N   a[3]まで使えるときのdp
    // 1 2 4 N N   a[4]まで使えるときのdp

    let INF = 1 << 60;
    let n = a.len();

    // dp[i]: 部分文字列の長さがi+1のとき、最後尾の最小値 (こっちの方がメモリ節約できる)
    let mut dp = vec![INF; n];
    dp[0] = a[0];
    // // dp[i][j]: aをインデックスiまで使えるとき、長さがj+1のときの、最後尾の最小値 (この問題に合わせてDPを変形)
    // let mut dp_2d = vec![vec![INF; n]; n];

    // lis_long_list[i]には、a[i]まで使えるときの最長増加部分列の長さが格納
    let mut lis_long_list = vec![];

    for i in 0..n {
        let index = dp.lower_bound(&a[i]);
        dp[index] = a[i];
        // println!("{:?}", dp);

        // 下の行は、この問題の為だけに存在。普段は要らない
        let lis_long = dp.lower_bound(&INF);
        lis_long_list.push(lis_long);
        // dp_2d[i] = dp.clone();
    }
    
    // println!("{:?}", dp);
    return lis_long_list;

    // 本当はdpを返すべき
    // return dp;
    // return dp_2d
}

    // // 蟻本: 下記はO(N^2)で解ける。
    // // i: 使って良いインデックス
    // for i in 0..n {
    //     dp[0] = min(dp[0], a[i]);
    //     //j: dpのj番目について考えているとき
    //     for j in 1..n {
    //         if dp[j-1] < a[i] {
    //             dp[j] = min(dp[j], a[i]);
    //         }
    //     }
    //     // println!("{:?}", dp);
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
