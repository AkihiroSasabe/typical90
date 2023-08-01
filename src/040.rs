use proconio::input;
use itertools::Itertools;
use std::cmp::{max, min};
use std::cmp::Ordering;
use std::collections::{VecDeque, HashMap, BTreeMap};
use std::collections::BinaryHeap;
use proconio::marker::Chars;

fn main() {
    input! {
        n: usize,
        w: usize,
        a: [usize; n],
    }
    // フォードファルカーソン法の実装はけんちょん本、この問題の解き方はtanaka-a(A Tanaka)氏の記事を参考。
    // https://qiita.com/tanaka-a/items/fb8d84c44190c7098047#%E3%83%91%E3%82%BF%E3%83%BC%E3%83%B31%E5%85%AC%E5%BC%8F%E8%A7%A3%E8%AA%AC%E6%96%B9%E5%BC%8F

    // [1]グラフの頂点の定義
    // n個の家に対応する頂点と、2個の頂点s, tからなる有向グラフGを構築。

    // [2]グラフの辺の定義
    // [2-1] sとn個の頂点には、家を訪れた時に得られる報酬Ai円を容量とする辺:s -> i をそれぞれ張る。
    // [2-2] tとn個の頂点には、家を訪れない時に払わなくて住む入場料W円を容量とする辺:i -> t をそれぞれ張る。
    // [2-3] 頂点iの鍵が頂点jにある場合、容量無限大の辺: i -> j を張る。

    // [3]最大集金額を求める方法
    // [3-1]s-tカットの最小容量
    // このグラフGを(訪れる家の集合S, 訪れない家の集合T)に分割することを考える。
    // 集合Sは頂点sに繋がれた頂点達、集合Tは頂点tに繋がれた頂点達である。
    // カット辺の容量の合計値が最小になるようなカット(S, T)を選ぶ。
    // [3-2]最大集金額
    // 集める事ができる最大金額は、全ての家を訪問した時に得られる報酬から、上でも求めたs-tカットの最小容量を差し引いたものとなる。

    // [4] お気持ち
    // [4-1] [2-3]のお気持ち(無限大の容量を張る理由):
    //  (1)iに訪れてjに訪れるような許されない状況をグラフで表現すると、s->i, j->tとなるカットが存在することであり、そのときカット辺はi->jが存在する
    //  最小容量となるs-tカットを選ぶ際に、このカット辺が含まれないようにするために、i->jに無限大の容量の辺を張る必要がある。
    //  ちなみに、j->iに容量無限の辺を張る必要はない。下記参照。
    //  (2)iとjに訪れるケース         <=> s->i, s->j <=> iとjに関するカット辺: 存在しない
    //  (3)iに訪れずjに訪れるケース   <=> s->j, i->t <=> iとjに関するカット辺: j->i (これは許されているケースなので、j->iに容量無限の辺を張る必要はない)
    //  (4)iとjに訪れないケース       <=> i->t, j->t <=> iとjに関するカット辺: 存在しない
    // [4-2] [3]のお気持ち(なぜ最小カット問題に帰結するか): 
    //  iに訪れたときはA[i]円得して、訪れないときは実質的にW円得すると考える。
    //  このうち2つを選ぶことはできないので、A[i]円かW円のどちらかを切り捨てる（カットする）必要がある。
    //  i=1~Nについて切り捨てる金額の合計を最小化すれば、集金額を最大化できるので、これを最小カット問題を解くことと同じである。
    //  最小カット問題は最大流問題と双対問題であり、s-tカットの最小容量はs-tの最大流量と等しく、これはフォードファルカーソン法O(F(E))や、ディニッツ法O(V^2E)やオーリン法O(VE)で解ける

    let start: usize = 0;  // (訪れる頂点集合の代表頂点)
    let terminatl: usize = n + 1; // (訪れない集合)
    let mut residual_graph = flow::ResidualGraph::new(n+2);
    for i in 0..n {
        // [2-1] startとn個の頂点には、家を訪れた時に得られる報酬Ai円を容量とする辺:s -> i を張る。
        residual_graph.add_edge(start, i+1, a[i] as isize);
        // [2-2] terminatlとn個の頂点には、家を訪れない時に払わなくて住む入場料W円を容量とする辺:i -> t を張る。
        residual_graph.add_edge(i+1, terminatl, w as isize);
    }
    
    let infinity: isize = 1 << 60; // 無限大の容量を表現 (他の全ての辺の和よりも大きければ良い。計算時にオーバーフローしない程度に注意。)
    for i in 0..n {
        input! {
            k_i: usize,
            mut c_i: [usize; k_i]
        }
        for j in 0..k_i {
            // [2-3]. 家c_i[j]の鍵が、家i+1にある場合、容量無限大の辺: c_i[j] -> i+1 を張る。
            residual_graph.add_edge(c_i[j], i+1, infinity);
        }
    }
    // s-tカットの最小容量を求める
    // let min_cut = flow::get_maximum_flow_from_weighted_directed_graph_by_ford_fulkerson(start, terminatl, &graph);
    let min_cut = flow::ford_fulkerson::get_maximum_flow(start, terminatl, &mut residual_graph);
    // println!("min_cut={}", min_cut);
    let mut ans = 0;
    for i in 0..n {
        ans += a[i] as isize;
    }
    // ans -= (n * w) as isize; // 入場料はカットの容量に含まれているので、ここで2重に差し引く必要はない
    ans -= min_cut;
    println!("{}", ans);
}

