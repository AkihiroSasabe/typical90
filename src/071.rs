use proconio::input;
use itertools::Itertools;
use std::cmp::{max, min};
use std::cmp::Ordering;
use std::collections::{VecDeque, HashMap};
use std::collections::BinaryHeap;
use std::hash::Hash;
use proconio::marker::Chars;


// 参考: 右の提出を参考に実装。(おそらくE8氏の回答をRustに翻訳したもの): https://atcoder.jp/contests/typical90/submissions/35618307
fn main() {
    input! {
        n: usize,
        m: usize,
        k: usize
    }

    let mut a = vec![];
    let mut b = vec![];
    let mut graph = vec![vec![]; n];
    // 全頂点の入次数を格納した配列
    let mut in_degrees = vec![0; n];
    for i in 0..m {
        input! {
            a_i: usize,
            b_i: usize,
        }
        a.push(a_i - 1);
        b.push(b_i - 1);
        graph[a_i - 1].push(b_i - 1);
        in_degrees[b_i - 1] += 1;
    }
    
    // todo := 現時点で入次数が0の頂点の集合
    let mut todo = vec![];
    for i in 0..n {
        if in_degrees[i] == 0 {
            todo.push(i);
        }
    }

    // 現在構築中の順列
    let mut candidate = vec![];
    // N文字全て揃った順列を格納する配列
    let mut candidates = vec![];

    // DFSでトポロジカルソートの全探索
    dfs(&graph, &mut in_degrees, &mut todo, &mut candidate, &mut candidates, k);

    if candidates.len() == k {
        for i in 0..k {
            for j in 0..candidates[i].len() {
                let v = candidates[i][j];
                print!("{} ", v + 1);
            }
            println!("");
        }
    }
    else {
        println!("-1");
    }
}


// トポロジカルソートの全パターンを網羅的に探索
// BFSのトポロジカルソートと同様に、入次数が0のものから探索していく
fn dfs (
    graph: &Vec<Vec<usize>>,            // 要素同士の前後関係を表す有向グラフ
    in_degrees: &mut Vec<usize>,        // 各要素の入次数
    todo: &mut Vec<usize>,              // 入次数が0の要素の集合で、構築中の順列に追加可能 (cand)
    candidate: &mut Vec<usize>,         // 現在構築中の順列 (perm)
    candidates: &mut Vec<Vec<usize>>,   // 構築完了した順列の集合。本問はこれを回答する。 (res)
    k: usize,                           // 回答に必要な順列の個数
) -> bool {                             // 探索(DFS)を続ける場合はtrue, そうでなければfalse

    // 順列がk個見つかったら探索(DFS)を終了
    if candidates.len() >= k {
        return false
    }
    // 現在構築中の順列が、N文字全て揃って構築完了した場合
    else if candidate.len() == graph.len() {
        // 構築完了済みの順列の集合に追加
        candidates.push(candidate.clone());
        return true
    }
    // 現在構築中の順列に追加可能な要素がなくなった場合
    else if todo.len() == 0 {
        // 探索(DFS)を終了
        return false
    }
    else {
        // 現在構築中の順列に、追加可能な要素を全通り追加して試す
        for i in (0..todo.len()).rev() {

            // 追加可能な要素vを、実際に追加 (追加するために元のtodoから取り出す) O(N)
            let v = todo.remove(i);
            for &u in &graph[v] {
                in_degrees[u] -= 1;
                if in_degrees[u] == 0 {
                    todo.push(u);
                }
            }
            candidate.push(v);
            
            // 構築中の順列candidateに1個要素を追加して、次の状態へ遷移(DFS)
            if !dfs(graph, in_degrees, todo, candidate, candidates, k) {
                // 探索(DFS)を終了
                return false
            }
            
            // 追加した要素vを、元に戻す
            candidate.pop();
            for &u in &graph[v] {
                if in_degrees[u] == 0 {
                    todo.pop();
                }
                in_degrees[u] += 1;
            }
            todo.insert(i, v);
        }
        return true
    }
}



/// The function `topological_sort_by_in_degree_wrapper` takes a graph represented as an adjacency list
/// and returns a topologically sorted list of vertices based on their in-degrees.
/// 
/// Arguments:
/// 
/// * `graph`: The `graph` parameter is a vector of vectors representing a directed graph. Each element
/// of the outer vector represents a vertex in the graph, and the inner vectors represent the outgoing
/// edges from that vertex. The indices of the outer vector correspond to the vertex numbers, and the
/// values of the inner vectors represent
/// 
/// Returns:
/// 
/// The function `topological_sort_by_in_degree_wrapper` returns an `Option<Vec<usize>>`.
fn topological_sort_by_in_degree_wrapper(graph: &Vec<Vec<usize>>) -> Option<Vec<usize>> {
    // 入次数を取得して、有向グラフをトポロジカルソートする関数

    // 頂点の入次数を格納していく
    let mut in_degrees = vec![0; graph.len()];
    for v in 0..graph.len() {
        for next_v in graph[v].iter() {
            in_degrees[*next_v] += 1;
        }
    }
    // トポロジカルソート開始
    let topological_sorted_list = topological_sort_by_in_degree(graph, &mut in_degrees);
    return topological_sorted_list
}

/// The function `topological_sort_by_in_degree` performs a topological sort on a directed acyclic graph
/// (DAG) using the in-degree of each vertex.
/// 
/// Arguments:
/// 
/// * `graph`: The `graph` parameter is a vector of vectors representing the adjacency list of a
/// directed graph. Each element of the outer vector represents a vertex, and the inner vector
/// represents the vertices that are adjacent to the corresponding vertex.
/// * `in_degrees`: `in_degrees` is a mutable reference to a vector that represents the in-degrees of
/// each vertex in the graph. The in-degree of a vertex is the number of edges that point into that
/// vertex.
/// 
/// Returns:
/// 
/// The function `topological_sort_by_in_degree` returns an `Option<Vec<usize>>`.
fn topological_sort_by_in_degree(graph: &Vec<Vec<usize>>, in_degrees: &mut Vec<usize>) -> Option<Vec<usize>> {
    // 入次数を元に有向グラフをトポロジカルソートする関数
    // 入次数（頂点に入ってくる辺の数）が0になったものを次々にキューに入れる

    // todo := 現時点で入次数が0の頂点の集合
    let mut todo = VecDeque::new();
    for i in 0..graph.len() {
        if in_degrees[i] == 0 {
            todo.push_back(i);
        }
    }

    let mut topological_sorted_list = vec![];
    while todo.len() != 0 {
        let v = todo.pop_front().unwrap();
        topological_sorted_list.push(v);
        for next_v in graph[v].iter() {
            // ソート済みリストに追加された頂点vは、グラフから削除したことにして、
            // 隣接する頂点の入次数を更新していく
            in_degrees[*next_v] -= 1;
            if in_degrees[*next_v] == 0 {
                todo.push_back(*next_v);
            }
        }
    }

    // 閉路検出判定
    let is_cycle = topological_sorted_list.len() != graph.len();
    if is_cycle {
        // 有向グラフが閉路を持っていたら(!=DAG)、トポロジカルソートが出来ない
        return None
    }
    else {
        // DAGならトポロジカルソート成功
        return Some(topological_sorted_list)
    }
}

