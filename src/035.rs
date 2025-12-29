use proconio::input;
use itertools::Itertools;
use std::cmp::{max, min};
use std::cmp::Ordering;
use std::collections::VecDeque;
use std::collections::BinaryHeap;
use proconio::marker::Chars;
use std::collections::BTreeMap;
use std::mem::swap;
fn main() {
    // 方針:E8氏と同じ
    // 1. 頂点V1, V2, ...VkをDFSの入った順番にソート。
    // 2. dist(V1, V2) + dist(V2, V3) + ... + dist(Vk, V1) / 2 
    //    が求める「残す必要がある辺の本数」となる。
    // dist(V1, V2)はLCA(最近共通祖先)のメソッドを使うとO(logN)で解ける。
    // LCAについては下を参照
    
    input! {
        n: usize,
    }
    let mut graph = vec![vec![]; n];
    for i in 0..n-1 {
        input! {
            a_i: usize,
            b_i: usize,
        }
        graph[a_i-1].push(b_i-1);
        graph[b_i-1].push(a_i-1);
    }
    // 最近共通祖先のインスタンス化
    let root = 0; // 根はv=0としておく
    let lca = LowestCommonAncestor::new(&graph, root);

    // 根(頂点0)から入る順にソートする
    let mut order_list = vec![graph.len(); graph.len()]; // order_list[v] := 順番
    let mut order = 0;
    dfs(&graph, root, &mut order_list, &mut order);

    input! {
        q: usize
    }
    let mut btree_list = vec![];
    for i in 0..q {
        input! {
            k_i: usize,
        }
        input! {
            v_i: [usize; k_i]
        }
        let mut btree = BTreeMap::new();
        for i in 0..k_i {
            btree.insert(order_list[v_i[i] - 1], v_i[i] - 1);
        }
        btree_list.push(btree);
    }

    for i in 0..q {
        // println!("q = {}", i);
        let mut ans = 0;
        let (_, first_v) = btree_list[i].iter().next().unwrap();
        let mut pre_v = *first_v;
        for (index, (_, v)) in btree_list[i].iter().enumerate() {
            if index == 0 {continue}
            // println!("pre_v, v = {}, {}", pre_v+1, *v+1);
            ans += lca.get_distance(pre_v, *v);
            pre_v = *v;
        }
        ans += lca.get_distance(pre_v, *first_v);
        // println!("pre_v, v = {}, {}", pre_v+1, *first_v+1);
        println!("{}", ans / 2);
    }
}

// 入る順にソートする
fn dfs(graph: &Vec<Vec<usize>>, v: usize, order_list: &mut Vec<usize>, order: &mut usize) {
    order_list[v] = *order;
    *order += 1;

    for i in 0..graph[v].len() {
        let next_v = graph[v][i];
        if order_list[next_v] != graph.len() {continue}
        dfs(graph, next_v, order_list, order);
    }    
}

// 参考実装: https://algo-logic.info/lca/
// ■LCA: 木の頂点uと頂点vの最近共通祖先xを求めるデータ構造 (ダブリングを使わなくても2分探索できると思うけど、一応ダブリングで実装)
// ■前処理
// 1. DFSで全ての頂点について、根からの距離と親を求める
// 2. 2^k先の祖先を求める
// ■クエリ(uとvの最近共通祖先xを求める)
// 1. uとvの深い方の頂点を近い方の頂点と同じ深さになるように変更して(u1, v1)とする。
// 2. 2分探索を用いて、u1,v1を最近共通祖先xの一つ手前まで段階的に移動させる
// (u1,v1が同じにならないギリギリまで移動させる)
// 3. 最終的なu1, v1の一つ先がLCA
// ■応用1: 2点u,vの距離もdist(u) - dist(v) - 2*dist(x)でO(logN)で求まる。
// ■応用2: 点aがxとyの間にあるかもdist(u,a)+dist(a,v)==dist(u,v)でO(logN)で判定可能
#[derive(Debug)]
struct LowestCommonAncestor {
    parent: Vec<Vec<usize>>, // parent[k][u]:= u の 2^k個上の親
    distance: Vec<usize>     // rootからの距離
}
impl LowestCommonAncestor {
    pub fn new(graph: &Vec<Vec<usize>>, root: usize) -> Self {
        let g_size = graph.len();
        // 最終的に2^k < g_sizeとなるような最小のk (=: k_over)を求める
        let mut k_over = 1; 
        while (1 << k_over) < g_size {
            k_over += 1;
        }
        let mut parent = vec![vec![g_size; g_size]; k_over]; // 親として存在しない頂点g_sizeをつけておく
        let mut distance = vec![0; g_size];
        
        // 一般的なダブリングの前処理
        // 1. それぞれの要素について、1個先の要素が何か記録 (今回はDFS)
        // 2. 前の結果を利用して、それぞれの要素について 2 個先の要素が何か記録
        // 3. 前の結果を利用して、それぞれの要素について 4 個先の要素が何か記録
        // 4. 前の結果を利用して、それぞれの要素について 8 個先の要素が何か記録
        // ...
        // k. 前の結果を利用して、それぞれの要素について 2^k 個先の要素が何か記録

        // 1. DFSで全ての頂点について、根からの距離と親を求める
        LowestCommonAncestor::get_dist_from_root_by_dfs(graph, root, g_size, 0, &mut distance, &mut parent);
        // 2. 2^k個上の祖先を求める(前処理の計算量O(NlogK))
        for k in 0..k_over-1 {
            for v in 0..g_size {
                // vの2^k個上の祖先が存在しない場合
                if parent[k][v] == g_size {
                    parent[k+1][v] = g_size;
                }
                else {
                    parent[k+1][v] = parent[k][parent[k][v]];
                }
            }
        }
        LowestCommonAncestor {parent, distance}
    }