// フローのアルゴリズムに関するモジュール
mod flow {
    // 残余グラフの辺
    #[derive(Debug, Clone, PartialEq, Eq)]
    pub struct Edge {
        pub to: usize,                  // 辺の終点
        pub capacity: isize,            // 辺の容量
        pub reverse_edge_index: usize   // 逆辺(to->from)のgraph[to]におけるindex
    }
    // 残余グラフ
    // フロー F(Ei) / 容量 C(Ei)
    // 容量制限を守ってフローの値を安全に更新するためのアイディア
    // 更新後の順辺の容量: 増やせる量の上限でC(Ei) - F(Ei)
    // 更新後の逆辺の容量: 減らせる量の下限でF(Ei)
    #[derive(Debug, Clone, PartialEq, Eq)]
    pub struct ResidualGraph {
        residual_graph: Vec<Vec<Edge>>
    }
    impl ResidualGraph {
        pub fn new(graph_size: usize) -> Self {
            ResidualGraph {
                residual_graph: vec![vec![]; graph_size]
            }
        }
        // 残余グラフのエッジを追加する関数 (容量0の逆辺も追加されることに注意)
        pub fn add_edge(&mut self, from: usize, to: usize, capacity: isize) {
            // 順方向(頂点fromから頂点to)へ容量capacityの辺を張る
            let reverse_edge_index = self[to].len(); // 後で張る逆辺のindex
            let forward_edge = Edge {to, capacity, reverse_edge_index};
            self[from].push(forward_edge);

            // 逆方向の辺(逆辺)も張る (toからfromへも容量0の辺を張って初期化)
            let edge_index = self[from].len() - 1;
            let reverse_edge = Edge {
                to: from, 
                capacity: 0, 
                reverse_edge_index: edge_index
            };
            self[to].push(reverse_edge);
        }
        fn len(&self) -> usize {
            self.residual_graph.len()
        }
    }
    // インデックスアクセスを可能にする為に、Indexトレイトを実装
    impl std::ops::Index<usize> for ResidualGraph {
        // Outputは、Indexトレイトがインデックスアクセスの結果として返す値の型を表しています。
        // Output型を指定することで、indexメソッドが返す値の型が明示され、コンパイラはその型に基づいて必要なチェックや推論を行うことができます。
        // この関連型を指定することで、Indexトレイトを実装する際により柔軟に異なる型を返すことも可能になります。
        type Output = Vec<Edge>;

        fn index(&self, index: usize) -> &Self::Output {
            &self.residual_graph[index]
        }
    }
    impl std::ops::IndexMut<usize> for ResidualGraph {
        fn index_mut(&mut self, index: usize) -> &mut Vec<Edge> {
            &mut self.residual_graph[index]
        }
    }

