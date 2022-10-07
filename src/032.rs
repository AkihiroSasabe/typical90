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
        a: [[usize; n]; n],
        m: usize,
    }
    let mut ng_list= vec![vec![]; n];
    for i in 0..m {
        input! {
            x: usize,
            y: usize
        }
        // 隣接リストで険悪判定
        ng_list[x-1].push(y-1);
        ng_list[y-1].push(x-1);
        // // NG判定は隣接リストより隣接行列の方が計算量を少なくできた。
        // // 隣接行列で険悪判定
        // ng_matrix[x-1][y-1] = true;
        // ng_matrix[y-1][x-1] = true;
    }

    let INF = 1 << 60;
    let mut ans = INF;
    
    let perms = (0..n).permutations(n);
    // O(N!): 10! = 3.6... * 10^6
    // perm[i]にはi番目の地区をどの選手が走るか、が格納されている
    for perm in perms {
        // println!("{:?}", perm);
        let mut time_p = 0;
        let mut ok_flag = true;

        for i in 0..n {
            if !ok_flag {break}
            // タスキの受け渡しが出来るか確認
            for ng in ng_list[perm[i]].iter() {
                if i+1 >= n {continue}
                if *ng == perm[i+1] {
                    ok_flag = false;
                    break
                }
            }
            time_p += a[perm[i]][i];
        }
        if !ok_flag {continue}
        // println!("time: {}", time_p);
        ans = min(time_p, ans);
    }

    if ans == INF {
        println!("-1");
        return
    }

    println!("{}", ans);

}