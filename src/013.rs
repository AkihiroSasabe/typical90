use proconio::{input, marker::{Usize1, Isize1}};
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
    let mut graph: Vec<Vec<(usize, usize)>> = vec![vec![]; n];
    for i in 0..m {
        input! {
            ai: Usize1,
            bi: Usize1,
            ci: usize,
        }
        graph[ai].push((bi, ci));
        graph[bi].push((ai, ci));
    }

    let distances = get_minimum_distances_by_dijkstra_algorithm(&graph, 0);
    let distances_from_goal = get_minimum_distances_by_dijkstra_algorithm(&graph, n-1);
    for i in 0..n {
        // distances[i] := パス(0,i) の最短距離
        // distances_from_goal[i] := パス(goal,i) の最短距離。
        // この問題は、無向グラフなので、パス(goal,i) と パス(i, goal) の距離は等しい
        // よって、パス(0, i, goal) = パス(0,i) + パス(i,goal) = パス(0,i) + パス(goal, i)
        let dist = distances[i] + distances_from_goal[i];
        println!("{}", dist);
    }

}


// ダイクストラ法
fn get_minimum_distances_by_dijkstra_algorithm(graph: &Vec<Vec<(usize, usize)>>, start_v: usize) -> Vec<usize> {
    // graph[v] := vec![(nv0, w0), (nv1, w1), ..., ]
    // 隣接頂点と、その重み
    
    use std::cmp::{Ordering, Reverse};
    // ヒープを使ったダイクストラ法
    // 密グラフではなく、疎グラフっぽいので、ヒープを利用したダイクストラ法で解く必要がある
    // 単純なダイクストラ法 計算量: O(|V|^2)
    // ヒープを使ったダイクストラ法 計算量: O(|E|log|V|)
    //     密グラフ|E| = |V|^2なら、O(|V|^2|log|V|)
    //     疎グラフ|E| = |V|なら、O(|V|log|V|)          ←今回の問題のケース

    const INF: usize = 1 << 60; // usizeが取りうる値は0~2^64。
    let mut distances = vec![INF; graph.len()];
    distances[start_v] = 0;

    // ヒープを使ったダイクストラ法 計算量: O(|E|log|V|)
    // ヒープの中には、到達可能な中で最短距離が未確定な頂点の、頂点番号と距離を格納
    let mut heap = std::collections::BinaryHeap::new();
    heap.push( Reverse((distances[start_v], start_v)));

    while !heap.is_empty() {
        let Reverse((min_dist, v)) = heap.pop().unwrap();
        
        // ゴミであるときはリトライ (ヒープの中には、同じ頂点vでも、更新前のd'[v]と更新後のd''[v]が格納されてしまう。ヒープのキー値d[v]を更新する代わりに、更新したd*[v]を挿入し続けるため)
        if min_dist > distances[v] {continue}

        // v を始点とした辺の緩和
        for &(nv, weight) in graph[v].iter() {
            // 緩和できる場合
            if distances[nv] > distances[v] + weight {
                // 緩和
                distances[nv] = distances[v] + weight;
                // 到達可能で最短距離が未確定な頂点リストに追加
                heap.push( Reverse((distances[nv], nv)));
            }
        }
    }
    return distances
}