    // 普通のグラフ: Vec<Vec<Vec<usize>>> から 残余グラフ ResidualGraph を生成する 
    // (既にグラフが存在する場合のAPI。0から構築するならnew()とadd_edge()を使う。)
    impl From<&Vec<Vec<Vec<usize>>>> for ResidualGraph {
        fn from(weighted_directed_graph: &Vec<Vec<Vec<usize>>>) -> Self {
            // weighted_directed_graph は頂点uが、i=1,...k(kは頂点uの持つエッジの本数)について[頂点vi, 容量ci]を要素とした配列を持つように定義すること
            // weighted_directed_graph[u] := [[v1, c1], [v2, c2], ..., [vk, ck]]
            let mut residual_graph = ResidualGraph::new(weighted_directed_graph.len());
            for from in 0..weighted_directed_graph.len() {
                for j in 0..weighted_directed_graph[from].len() {
                    // 順方向(頂点fromから頂点to)へ容量capacityの辺を張る
                    let to = weighted_directed_graph[from][j][0];
                    let capacity = weighted_directed_graph[from][j][1] as isize;
                    // 残余グラフのエッジ(順方向の辺と逆辺)を張る
                    residual_graph.add_edge(from, to, capacity);
                }
            }
            return residual_graph
        }
    }

    // フォード・ファルカーソン法 (Ford–Fulkerson algorithm. 実装はけんちょん本を参考にした)
    // 最大流(=最小カット)を求めるアルゴリズム. 計算量(O(Flow_max * E)).
    // Flow_maxが"数値"を表す量なので、多項式アルゴリズムではない。(VやEは"個数"を表す。)
    // 擬多項式時間(pseudo-polynominal time): 数値に関して多項式であるが、実際には多項式時間ではない計算量
    // 残余グラフは、早稲田大学の早水桃子准教授による解説動画が分かりやすい。 
    // https://www.youtube.com/watch?v=TjOA3vK0HCI&list=PLCo60G1m_ibpJgfB4WcGwWybC6sfyawoL&index=10 (9:34~)

    // 各辺の容量が1の場合のフォード・ファルカーソン法を考えると良い
    // (<=>2頂点s-t間の辺連結度を求める)
    // 1. パスの本数をfとして、f=0で初期化
    // 2. 残余グラフG'を元のグラフGで初期化
    // 3. while G'において、s-tパスPが存在する
    //        f += 1
    //        残余グラフG'をPに関して更新
    //          (G'のP上の辺を全て逆向きに張り替える)
    // 4. fが最終的に求める辺連結度

    // 次に、各辺の容量を一般化して考える
    // 1. フロー流量FをF=0で初期化
    // 2. 残余グラフG'を元のグラフGで初期化
    // 3. while G'において、s-tパスPが存在する
    //        f = パスP上の各辺の容量の最小値とする
    //        F += f
    //        パスP上に大きさfのフローを流す
    //        残余グラフG'をPに関して更新
    //          (G'上のP上の辺に)
    // 4. Fが求める最大流量
    pub mod ford_fulkerson {
        use super::ResidualGraph;
        // フォード・ファルカーソン法で(頂点start-頂点terminal間の)最大流を求める関数
        pub fn get_maximum_flow(
            start: usize, 
            terminal: usize,
            residual_graph: &mut ResidualGraph
        ) -> isize {
            let infinite = std::isize::MAX; // 各augmenting pathの流量の初期値は無限大
            let mut flow_sum = 0;           // 各augmenting pathの流量の総和の初期値は0
            
            // 残余グラフにs-tパスが見つからなくなるまで反復
            loop {
                // O(V)
                let mut seen = vec![false; residual_graph.len()];
                // O(E)
                let flow = dfs(start, terminal, infinite, residual_graph, &mut seen,);
                flow_sum += flow;
    
                // s-tパスが見つからなかったら終了
                if flow == 0 {break}
            }
            return flow_sum
        }
        
