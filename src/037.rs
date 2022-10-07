use proconio::input;
use itertools::Itertools;
use std::cmp::{max, min};
use std::cmp::Ordering;
use std::collections::VecDeque;
use std::collections::BinaryHeap;
use proconio::marker::Chars;
fn main() {
    input! {
        w: usize,
        n: usize,
    }

    let mut l = vec![];
    let mut r = vec![];
    let mut v = vec![];

    for i in 0..n {
        input! {
            l_i: usize,
            r_i: usize,
            v_i: isize
        }
        l.push(l_i);
        r.push(r_i);
        v.push(v_i);
    }

    //    3
    //  2   2
    // 1 1 1 1
    // let mut seg_tree = SegmentTree::new(4);

    // println!("{:?}", seg_tree);
    // seg_tree.print_list();

    // seg_tree.update(0, 100);
    // seg_tree.update(1, 5);
    // seg_tree.update(2, 1000);
    // println!("rmq: {}", seg_tree.range_max_query(0, 2));

    // println!("{:?}", seg_tree);
    // seg_tree.print_list();
    // seg_tree.print_tree();

    // return;


    let NEGATIVE_INF = (1 << 60) * (-1);
    // dp[n][w] n+1個の料理でちょうどw[g]の香辛料を使った時の価値の最大値
    let mut dp = vec![vec![NEGATIVE_INF; w+1]; n];
    // Example0:
    // 100 4
    // 30 40 120
    // 30 40 30
    // 30 40 1500
    // 30 40 40
    // dp[n][w] n+1個の料理でちょうどw[g]の香辛料を使った時の価値の最大値
    //     10  20  30  40  50  60  70  80  90  100
    // 0   INF INF 120 120 INF INF INF INF INF INF
    // 1   INF INF 120 120 INF 150 150 150 INF INF
    // 2         1500 1500 INF 1620        1650
    // 3                                   1660

    // Example Ore original:
    // 10 3
    // 2 3 10
    // 4 6 100
    // 6 8 1000
    // dp[n][w] n+1個の料理でちょうどw[g]の香辛料を使った時の価値の最大値
    //     0   1   2   3   4   5   6   7   8    9    10
    // 0   0   0   10  10  0   0   0   0   0    0    0
    // 1   0   0   10  10  100 100 110 110 110  110  0
    // 2   0   0   10  10  100 100 110 110 110 110 1100

    // 高さlog(w+1)のセグメント木を、n本用意
    let mut trees = vec![SegmentTree::new(w+1); n];

    // dpとセグメント木の初期化
    for j in 0..(w+1) {
        for k in l[0]..(r[0]+1) {
            // 重量オーバーで料理を追加出来ないときはスキップ
            if j < k {continue}
            dp[0][k] = v[0];
        }
        // セグメント木を更新
        trees[0].update(j, dp[0][j]);
    }
    for i in 0..n {
        dp[i][0] = 0;
        trees[0].update(0, dp[i][0]);
    }

    for i in 1..n {
        for j in 0..(w+1) {
            let left: usize;
            let right: usize;
            // 重量オーバーで、料理を追加出来ないとき
            if j < l[i] {
                dp[i][j] = dp[i-1][j];
            }
            else {
                left = max(0, j as isize - r[i] as isize) as usize;
                right = j - l[i];
                let rmq = trees[i-1].range_max_query(left, right);
                // 遷移元が存在しない為に、料理を追加出来ないとき
                if rmq == NEGATIVE_INF {
                    dp[i][j] = dp[i-1][j];
                }
                // 料理を追加できるとき
                else {
                    dp[i][j] = max(rmq + v[i], dp[i-1][j]);
                }    
            }
            trees[i].update(j, dp[i][j]);
            // 下記の方法は計算量がO(N*W^2)となりTLE
            // for k in l[i]..(r[i]+1) {
            //     // 重量オーバーで、i番目の料理を追加出来ないとき
            //     if j < k {
            //         dp[i][j] = max(dp[i][j], dp[i-1][j]);
            //     }
            //     // 遷移元が無くて、料理を追加できないとき
            //     else if dp[i-1][j-k] == 0 && j-k != 0{
            //         dp[i][j] = max(dp[i][j], dp[i-1][j]);
            //     }
            //     // 料理を追加出来るとき
            //     else {
            //         dp[i][j] = max(dp[i][j], dp[i-1][j-k] + v[i]);
            //         dp[i][j] = max(dp[i][j], dp[i-1][j]);
            //     }
            // }
        }

    }
    // 最終的なdpテーブルと、セグメント木の確認
    // for i in 0..n {
    //     println!("dp[{}]: {:?}", i, dp[i]);
    //     // trees[i].print_list();
    //     trees[i].print_tree();
    // }

    let answer = dp[n-1][w];
    if answer == NEGATIVE_INF {
        println!("-1");
    }
    else {
        println!("{}", answer);
    }

}

