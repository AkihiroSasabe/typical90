use proconio::input;
use itertools::Itertools;
use std::cmp::{max, min};
use std::cmp::Ordering;
use std::collections::{VecDeque, HashMap, BTreeMap};
use std::collections::BinaryHeap;
use std::hash::Hash;
use proconio::marker::Chars;
fn main() {
    input! {
        n: usize,
        t: isize,
        axy: [(isize, isize); n],
        bxy: [(isize, isize); n],
    }
    // 飛行方向
    let dir_x = vec![1, 1, 0, -1, -1, -1,  0,  1];
    let dir_y = vec![0, 1, 1,  1,  0, -1, -1, -1];
    // 座標(x,y)をキー、向きdをvalueにしたHashMap
    let mut dir_hash: HashMap<(isize, isize), usize> = HashMap::new();
    for i in 0..8 {
        dir_hash.insert((dir_x[i], dir_y[i]), i);
    }

    // bを全部hashに入れていく
    let mut b_hash = HashMap::new();
    for i in 0..n {
        b_hash.insert(bxy[i], i);
    }

    // 有向グラフ 飛行機iと、到達可能な位置+n をエッジとする有向グラフ
    let mut graph = vec![vec![]; 2*n];
    for i in 0..n {
        for j in 0..dir_x.len() {
            // 飛行機iが方向jにT秒間飛んだ時の到達位置
            let destination = (axy[i].0 + t * dir_x[j], axy[i].1 + t * dir_y[j]);
            // aのt秒後の行き先に、bがあれば有向グラフのエッジを張る
            if b_hash.contains_key(&destination) {
                graph[i].push(b_hash[&destination] + n);
            }
        }
    }

    // 2部マッチング問題を解く
    // 全辺が容量1の残余グラフ(residual graph)の形成
    // let mut residual_graph = vec![vec![]; 2*n + 2];
    let mut residual_graph = flow::ResidualGraph::new(2*n + 2);
    for v in 0..graph.len() {
        for next_v in graph[v].iter() {
            residual_graph.add_edge(v, *next_v, 1);
        }
    }
    // 始点の頂点sと終点の頂点tを追加
    let start_v = 2 * n;
    let terminal_v = 2 * n + 1;
    for v in 0..n {
        // start_v から、v=0,1,...n-1 に向かう辺を追加
        residual_graph.add_edge(start_v, v, 1);
        // v=n,n+1,...2*n-1 から terminal_v に向かう辺を追加
        residual_graph.add_edge(v + n, terminal_v, 1);
    }

    // start_v から terminal_v への最大フローを求める
    let max_flow = flow::dinic::get_maximum_flow(start_v, terminal_v, &mut residual_graph);

    if max_flow < n as isize {
        // n機の飛行機と、n個の終点B繋ぐフローがn未満なら、全機と全目的地が1対1でマッチングする組み合わせが存在しない
        println!("No");
    }
    else {
        println!("Yes");
        // A -> B に向かう全エッジについて全探索O(8*N*2)
        for v in 0..n {
            // vから出るエッジの本数は最大でも8本
            for j in 0..residual_graph[v].len() {
                // 飛行機vのT秒後の位置(インデックス)
                let terminal = residual_graph[v][j].to;
                // 水が流れているエッジ(<=> 容量が1->0になったエッジ) で かつ エッジ先端が対岸のN個(start_vとterminal_vを回避) なエッジ
                if residual_graph[v][j].capacity == 0 && terminal < 2*n {
                    
                    // T秒後の飛行機の座標(BXi, BYi)
                    let terminal_x = bxy[terminal - n].0;
                    let terminal_y = bxy[terminal - n].1;

                    // 0秒後の飛行機の座標(AXv, AYv)
                    let start_x = axy[v].0;
                    let start_y = axy[v].1;

                    // 飛行方向
                    let dx = (terminal_x - start_x) / t; 
                    let dy = (terminal_y - start_y) / t; 
                    let dir_index = dir_hash.get(&(dx, dy)).unwrap();
                    
                    print!("{} ", dir_index + 1);
                }
            }
        }
    }
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
    
    // ディニック法: 最大流 を O(V^2 * E) で求めるアルゴリズム (Dinic's algorithm. 著者はDinitzだが、弟子が授業で綴りを間違えて広まった)
    // Googleの競技プログラマーによる解説動画 ( https://www.youtube.com/watch?v=M6cm8UeeziI )が直感的に分かりやすい。実装は蟻本を参考。
    // 同じく最大流を求める Ford-Fulkerson法 も知っていると理解が捗る(特に残余グラフ)。早稲田大学の早水桃子准教授による解説動画が分かりやすい。 
    // https://www.youtube.com/watch?v=TjOA3vK0HCI&list=PLCo60G1m_ibpJgfB4WcGwWybC6sfyawoL&index=10 (9:34~)

    // ポイント1: 増加パスの探索は、最短のs-tパスだけに注目
    // Ford–Fulkerson法では増加パスを愚直にDFSで探していたが、探索対象をs-tが最短距離になるパスだけに絞れば高速になる。
    // 増加パスの流量はs-tパス上のボトルネック(=最小容量)で決まるが、増加パスを探す際に後方や横へ向かうエッジは調べてもボトルネックを小さくしてしまうだけなので除去するのが有効である。
    // (東にあるカフェを探す場合、西側に歩くことは無意味であるというアナロジー)

    // ポイント2: DFSのbacktrackの時にdead ends(行き止まり, 袋小路)を枝刈り 
    // backtrack: 〔来た時と〕同じ道を引き返すこと
    // 1975年にShimon EvenとAlon Itaiがdead endsが提案。同じ辺を調べるという操作がなくなるので高速になる。

    // 手順
    // 1. BFSでソースsからの最短距離を全頂点に対して記録し、"シンク方向(距離が増える)" かつ "cap - flow > 0" のエッジだけを残す(これをレベルグラフという。実装上は残余グラフ上で仮想的なレベルグラフの制約を再現する)
    // 2. レベルグラフ上でシンクtに到達できなくなれば、その時点でのフローの合計が最大流となり、アルゴリズムを終了する。
    // 3. レベルグラフ上でDFSを行って1本のaugmenting path (s-tパスで流量が増加するパス)を見つけて、残余グラフに流水する。流量はaugmenting path上のボトルネック(=最小容量=最大流)に等しい。
    // 4. DFS時に遭遇したレベルグラフ上dead ends(行き止まり)は適宜、枝刈りしていく
    // 5. レベルグラフ上でブロッキングフローに達する(=これ以上増加パスが見つからなくなる)まで、このDFSを何度も繰り返し増加パスのフローを積算していく。
    // 6. 1-5 を繰り返す (BFSでレベルグラフを再構築するとき、エッジの残容量が前回構築したレベルグラフと異なる)

    // 計算量 O(V^2E)
    // 1. 外側のループはO(V): 
    // レベルグラフを再構築するたびに、s-t間の距離は1以上増加する(最短のs-t間の距離は流水されて残容量がなくなりレベルグラフ上でエッジが切れる)。
    // s-tパスの最長距離はV-1なので、最大でV-1回BFSを構築すれば最大流が求まるのでO(V)
    //
    // 2. 1回のレベルグラフの構築:
    // BFSなのでO(E)
    //
    // 3. 内側のループはO(VE)
    // 内側のループは、DFSを複数回繰り返してブロッキングフローに到達するまでの計算量。 <- 証明は謎
    // dynamic treesを使うとO(ElogV)でできる
    // 
    // 4. トータルの計算量は、
    // O(V*(E+VE)) = O(VE+V^2E) = O(V^2E)
    //
    // 5. 全辺の容量が1のときのトータルの計算量は、
    // O(min(E*V^(2/3), E^(3/2)))。
    // 内側のループ(ブロッキングフローを見つけるループ)がO(E)になるため。
    pub mod dinic {
        use super::ResidualGraph;
        // 始点sから終点tの最大流を求める関数
        pub fn get_maximum_flow(
            s: usize, 
            t: usize, 
            residual_graph: &mut ResidualGraph
        ) -> isize {
            // 最大流の初期化
            let mut flow_sum = 0;
        
            // 無限大の表現
            let infinite = std::isize::MAX;
        
            loop {
                // 1. sからtへ向かう辺(sからの距離が増加する向きの辺)のみで構成されたグラフをbfsで取得 O(E)
                let level: Vec<isize> = bfs(s, &residual_graph); // level[v] := 頂点sから頂点vまでの距離
        
                // 3. sからtへのパスが存在しなくなったとき最大流が確定
                if level[t] < 0 {
                    return flow_sum;
                }
        
                // dead_end_edge_num[v] := 頂点vのエッジのうち、袋小路である(これ以上流量が増やせない)ことが確定したエッジの本数を格納 (探索はindexが若い辺から行うので、index=dead_end_edge_num[v]-1まで探索が完了していると考えて良い)
                let mut dead_end_edge_num: Vec<usize> = vec![0; residual_graph.len()]; // DFSのBackTrack(〔来た時と〕同じ道を引き返す)時に、DeadEnd(行き止まり、袋小路)を重複して探索しない為の対応
                loop {
                    // 2. 1本の augmenting path (s-tパスで流量が増加するパス) をDFSで取得し、フローを流す O(V-1)
                    let flow = dfs(s, t, infinite, residual_graph, &mut dead_end_edge_num, &level);
                    if flow > 0 {
                        flow_sum += flow;
                    }
                    else {
                        //augmenting paths(増加パス)が存在しないときは、ループ終了
                        break
                    }
                }
            }
        }

        // BFSで始点 start_v からの最短距離 level[v] を更新. (普通のBFSと異なる点は、容量が0の辺は遷移不可としている点)
        fn bfs(
            start_v: usize,                     // 始点
            residual_graph: &ResidualGraph      // 残余グラフ
        ) -> Vec<isize> {                       // level[v] := 始点sから頂点vまでの最短距離 (未確定の頂点は-1で初期化済み)
            let mut level: Vec<isize> = vec![-1; residual_graph.len()];
            let mut todo = std::collections::VecDeque::new();
            todo.push_back(start_v);
            level[start_v] = 0;

            while todo.len() != 0 {
                let v = todo.pop_front().unwrap();
                for edge in residual_graph[v].iter() {
                    if edge.capacity > 0 && level[edge.to] == -1 {
                        // 流す容量が残っている かつ 未訪問のedge
                        level[edge.to] = level[v] + 1; // edgeの先端の頂点の距離を更新
                        todo.push_back(edge.to);
                    }
                }
            }
            return level
        }

        // DFSで残余グラフ上の1本の augmenting path (s-tパスで流量が増加するパス) を見つける。
        // 返り値は見つけたaugmenting pathの流量(=s-tパス上の最小容量)で、見つからなかったら流量0を返す。
        // 見つかったら残余グラフにそのflowを流して更新する。
        fn dfs(
            v: usize,                               // これから調査する頂点v 
            terminal: usize,                        // 終点の頂点t
            temporary_flow: isize,                  // s-v path における最大流量 (s-t pathの途中)
            residual_graph: &mut ResidualGraph,     // 残余グラフ
            dead_end_edge_num: &mut Vec<usize>,     // dead_end_edge_num[v] := 頂点vから出る辺のうち、袋小路である(これ以上流量が増やせない)ことが確定したエッジの本数
            level: &Vec<isize>                      // level[v] := 始点sから頂点vまでの最短距離 (未確定の頂点は-1で初期化済み)
        ) -> isize {
            if v == terminal {
                // 終点tに到達 <=> augmenting path が1本見つかったので、そのflowを流す
                return temporary_flow
            }
            // 頂点vから湧き出る全てのエッジを探索 (袋小路が確定したエッジは探索スキップ)
            while dead_end_edge_num[v] < residual_graph[v].len() {
                // 次に調査するエッジのindex
                let edge_index: usize           = dead_end_edge_num[v];
                let next_v: usize               = residual_graph[v][edge_index].to;
                let reverse_edge_index: usize   = residual_graph[v][edge_index].reverse_edge_index;

                // 流す容量が残っていて、かつ、sからt方向へ進むエッジ
                if residual_graph[v][edge_index].capacity > 0 && level[v] < level[next_v] {
                    // 1本の augmenting path の流量
                    let flow = dfs(next_v, terminal, std::cmp::min(temporary_flow, residual_graph[v][edge_index].capacity), residual_graph, dead_end_edge_num, level);
                    if flow > 0 {
                        // augmenting pathが見つかったら(flow > 0)、残余グラフの辺にも水を流して状態更新
                        residual_graph[v][edge_index].capacity -= flow;              // 順方向の辺の容量を減らす
                        residual_graph[next_v][reverse_edge_index].capacity += flow; // 逆辺の容量を増やす

                        // augmenting pathが見つかったら(flow > 0)、その流量を返す
                        return flow
                    }
                }
                // 上でflowをreturn出来なかった <=> 現在調査中のvから湧き出るエッジが袋小路であることが確定
                dead_end_edge_num[v] += 1; // 袋小路が確定したエッジの本数を1本増加
            }
            // 終点に到達不可 <=> augmenting path が見つからなかったので、flow=0を返す
            return 0;
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
            // FordFulkerson => ford_fulkerson::get_maximum_flow(start, terminal, &mut residual_graph), // 040.rsを参照
            Dinic => dinic::get_maximum_flow(start, terminal, &mut residual_graph), // 077.rsを参照
            FordFulkerson => 0,
        };
        return maximum_flow
    }
    // 最大流のアルゴリズムは2種類ある
    enum Algorithm {
        FordFulkerson,
        Dinic
    }
}