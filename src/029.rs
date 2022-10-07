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
    for i in 0..n {
        input! {
            l_i: usize,
            r_i: usize,
        }
        l.push(l_i - 1);
        r.push(r_i - 1);
    }

    // let mut segment_tree = SegmentTree::new(w);
    let mut lazy_segment_tree = LazySegmentTree::new(w);

    // for i in 0..w {
    //     segment_tree.update(i, 0);
    // }
    lazy_segment_tree.range_update(0, w -1, 0);
    // lazy_segment_tree.range_update(0, lazy_segment_tree.leaf_size -1, 0);
    // lazy_segment_tree.print_tree();

    for i in 0..n {
        // let max_height = segment_tree.range_max_query(l[i], r[i]);
        let max_height = lazy_segment_tree.range_max_query(l[i], r[i]);
        println!("{}", max_height+1);

        // 普通のセグメント木だとO(N * WlogW)かかってしまう。区間[l[i],r[i]]の更新でO(W*logW)
        // for j in l[i]..(r[i]+1) {
        //     segment_tree.update(j, max_height + 1);
        // }

        // lazy_segment_tree.print_tree();
        // 遅延評価セグメント木なら、O(N * logW)。区間[l[i], r[i]]の更新でO(logW)
        lazy_segment_tree.range_update(l[i], r[i], max_height + 1);
        // lazy_segment_tree.print_tree();
    }
}

// 普通のセグメント木の一点更新では、更新するべきノードを一気に更新してしまう。
// 更新を必要があるときまで遅らせるのが遅延評価セグメント木。
// Derive注釈は、自作の構造体に有用な振る舞いを追加する。(Debugはprintの為、Cloneはベクトルの要素として使う為に追加した)
// 参考: https://doc.rust-jp.rs/book-ja/ch05-02-example-structs.html?highlight=derive#%E3%83%88%E3%83%AC%E3%82%A4%E3%83%88%E3%81%AE%E5%B0%8E%E5%87%BA%E3%81%A7%E6%9C%89%E7%94%A8%E3%81%AA%E6%A9%9F%E8%83%BD%E3%82%92%E8%BF%BD%E5%8A%A0%E3%81%99%E3%82%8B
#[derive(Debug, Clone)]
struct LazySegmentTree {
    // 探索対象の配列の大きさ
    list_size: usize,
    // セグメント木の頂点の総数
    tree_size: usize,
    // セグメント木の葉の総数
    leaf_size: usize,
    // セグメント木
    tree: Vec<isize>,
    // 遅延配列を格納する木
    lazy_tree: Vec<isize>,
}