// セグメント木
// Derive注釈は、自作の構造体に有用な振る舞いを追加する。(Debugはprintの為、Cloneはベクトルの要素として使う為に追加した)
// 参考: https://doc.rust-jp.rs/book-ja/ch05-02-example-structs.html?highlight=derive#%E3%83%88%E3%83%AC%E3%82%A4%E3%83%88%E3%81%AE%E5%B0%8E%E5%87%BA%E3%81%A7%E6%9C%89%E7%94%A8%E3%81%AA%E6%A9%9F%E8%83%BD%E3%82%92%E8%BF%BD%E5%8A%A0%E3%81%99%E3%82%8B
#[derive(Debug, Clone)]
struct SegmentTree {
    // 探索対象の配列の大きさ
    list_size: usize,
    // セグメント木の頂点の総数
    tree_size: usize,
    // セグメント木の葉の総数
    leaf_size: usize,
    // セグメント木
    tree: Vec<isize>,
}

// セグメント木実装で参考にしたのは、蟻本とE8氏の解答と下記のブログ
// https://easthop.hatenablog.com/entry/2020/12/15/211044
impl SegmentTree {
    fn new(list_size: usize) -> Self {
        // セグメント木の頂点の総数tree_sizeを求める。
        // まずはセグメント木の葉の数leaf_sizeを、
        // (leaf_size / 2 < list_size <= leaf_size)
        // を満たす2のべき乗数となるように計算
        let mut leaf_size = 1;
        while (leaf_size < list_size) {
            leaf_size *= 2;
        }

        // セグメント木の頂点数 = セグメント木の葉の数 * 2 - 1
        let tree_size = leaf_size * 2 - 1;

        // 1 << 60 = 1,152,921,504,606,846,976 = 1.152 * 10^18
        let NEGATIVE_INF = (1 << 60) * (-1);
        let tree = vec![NEGATIVE_INF; tree_size];
        return SegmentTree {list_size, tree_size, leaf_size, tree}
    }

    //      0
    //  1       2
    // 3, 4    5, 6
    // child1 = 2 * parent + 1
    // child2 = 2 * parent + 2

    // 探索対象の配列のk番目の要素を、値xに変更する
    fn update(&mut self, k: usize, x: isize) {
        // セグメント木におけるインデックスに変換
        let mut tree_index = k + self.tree_size / 2;
        self.tree[tree_index] = x;

        // 木を登りながら更新
        while tree_index > 0 {
            // 親の頂点
            tree_index = (tree_index - 1) / 2;
            let child_index_0 = tree_index * 2 + 1;
            let child_index_1 = tree_index * 2 + 2;
            self.tree[tree_index] = max(self.tree[child_index_0], self.tree[child_index_1]);
        }
    }

    // クラスの外からクエリを行うときのメソッド
    fn range_max_query(&self, q_l: usize, q_r: usize) -> isize {
        return self._range_max_query(q_l, q_r, 0, 0, self.leaf_size - 1);
    }

    // 閉区間[q_l, q_r]の最大値を求める。右端が開区間')'ではなく、閉区間']'にしているので注意
    fn _range_max_query(&self, q_l: usize, q_r: usize, v: usize, v_l: usize, v_r: usize) -> isize {
        // q_l:    探索区間の左端
        // q_r:    探索区間の右端(閉区間)
        // v:      現在の頂点のインデックス
        // v_l:    現在の頂点の守備範囲の左端
        // v_r:    現在の頂点の守備範囲の右端(閉区間)
        // 外からは、self._range_max_query(q_l, q_r, 0, 0, self.leaf_size - 1)として呼ぶ。特にv_rは、self.list_sizeではないので注意

        // (1)探索範囲が、その頂点が持つ守備範囲と、交差しない
        if v_r < q_l || q_r < v_l {
            let NEGATIVE_INF = (1 << 60) * (-1);
            return NEGATIVE_INF
        }
        // (2)探索範囲が、その頂点が持つ守備範囲を、完全に含む
        else if q_l <= v_l && v_r <= q_r {
            return self.tree[v]
        }
        // (3)探索範囲が、その頂点が持つ守備範囲と、部分一致
        else {
            // 2つの子頂点の内、大きい方を返す
            let child_0 = self._range_max_query(q_l, q_r, 2 * v + 1, v_l, (v_l + v_r) / 2);
            let child_1 = self._range_max_query(q_l, q_r, 2 * v + 2, (v_l + v_r) / 2 + 1, v_r);
            return max(child_0, child_1);
        }
    }

    // 配列を確認(デバッグ用)
    fn print_list(&self) {
        println!("Print Array: ");
        for i in 0..self.list_size {
            let tree_index = i + self.tree_size / 2;
            print!("{}, ", self.tree[tree_index]);
        }
        println!("");
    }

    // セグメント木を確認(デバッグ用)
    fn print_tree(&self) {
        let mut next_depth_index = 1;
        println!("Print Segment Tree: ");
        for i in 0..self.tree_size {
            if i == next_depth_index {
                println!("");
                next_depth_index = next_depth_index * 2 + 1;
            }
            print!("{} ", self.tree[i]);
            
        }
        println!("");
    }

}