        // DFSで残余グラフ上の1本の augmenting path (s-tパスで流量が増加するパス) を見つける。
        // 返り値は見つけたaugmenting pathの流量(=s-tパス上の最小容量)で、見つからなかったら流量0を返す。
        // 見つかったら残余グラフにそのflowを流して更新する。
        fn dfs(
            v: usize,                           // これから調査する頂点v 
            terminal: usize,                    // 終点の頂点t
            temporary_flow: isize,              // s-v path における最大流量 (s-t pathの途中, sからvへ到達した過程の各辺の容量の最小値)
            residual_graph: &mut ResidualGraph, // 残余グラフ
            seen: &mut Vec<bool>                // seen[v] := 頂点vが訪問済みかを記録
        ) -> isize {
            if v == terminal {
                // 終点tに到達 <=> augmenting path が1本見つかったので、そのflowを流す
                return temporary_flow
            }
            seen[v] = true;
    
            // 頂点vから湧き出る全てのエッジを探索
            for edge_index in 0..residual_graph[v].len() {
                let next_v: usize               = residual_graph[v][edge_index].to;
                let capacity: isize             = residual_graph[v][edge_index].capacity;
                let reverse_edge_index: usize   = residual_graph[v][edge_index].reverse_edge_index;
    
                // 訪問済みの頂点はスキップ
                if seen[next_v] {continue}
                
                // 容量0の辺には水を流せないのでスキップ
                if capacity == 0 {continue}
    
                // s-tパスを探す
                // 見つかったらflowはパス上の最小容量(終端まで辿りつくことができた水の流量でもある)
                // 見つからなかったらflow=0
                let flow = dfs(next_v, terminal, std::cmp::min(temporary_flow, capacity), residual_graph, seen);
                if flow > 0 {
                    // augmenting pathが見つかったら(flow > 0)、残余グラフの辺にも水を流して状態更新
                    residual_graph[v][edge_index].capacity -= flow;              // 順方向の辺の容量を減らす
                    residual_graph[next_v][reverse_edge_index].capacity += flow; // 逆辺の容量を増やす
    
                    // augmenting pathが見つかったら(flow > 0)、その流量を返す
                    return flow
                }
            }
            // 上でflowをreturn出来なかった <=> augmenting path が見つからなかったので、flow=0を返す
            return 0
        }
    }

    // 重み付き有向グラフ から 残余グラフ を生成し、最大流を求める関数
    // 既に重み付き有向グラフがある場合はこのAPIを使い、ない場合は、 get_maximum_flow() を使うこと)
    // FordFulkerson法
    pub fn get_maximum_flow_from_weighted_directed_graph_by_ford_fulkerson(
        start: usize, 
        terminal: usize,
        weighted_directed_graph: &Vec<Vec<Vec<usize>>>
    ) -> isize {
        get_maximum_flow_by_weighted_directed_graph(start, terminal, weighted_directed_graph, Algorithm::FordFulkerson)
    }
    // Dinic法
    pub fn get_maximum_flow_from_weighted_directed_graph_by_dinic(
        start: usize, 
        terminal: usize,
        weighted_directed_graph: &Vec<Vec<Vec<usize>>>
    ) -> isize {
        get_maximum_flow_by_weighted_directed_graph(start, terminal, weighted_directed_graph, Algorithm::Dinic)
    }
    // FordFulkerson法とDinic法の共通処理
    fn get_maximum_flow_by_weighted_directed_graph (
        start: usize, 
        terminal: usize,
        weighted_directed_graph: &Vec<Vec<Vec<usize>>>,
        algorithm: Algorithm
    ) -> isize {
        // weighted_directed_graph は頂点uが、i=1,...k(kは頂点uの持つエッジの本数)について[頂点vi, 容量ci]を要素とした配列を持つように定義すること
        // weighted_directed_graph[u] := [[v1, c1], [v2, c2], ..., [vk, ck]]
        
        // 元のグラフから残余グラフを生成
        let mut residual_graph = ResidualGraph::from(weighted_directed_graph);
        // 最大流の取得
        let maximum_flow = match algorithm {
            FordFulkerson => ford_fulkerson::get_maximum_flow(start, terminal, &mut residual_graph), // 040.rsを参照
            // Dinic => dinic::get_maximum_flow(start, terminal, &mut residual_graph), // 077.rsを参照
            Dinic => 0,
        };
        return maximum_flow
    }
    // 最大流のアルゴリズムは2種類ある
    enum Algorithm {
        FordFulkerson,
        Dinic
    }
}