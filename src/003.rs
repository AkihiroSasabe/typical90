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
    let mut graph = vec![vec![]; n];
    for i in 0..(n-1) {
        input! {
            ai: usize,
            bi: usize,
        }
        graph[ai - 1].push(bi - 1);
        graph[bi - 1].push(ai - 1);
    }

    // 木の直径を求める
    use tree_diameter::{get_diamter_for_graph_without_weights, get_diamter_for_graph_with_weights};
    let (diameter, vs, vt) = get_diamter_for_graph_without_weights(&graph);
    // let (diameter, vs, vt) = get_diamter_for_graph_with_weights(&graph);

    let ans = diameter + 1;
    println!("{}", ans);
}

/// 木の直径を求めるモジュール (参考: https://algo-logic.info/tree-diameter/)
/// 木の直径とは、木に存在する2つノード間の最大距離を木の直径
/// 重み無しの木でも、重み付きの木でも、木の最遠頂点間の距離が直径
/// 1.計算量
/// O(|V|)
/// 2.使い方
/// use tree_diameter::{get_diamter_for_graph_without_weights, get_diamter_for_graph_with_weights};
/// let (diameter, vs, vt) = get_diamter_for_graph_without_weights(&graph); // 重み無しの木
/// let (diameter, vs, vt) = get_diamter_for_graph_with_weights(&graph); // 重み付きの木
/// 3.アルゴリズム詳細
/// 3.1.任意の頂点 v0 を選ぶ
/// 3.2.v0からBFS or DFSで、最も遠くにある頂点 vs を探索する
/// 3.3.vsからBFS or DFSで、最も遠くにある頂点 vt を探索する
/// 3.4.vsとvtを結ぶパスが直径となる
mod tree_diameter {
    use std::collections::VecDeque;
    /// 重み無しの木の直径を求める
    pub fn get_diamter_for_graph_without_weights(graph_without_weights: &Vec<Vec<usize>>) -> (usize, usize, usize) {
        let n = graph_without_weights.len();
        let mut graph_with_weights: Vec<Vec<(usize, usize)>> = vec![vec![]; n];
        for v in 0..n {
            for &nv in graph_without_weights[v].iter() {
                graph_with_weights[v].push((nv, 1));
            }
        }
        let (diameter, vs, vt) = get_diamter_for_graph_with_weights(&graph_with_weights);
        return (diameter, vs, vt)
    }

    /// 重み有りの木の直径を求める
    pub fn get_diamter_for_graph_with_weights(graph: &Vec<Vec<(usize, usize)>>) -> (usize, usize, usize) {
        
        // 任意の頂点 v0
        let v0 = 0;
        // 任意の頂点 v0 から各頂点への距離を取得
        let distances_from_v0 = get_distances(graph, v0);
        // 任意の頂点vsからの距離が最大の頂点 vs を取得 ( vs が木の直径の端点の片側)
        let (max_dist, vs) = get_max_distance_and_its_vertex(&distances_from_v0);

        // vs から各頂点への距離を取得
        let distances_from_vs = get_distances(graph, vs);

        // vs からの距離の最大が木の直径 diameter で、その頂点 vt が、木の直径の端点の反対側となる。
        let (diameter, vt) = get_max_distance_and_its_vertex(&distances_from_vs);

        return (diameter, vs, vt)
    }

    /// 幅優先探索で、頂点 start_v から、各頂点への距離を求める
    fn get_distances(graph: &Vec<Vec<(usize, usize)>>, start_v: usize) -> Vec<usize> {
        let n = graph.len();
        let mut queue = VecDeque::new();
        let init_distance = usize::MAX;
        let mut distances = vec![init_distance; n];
        distances[start_v] = 0;
        queue.push_back(start_v);
        while queue.len() != 0 {
            let v = queue.pop_front().unwrap();
            for &(nv, nw) in graph[v].iter() {
                // 既に通ったことがあれば、スルー
                if distances[nv] != init_distance {continue}
                distances[nv] = distances[v] + nw;
                queue.push_back(nv);
            }
        }
        return distances
    }

    /// 最大距離と、その頂点を求める
    fn get_max_distance_and_its_vertex(distances: &Vec<usize>) -> (usize, usize) {
        let n = distances.len();
        let mut max_dist = distances[0];
        let mut max_v = 0;
        for v in 1..n {
            if max_dist < distances[v] {
                max_v = v;
                max_dist = distances[v];
            }
        }
        return (max_dist, max_v)
    }
}