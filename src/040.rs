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
    let mut graph = vec![vec![]; n+2];
    for i in 0..n {
        // [2-1] startとn個の頂点には、家を訪れた時に得られる報酬Ai円を容量とする辺:s -> i を張る。
        graph[start].push(vec![i+1, a[i]]);
        // [2-2] terminatlとn個の頂点には、家を訪れない時に払わなくて住む入場料W円を容量とする辺:i -> t を張る。
        graph[i+1].push(vec![terminatl, w]);
    }
    
    let infinity = 1 << 60; // 無限大の容量を表現 (他の全ての辺の和よりも大きければ良い。計算時にオーバーフローしない程度に注意。)
    for i in 0..n {
        input! {
            k_i: usize,
            mut c_i: [usize; k_i]
        }
        for j in 0..k_i {
            // [2-3]. 家c_i[j]の鍵が、家i+1にある場合、容量無限大の辺: c_i[j] -> i+1 を張る。
            graph[c_i[j]].push(vec![i+1, infinity]);
        }
    }
    // s-tカットの最小容量を求める
    let min_cut = ford_fulkerson::get_maximum_flow(&graph, start, terminatl);
    // println!("min_cut={}", min_cut);
    let mut ans = 0;
    for i in 0..n {
        ans += a[i] as isize;
    }
    // ans -= (n * w) as isize; // 入場料はカットの容量に含まれているので、ここで2重に差し引く必要はない
    ans -= min_cut;
    println!("{}", ans);


}

// フォード・ファルカーソン法 (実装はけんちょん本を参考にした)
// 最大流(=最小カット)を求めるアルゴリズム. 計算量(O(Flow_max * (V + E))).
// Flow_maxが"数値"を表す量なので、多項式アルゴリズムではない。(VやEは"個数"を表す。)
// 擬多項式時間(pseudo-polynominal time): 数値に関して多項式であるが、実際には多項式時間ではない計算量

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
mod ford_fulkerson {
    // フォード・ファルカーソン法で(頂点start-頂点terminal間の)最大流を求めるメソッド
    pub fn get_maximum_flow(original_graph: &Vec<Vec<Vec<usize>>>, start: usize, terminal: usize) -> isize {
        // original_graphは頂点uが、i=1,...k(kは頂点uの持つエッジの本数)について[頂点vi, 容量ci]を要素とした配列を持つように定義すること
        // original_graph[u] := [[v1, c1], [v2, c2], ..., [vk, ck]]
        
        // 元のグラフから残余グラフを生成
        let mut residual_graph = ResidualGraph::new(original_graph);
        let infinite = std::isize::MAX;
        let mut flow_sum = 0;

        // 残余グラフにs-tパスが見つからなくなるまで反復
        loop {
            // O(V)
            let mut seen = vec![false; residual_graph.len()];
            // O(V+E)
            let flow = dfs(&mut residual_graph, &mut seen, start, terminal, infinite);
            flow_sum += flow;

            // s-tパスが見つからなかったら終了
            if flow == 0 {break}
        }

        return flow_sum
    }
    // 残余グラフ上でs-tパスを見つける (深さ優先探索)
    // 返り値はs-tパス上の全辺の容量の最小値 (見つからなかったら0).(終端まで辿りつくことができた水の流量でもある.)
    // residual_graph: 残余グラフ
    // seen: 各頂点が訪問済みかを記録
    // v: これから訪問する頂点v
    // terminal: 終端の頂点t
    // temporary_flow: sからvへ到達した過程の各辺の容量の最小値
    fn dfs(residual_graph: &mut ResidualGraph, seen: &mut Vec<bool>, v: usize, terminal: usize, temporary_flow: isize) -> isize {
        // 終端tに達したらreturn
        if v == terminal {
            return temporary_flow
        }

        // 深さ優先探索
        seen[v] = true;
        for i in 0..residual_graph[v].len() {
            let next_v = residual_graph[v][i].to;
            let next_capacity = residual_graph[v][i].capacity;
            let next_rev_index: usize = residual_graph[v][i].reverse_edge_index;
            if seen[next_v] {continue}
            
            // 容量0の辺は存在しない
            if next_capacity == 0 {continue}

            // s-tパスを探す
            // 見つかったらflowはパス上の最小容量(終端まで辿りつくことができた水の流量でもある)
            // 見つからなかったらflow=0
            let flow = dfs(residual_graph, seen, next_v, terminal, std::cmp::min(temporary_flow, next_capacity));

            // s-tパスが見つからなかったら次辺を試す
            if flow == 0 {continue}

            // 辺eに容量flowのフロー探す
            run_flow(residual_graph, v, i, next_v, next_rev_index, flow);

            // s-tパスを見つけたら、パス上最小容量を流す
            return flow
        }

        // s-tパスが見つからなかったことを示す
        return 0
    }

