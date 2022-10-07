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
        q: usize,
    }

    let mut t = vec![];
    let mut x = vec![];
    let mut y = vec![];
    let mut v = vec![];

    for i in 0..q {
        input! {
            t_i: usize,
            x_i: usize,
            y_i: usize,
            v_i: usize,
        }
        t.push(t_i);
        x.push(x_i - 1);
        y.push(y_i - 1);
        v.push(v_i);
    }

    // let mut graph = vec![vec![]; n];
    let mut uft = UnionFindTree::new(n);

    let INF = 1 << 60;
    // sum[i]には、a[i] + a[i+1]の値が格納されている
    let mut sum: Vec<isize> = vec![INF; n];
    for i in 0..q {
        if t[i] == 0 {
            // graph[x[i]].push(vec![y[i] as usize, v[i]]);
            // graph[y[i]].push(vec![x[i] as usize, v[i]]);
            sum[x[i]] = v[i] as isize;
        }
    }
    // println!("sum: {:?}", sum);
    // potential[i]には、連結したグループの先頭を0としたときの値が格納されている。a[0] = 0
    let mut potential: Vec<isize> = vec![0; n];
    for i in 0..(n-1) {
        if sum[i] == INF {continue}
        potential[i+1] = sum[i] - potential[i];
    }
    // println!("potential: {:?}", potential);

    for i in 0..q {
        if t[i] == 0 {
            uft.unite(x[i], y[i]);
            // graph[x[i]].push(vec![y[i] as usize, v[i]]);
            // graph[y[i]].push(vec![x[i] as usize, v[i]]);
        }
        else {
            // a[x[i]] = v[i]のとき、a[y[i]]の値を知りたい。
            if !uft.issame(x[i], y[i]) {
                println!("Ambiguous");
            }
            else {
                let diff = v[i] as isize - potential[x[i]] as isize;
                let index_distance = max(y[i], x[i]) - min(y[i], x[i]);
                // インデックスの差が、2の倍数であるときは、同じ分だけ上がる
                if index_distance % 2 == 0 {
                    println!("{}", potential[y[i]] as isize + diff);
                }
                // インデックスの差が、2の倍数+1 であるときは、同じ分だけ下がる
                else {
                    println!("{}", potential[y[i]] as isize - diff);
                }

                // グラフの深さ優先探索で解くとTLEする
                // let mut a_yi = 0;
                // let mut seen = vec![false; n];
                // saiki(x[i], y[i], v[i], &mut graph, &mut a_yi, &mut seen);
                // println!("{}", a_yi);
            }
        }
    }
}

fn saiki(xi: usize, yi: usize, a_xi: usize, graph: &mut Vec<Vec<Vec<usize>>>, a_yi: &mut usize, seen: &mut Vec<bool>) {
    seen[xi] = true;
    if yi == xi {
        *a_yi = a_xi;
        return;
    }
    for i in 0..graph[xi].len() {
        let xi_next = graph[xi][i][0];
        // 逆流を防ぐ
        if seen[xi_next] {continue}
        let a_xi_next = graph[xi][i][1] - a_xi;
        saiki(xi_next, yi, a_xi_next, graph, a_yi, seen);
    }
}



// Union-Find
// グループの管理（グループ同士の結合や、要素同士の所属か同じか判定）するのに便利なデータ構造
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