    /// DFSで全ての頂点vについて、根からの距離と親(1つ上の頂点)を求める
    fn get_dist_from_root_by_dfs(
        graph: &Vec<Vec<usize>>, // graph[v][i] := 頂点vとi番目に繋がっている頂点
        v: usize, // 頂点v
        p: usize, // 頂点vの親p
        d: usize, // 根と頂点vの距離d
        distance: &mut Vec<usize>,   // distance[v] := 頂点vと根からの距離
        parent: &mut Vec<Vec<usize>> // parent[k][v] := vの2^k個上の祖先
    )
    {
        distance[v] = d;
        parent[0][v] = p; // 頂点vの2^0=1個上の親をpとする
        for i in 0..graph[v].len() {
            let next_v = graph[v][i];
            // 逆流しないようにする(子から親へ進まないようにする)
            if next_v != p {
                LowestCommonAncestor::get_dist_from_root_by_dfs(graph, next_v, v, d + 1, distance, parent);
            }
        }
    }

    /// 頂点uと頂点vのLCA(最近共通祖先)を求める
    pub fn query(&self, mut u: usize, mut v: usize) -> usize {
        // uの方が深いようにする
        if self.distance[u] < self.distance[v] {
            swap(&mut u, &mut v);
        }
        // 頂点までの距離を同じにする
        let k_over = self.parent.len();
        let diff = self.distance[u] - self.distance[v];
        for k in 0..k_over {
            if (diff & (1 << k)) != 0 {
                u = self.parent[k][u];
            }
        }
        // LCA(最近共通祖先を求める)
        //         0
        //    1          2
        //  3   4     5     6
        // 7 8 9 10 11 12 13 14
        // 例えば上の木の7と10の最近共通祖先は1

        // 2分探索で求める
        if u == v {return u}
        for i in 0..k_over {
            let k = k_over - 1 - i;
            if self.parent[k][u] != self.parent[k][v] {
                // 2^k個上が、共通祖先じゃなかったら上がっていく
                // (最近共通祖先じゃなくても上がっていく)
                u = self.parent[k][u];
                v = self.parent[k][v];
            }
        }
        // forループが終わると、uとvは最近共通祖先の1個下に居る。
        // (任意の数字は2の累乗の和で表現できるので、今回の"uから最近共通祖先までの距離-1"まで上がることも可能)
        // 返すべきは最近共通祖先なので、今のuの1=2^0個上の頂点を返す
        return self.parent[0][u];

        // 2分探索で求める
        // let mut ng: usize = 1 << (k_over-1); // uとvのng個上の頂点は共通祖先
        // let mut ok: usize = 0;        // uとvのok個上の頂点は共通祖先ではない
        // while (ng as isize - ok as isize).abs() < 1 {
        //     let mid = (ok + ng) / 2;
        //     let mut u_temp = u;
        //     let mut v_temp = v;
        //     for k in 0..k_over {
        //         if (mid & (1 << k)) != 0 {
        //             u_temp = self.parent[k][u_temp];
        //             v_temp = self.parent[k][v_temp];
        //         }
        //     }
        //     if u_temp != v_temp {
        //         ok = mid;
        //     }
        //     else {
        //         ng = mid;
        //     }
        // }
        // return ok + 1
    }

    /// 頂点uと頂点vの距離を求める
    pub fn get_distance(&self, u: usize, v: usize) -> usize {
        let lca = self.query(u, v);
        let dist_from_u_to_lca = self.distance[u] - self.distance[lca];
        let dist_from_v_to_lca = self.distance[v] - self.distance[lca];
        return dist_from_u_to_lca + dist_from_v_to_lca
    }

    /// 頂点xが頂点uと頂点vを結ぶパス上に存在するか?
    pub fn is_on_path(&self, x: usize, u:usize, v: usize) -> bool {

        return self.get_distance(u, x) + self.get_distance(x, v) == self.get_distance(u, v)
        
        // (下記でも良さそう.※要検証)
        // let lca_xu = self.query(x, u);
        // let lca_xv: usize = self.query(x, v);
        // return lca_xu == x || lca_xv == x

    }

}


// ダブリングの定義: https://algo-logic.info/lca/#
// ダブリングは全体の要素数がN個のとき「K個先の要素を求めるのに O(K) かかる」ような状況を
// 前処理：O(NlogK) 時間, O(NlogK) 空間
// クエリ：O(logK)
// で行うことができるようにするアルゴリズムのこと。

// ・ダブリングのアイディア by E8氏
// parent[k][v] := 頂点vの2^k個上にいる頂点
// parent[k+1][v] := parent[k][parent[k][v]]
// で求めることができる。
// 計算量O(NlogN) <- ?

// ◆ダブリングでできること[1]: d個上の頂点を求める
// 頂点vの37個上の頂点
// 37 = 1 + 4 + 32
//    = 2^0 + 2^2 + 2^5
// parent[37][v] = parent[0][parent[2][parent[5][v]]]
// O(logD)で求めることが可能 <- これは分かる

// ◆ダブリングでできること[2]: 最近共通祖先LCA


// ◆ダブリングでできること[3]: 2頂点間の経路長
// 頂点xから頂点yまでの経路の長さは、
// depth[x] + depth[y] - 2 * depth[LCA(x, y)]
// 計算量: O(log(N)) <- 分かる