    // 辺e=(u,v)に流量fのフローを流す。
    // <=> 辺e=(u,v)の容量をfだけ減少させる　かつ
    //     逆辺e'=(v,u)の流量を増やす
    fn run_flow(residual_graph: &mut ResidualGraph, u: usize, u2v_edge_index: usize, v: usize, v2u_edge_index: usize, flow: isize) {
        // 順方向の辺の容量を減らす
        residual_graph[u][u2v_edge_index].capacity -= flow;

        // 逆辺の容量を減らす
        residual_graph[v][v2u_edge_index].capacity += flow;

        // // 順方向の辺の容量を減らす
        // forward_edge.capacity -= flow;
        
        // // 逆辺の容量を減らす
        // backward_edge.capacity += flow;
    }

    // グラフの辺
    #[derive(Debug, Clone, PartialEq, Eq)]
    struct Edge {
        from: usize,        // 辺の始点
        to: usize,          // 辺の終点
        capacity: isize,    // 辺の容量
        reverse_edge_index: usize,  // 逆辺(to->from)のgraph[to]におけるindex
    }

    // 残余グラフ
    #[derive(Debug, Clone, PartialEq, Eq)]
    struct ResidualGraph {
        data: Vec<Vec<Edge>>
    }
    impl ResidualGraph {
        fn new(original_graph: &Vec<Vec<Vec<usize>>>) -> Self {
            let mut residual_graph = vec![vec![]; original_graph.len()];
            for from in 0..original_graph.len() {
                for j in 0..original_graph[from].len() {
                    // 頂点fromから頂点toへ容量capacityの辺を張る
                    let to = original_graph[from][j][0];
                    let capacity = original_graph[from][j][1] as isize;
                    let reverse_edge_index = residual_graph[to].len();
                    residual_graph[from].push(
                    Edge {
                            from: from, 
                            to: to, 
                            capacity: capacity, 
                            reverse_edge_index: reverse_edge_index
                        }
                    );
                    let edge_index = residual_graph[from].len();
                    // 逆辺も張る (toからfromへも容量0の辺を張って初期化)
                    residual_graph[to].push(
                    Edge {
                            from: to, 
                            to: from, 
                            capacity: 0, 
                            reverse_edge_index: edge_index
                        }
                    );
                }
            }
            ResidualGraph {
                data: residual_graph
            }
        }
        
        fn len(&self) -> usize {
            self.data.len()
        }
    }

    // インデックスアクセスを可能にする為に、Indexトレイトを実装
    impl std::ops::Index<usize> for ResidualGraph {
        // Outputは、Indexトレイトがインデックスアクセスの結果として返す値の型を表しています。
        // Output型を指定することで、indexメソッドが返す値の型が明示され、コンパイラはその型に基づいて必要なチェックや推論を行うことができます。
        // この関連型を指定することで、Indexトレイトを実装する際により柔軟に異なる型を返すことも可能になります。
        type Output = Vec<Edge>;

        fn index(&self, index: usize) -> &Self::Output {
            &self.data[index]
        }
    }
    impl std::ops::IndexMut<usize> for ResidualGraph {
        fn index_mut(&mut self, index: usize) -> &mut Vec<Edge> {
            &mut self.data[index]
        }
    }
}

