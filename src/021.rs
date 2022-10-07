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
        m: usize,
    }
    let mut graph = vec![vec![]; n];
    let mut reverse_graph = vec![vec![]; n];
    for i in 0..m {
        input! {
            a_i: usize,
            b_i: usize
        }
        graph[a_i - 1].push(b_i - 1);
        reverse_graph[b_i - 1].push(a_i - 1);
    }

    // 強連結成分分解
    // 2回DFSをする
    // 1回目: ただのトポロジカルソート。帰りがけ順に番号を振っていく。
    // 2回目: 辺を逆向きにする。大きい番号から開始。たどり着ける部分が、強連結成分。たどり着けなければ新しい強連結成分。
    let scc_list = decompositon_of_strongly_connected_components(&graph, &reverse_graph, n);

    let mut answer = 0;
    for scc in scc_list {
        // println!("{:?}", scc);
        let scc_size = scc.len();
        if scc_size < 2 {continue}
        // nC2
        answer += scc_size * (scc_size - 1) / 2; 
    }
    println!("{}", answer);

}


// 1回目のDFS
fn dfs1(graph: &Vec<Vec<usize>>, v: usize, seen: &mut Vec<bool>, sorted_list: &mut Vec<usize>) {
    seen[v] = true;
    for next_v in graph[v].iter() {
        if seen[*next_v] {continue}
        dfs1(graph, *next_v, seen, sorted_list);
    }
    sorted_list.push(v);
}

// 2回目のDFS。トポロジカルソートした番号の逆順から攻める。
fn dfs2(graph: &Vec<Vec<usize>>, v: usize, seen: &mut Vec<bool>, scc: &mut Vec<usize>) {
    seen[v] = true;
    for next_v in graph[v].iter() {
        if seen[*next_v] {continue}
        dfs2(graph, *next_v, seen, scc);
    }
    scc.push(v);
}


// 強連結成分分解 (蟻本p285~p288) 計算量O(E)
fn decompositon_of_strongly_connected_components(graph: &Vec<Vec<usize>>, reverse_graph: &Vec<Vec<usize>>, v_num: usize) -> Vec<Vec<usize>>{

    // 1回目のDFS: トポロジカルソートする
    let mut reverse_topological_sorted_list = vec![];
    let mut seen = vec![false; v_num];
    for v in 0..v_num {
        if seen[v] {continue}
        dfs1(graph, v, &mut seen, &mut reverse_topological_sorted_list);
    }

    // 2回目のDFS: グラフの辺を逆向きにして、たどり着ける頂点を強連結成分としてまとめる
    let mut scc_list = vec![];
    let mut seen = vec![false; v_num];
    reverse_topological_sorted_list.reverse();
    let topological_sorted_list = reverse_topological_sorted_list;
    for v in topological_sorted_list {
        if seen[v] {continue}
        let mut strongly_connected_components = vec![];
        dfs2(&reverse_graph, v, &mut seen, &mut strongly_connected_components);
        scc_list.push(strongly_connected_components);
    }

    return scc_list
}
