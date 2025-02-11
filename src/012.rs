use proconio::input;
use itertools::Itertools;
use std::cmp::{max, min};
use std::cmp::Ordering;
use std::collections::VecDeque;
use std::collections::BinaryHeap;
use proconio::marker::Chars;
use std::mem::swap;

fn main() {
    input! {
        h: usize,
        w: usize,
        q: usize,
    }
    
    let mut query = vec![vec![]; q];
    // HxWのマス目を作る。0が白、1が赤とする。
    let mut map = vec![vec![0; w]; h];
    let mut uft = UnionFindTree::new(h*w);
    let cross_steps = vec![[-1, 0], [1, 0], [0, -1], [0, 1]];
    for i in 0..q {
        input! {
            t_i: usize
        }
        if t_i == 1 {
            input! {
                r_i: usize,
                c_i: usize
            }
            query[i] = vec![t_i, r_i, c_i];
        }
        else if t_i == 2 {
            input! {
                ra_i: usize,
                ca_i: usize,
                rb_i: usize,
                cb_i: usize
            }
            query[i] = vec![t_i, ra_i, ca_i, rb_i, cb_i];
        }
    }

    for i in 0..q {
        let t_i = query[i][0];
        if t_i == 1 {
            let r_i = query[i][1];
            let c_i = query[i][2];
            // マス目を赤く塗る
            map[r_i - 1][c_i - 1] = 1;
            let flat_index = get_flat_index(r_i - 1, c_i - 1, w);

            // 周囲が赤かったら結合する
            for step in cross_steps.iter() {
                let y_around_v = r_i as isize - 1 + step[0];
                let x_around_v = c_i as isize - 1 + step[1];
                if y_around_v < 0 || h as isize <= y_around_v {continue}
                if x_around_v < 0 || w as isize <= x_around_v {continue} 
                let v_around = get_flat_index(y_around_v as usize, x_around_v as usize, w);
                if map[y_around_v as usize][x_around_v as usize] == 1 {
                    uft.unite(flat_index, v_around);
                }
            }
        }
        else if t_i == 2 {
            let ra_i = query[i][1];
            let ca_i = query[i][2];
            let rb_i = query[i][3];
            let cb_i = query[i][4];
            if !(map[ra_i - 1][ca_i - 1] == 1 && map[rb_i - 1][cb_i - 1] == 1) {
                println!("No");
                // dbg!(1);
                continue
            }
            let flat_index_a = get_flat_index(ra_i - 1, ca_i - 1, w);
            let flat_index_b = get_flat_index(rb_i - 1, cb_i - 1, w);
            if uft.issame(flat_index_a, flat_index_b) {
                println!("Yes");
            }
            else {
                println!("No");
                // dbg!(2);
            }
        }
        // for j in 0..h {
        //     println!("{:?}", map[j]);
        // }
        // println!("=====");
    }

}

fn get_flat_index(h_i: usize, w_i: usize, w: usize) -> usize {
    let mut flat_index = h_i * w + w_i;
    return flat_index
}


// Union-Find
// グループの管理（グループ同士の結合や、要素同士の所属か同じか判定）するのに便利なデータ構造
#[derive(Clone)]
struct UnionFindTree {
    parents: Vec<usize>,    // 各頂点の属するグループ(根付き木)の親頂点の番号
    sizes: Vec<usize>       // 各頂点の属するグループ(根付き木)のサイズ(頂点数)
}

impl UnionFindTree {
    // 初期化
    fn new(n: usize) -> Self {
        // 各頂点が属するグループの根を格納
        let parents = (0..n).collect();
        // 各頂点が属するグループのサイズ(頂点数)を格納。※ただし、更新するのは根のインデックスだけで良い
        let sizes = vec![1; n];
        return UnionFindTree {parents, sizes}
    }

    // 根を求める。経路圧縮により計算量を削減
    fn root(&mut self, v: usize) -> usize {
        if self.parents[v] == v {return v}
        else {
            // 経路圧縮 (親を根に張り替える。)
            self.parents[v] = self.root(self.parents[v]);
            return self.parents[v] as usize
        }
    }

    // 同じグループに属するか
    fn issame(&mut self, v0: usize, v1: usize) -> bool {
        return self.root(v0) == self.root(v1)
    }

    // 頂点vが属する根付き木のサイズを取得
    fn size(&mut self, v: usize) -> usize {
        let root = self.root(v);
        return self.sizes[root]
    }

    // v0を含むグループと、v1を含むグループとを併合する。Union by sizeで計算量削減。
    fn unite(&mut self, mut v0: usize, mut v1: usize) -> bool {
        // 既に同じグループであれば何もしない
        v0 = self.root(v0);
        v1 = self.root(v1);
        if v0 == v1 {
            return false
        } 
        let child: usize;
        let parent: usize;
        // Union by sizeにより、サイズが小さいグループを、サイズが大きいグループに併合する
        if self.size(v0) <= self.size(v1) {
            child = v0;
            parent = v1;
        }
        else {
            child = v1;
            parent = v0;
        }
        self.sizes[parent] += self.size(child);
        self.parents[child] = parent;
        return true
    }
}