// 遅延評価セグメント木実装で参考にしたブログ(tsutaj氏)
// https://tsutaj.hatenablog.com/entry/2017/03/30/224339
// https://algo-logic.info/segment-tree/
impl LazySegmentTree {
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
        let lazy_tree = vec![NEGATIVE_INF; tree_size];
        return LazySegmentTree {list_size, tree_size, leaf_size, tree, lazy_tree}
    }

    // 遅延木のv番目の要素について、遅延評価を行う(セグメント木には無かったメソッド)
    fn eval(&mut self, v: usize, l: usize, r: usize) {
        // 自ノードの値配列に値を伝播させる
        // 子ノードの遅延配列に値を伝播させる
        // 自分のノードの遅延配列を空にする

        // 遅延評価で空でない場合、自頂点及び子頂点への値の伝播が起こる
        let NEGATIVE_INF = (1 << 60) * (-1);
        if (self.lazy_tree[v] != NEGATIVE_INF) {
            self.tree[v] = self.lazy_tree[v];

            // 最下段ではない場合、子頂点へ伝播
            if r -l >= 1 {
                self.lazy_tree[2*v+1] = self.lazy_tree[v];
                self.lazy_tree[2*v+2] = self.lazy_tree[v];
            }
            // 伝播が終わったので、自頂点の遅延配列を空にする
            self.lazy_tree[v] = NEGATIVE_INF;
        }
    }

    fn range_update(&mut self, q_l: usize, q_r: usize, x: isize) {
        // println!("start to update between {}-{} to {}", q_l, q_r, x);
        self._range_update(q_l, q_r, x, 0, 0, self.leaf_size - 1);
    }

    // 探索対象の配列の区間[l,r]の要素を、値xに変更する (1つの要素ではなく、複数要素を含む区間の更新)
    // 根から下に下がっていく。(セグメント木の1つの要素の更新のときは下から根に向かって更新していた)
    fn _range_update(&mut self, q_l: usize, q_r: usize, x: isize, v: usize, v_l: usize, v_r: usize) {
        // q_l:    探索区間の左端
        // q_r:    探索区間の右端(閉区間)
        // x:      更新後の値
        // v:      現在の頂点のインデックス
        // v_l:    現在の頂点の守備範囲の左端
        // v_r:    現在の頂点の守備範囲の右端(閉区間)
        // 外からは、self._range_update(q_l, q_r, x, 0, 0, self.leaf_size - 1)として呼ぶ。特にv_rは、self.list_sizeではないので注意
        
        // v番目の頂点の遅延評価
        self.eval(v, v_l, v_r);

        // (1)更新範囲が、その頂点が持つ守備範囲と、交差しない
        if v_r < q_l || q_r < v_l {
            // 何もしない
            return;
        }
        // (2)更新範囲が、その頂点が持つ守備範囲を、完全に含む:Query ⊃ Vertex
        else if q_l <= v_l && v_r <= q_r {
            // 遅延木に値を入れた後に評価
            self.lazy_tree[v] = x;
            // ここで評価を入れないと、(3)で子の配列が更新前の状態で、max()を呼び出すことになる
            self.eval(v, v_l, v_r);
            return;
        }
        // (3)更新範囲が、その頂点が持つ守備範囲と、部分一致:Query ∩ Vertex ≠ ∅
        else {
            // 2つの子頂点の内、大きい方に更新
            self._range_update(q_l, q_r, x, 2 * v + 1, v_l, (v_l + v_r) / 2);
            self._range_update(q_l, q_r, x, 2 * v + 2, (v_l + v_r) / 2 + 1, v_r);
            self.tree[v] = max(self.tree[2 * v + 1], self.tree[2 * v + 2]);
            return;
        }
    }

    // クラスの外からクエリを行うときのメソッド
    fn range_max_query(&mut self, q_l: usize, q_r: usize) -> isize {
        // println!("start to query between {}-{} !!", q_l, q_r);
        return self._range_max_query(q_l, q_r, 0, 0, self.leaf_size - 1);
    }

    // 閉区間[q_l, q_r]の最大値を求める。右端が開区間')'ではなく、閉区間']'にしているので注意
    fn _range_max_query(&mut self, q_l: usize, q_r: usize, v: usize, v_l: usize, v_r: usize) -> isize {
        // q_l:    探索区間の左端
        // q_r:    探索区間の右端(閉区間)
        // v:      現在の頂点のインデックス
        // v_l:    現在の頂点の守備範囲の左端
        // v_r:    現在の頂点の守備範囲の右端(閉区間)
        // 外からは、self._range_max_query(q_l, q_r, 0, 0, self.leaf_size - 1)として呼ぶ。特にv_rは、self.list_sizeではないので注意

        // 遅延評価!
        self.eval(v, v_l, v_r);

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
            // 2つの子頂点の内、小さい方を返す
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
    fn _print_tree(&self, kind: &str) {
        let mut next_depth_index = 1;
        for i in 0..self.tree_size {
            if i == next_depth_index {
                println!("");
                next_depth_index = next_depth_index * 2 + 1;
            }
            match kind {
                "segment_tree" => print!("{} ", self.tree[i]),
                "lazy_tree" => print!("{} ", self.lazy_tree[i]),
                _ => ()
            }
        }
        println!("");
    }

    fn print_tree(&self) {
        println!("==== Print Segment Tree ====");
        self._print_tree("segment_tree");
        println!("==== Print Lazy Tree ====");
        self._print_tree("lazy_tree");
        println!("==== ==== ==== ==== ==== =====");
    }
}