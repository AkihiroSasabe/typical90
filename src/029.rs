fn main() {
    // 遅延評価セグメント木の動作確認用のテスト問題
    // RMQ and RAQ  ([1]区間加算更新、区間最小値取得)   https://judge.u-aizu.ac.jp/onlinejudge/description.jsp?id=DSL_2_H&lang=ja
    // ABC327F      ([2]区間加算更新、区間最大値取得)   https://atcoder.jp/contests/abc327/tasks/abc327_f
    // RSQ and RAQ  ([3]区間加算更新、区間和取得)       https://judge.u-aizu.ac.jp/onlinejudge/description.jsp?id=DSL_2_G&lang=ja
    // ModInt + RSQ and RAQ  ([3-mod] ModInt区間加算更新、区間和取得)       https://atcoder.jp/contests/abc179/tasks/abc179_d
    // RMQ and RUQ  ([4]区間代入更新、区間最小値取得)   https://judge.u-aizu.ac.jp/onlinejudge/description.jsp?id=DSL_2_F&lang=ja
    // 典型29は     ([5]区間代入更新、区間最大値取得)   https://atcoder.jp/contests/typical90/tasks/typical90_ac
    // RSQ and RUQ  ([6]区間代入更新、区間和取得)       https://judge.u-aizu.ac.jp/onlinejudge/description.jsp?id=DSL_2_I&lang=ja 

    solve_5_rass_rsam_typical_no29();   // 2024-01-17 実装
    // solve_2_rinc_rmax_abc327f();        // 2023-11-12 実装
    // solve_3_rinc_rsum_aoj();            // 2024-01-05 実装
    // solve_1_rinc_rmin_aoj();            // 2024-01-18 実装
    // solve_4_rass_rmin_aoj();            // 2024-01-20 実装
    // solve_6_rass_rsum_aoj();            // 2024-01-20 実装
    // solve_3mod_rinc_rsum_abc179d();     // 2024-03-29 実装
}

fn solve_5_rass_rsam_typical_no29() {
    // 典型29: https://atcoder.jp/contests/typical90/tasks/typical90_ac
    // [5]区間代入、区間最大値取得 のテスト
    let wn: Vec<usize> = read_vec_usize();
    let w = wn[0];
    let n = wn[1];

    let mut l = vec![];
    let mut r = vec![];
    for i in 0..n {
        let lr: Vec<usize> = read_vec_usize();
        let l_i = lr[0];
        let r_i = lr[1];
        l.push(l_i - 1);
        r.push(r_i - 1);
    }

    // こっちでもいい
    // let mut lazy_segment_tree = lazy_segment_tree::new_range_assignment_update_and_range_maximum_query(w);
    // // 遅延評価セグメント木の data 配列の初期化
    // lazy_segment_tree.range_update(0, w -1, 0);

    let init_vector = vec![0; w];
    let mut lazy_segment_tree = lazy_segment_tree::new_range_assignment_update_and_range_maximum_query_from_vec(&init_vector);

    for i in 0..n {
        let max_height = lazy_segment_tree.range_query(l[i], r[i]);
        println!("{}", max_height+1);

        // lazy_segment_tree.print_tree();
        // 遅延評価セグメント木なら、O(N * logW)。区間[l[i], r[i]]の更新でO(logW)
        lazy_segment_tree.range_update(l[i], r[i], max_height + 1);
        // lazy_segment_tree.print_tree();
    }
}

fn solve_2_rinc_rmax_abc327f() {
    // ABC327 Fのテスト
    // https://atcoder.jp/contests/abc327/tasks/abc327_f
    // [2]区間加算、区間最大値取得 のテスト
    
    use std::cmp::max;

    // 入力
    let ndw: Vec<usize> = read_vec_usize();
    let n = ndw[0];
    let d = ndw[1];
    let w = ndw[2];
    let mut t = vec![];
    let mut x = vec![];
    for i in 0..n {
        let tx: Vec<usize> = read_vec_usize();
        let ti = tx[0];
        let xi = tx[1];
        t.push(ti);
        x.push(xi);
    }

    // debug用に小さい値を用意
    // let max_x = 7;
    // let max_t = 8;

    let max_x = 200_000;
    let max_t = 200_000;

    // time_x[t] := 時刻tに落ちてくるりんご、消えるリンゴのx座標のリスト
    let mut time_x: Vec<Vec<(usize, isize)>> = vec![vec![]; max_t + 1];
    for i in 0..n {
        // リンゴiを回収できる、最後のカゴの設置開始時刻
        let end_time = t[i];

        // リンゴiを回収できる、最初のカゴの設置開始時刻
        let start_time = match end_time > d {
            true => end_time - d,
            false => 0
        };
        time_x[start_time].push((x[i], 1));
        time_x[end_time].push((x[i], - 1));
    }

    // 遅延評価セグメント木のコンストラクタの入力
    let init_value: isize = 0;
    let list_size: usize = max_x + 1;
    // Boxで型を明示すると、処理速度が遅くなる。
    // let e           : Box<dyn Fn() -> isize>                = Box::new(|| init_value);
    // let op          : Box<dyn Fn(&isize, &isize) -> isize>  = Box::new(|&s0: &isize, &s1: &isize| s0.max(s1));
    // let mapping     : Box<dyn Fn(&isize, &isize) -> isize>  = Box::new(|&f: &isize ,&x: &isize| f + x);
    // let composition : Box<dyn Fn(&isize, &isize) -> isize>  = Box::new(|&f: &isize, &g: &isize| f + g);
    // let id          : Box<dyn Fn() -> isize>                = Box::new(|| init_value);

    // 以下は型を明示しなくてもコンパイルが通る
    let e = || init_value; // 返り値の型は、型推論によって省略可能
    // let e = || -> isize {init_value};
    // let e:  fn() -> isize = || init_value; // 型注釈もできない
    // 関数は外部の変数をキャプチャできない
    // fn e() -> isize {
    //     return init_value
    // }

    let op = |&s0: &isize, &s1: &isize| s0.max(s1);
    // let op: fn(&isize, &isize) -> isize = |&s0: &isize, &s1: &isize| s0.max(s1); // クロージャーが外部変数をキャプチャしなければ、型注釈しても良い。
    // fn op(&s0: &isize, &s1:  &isize) -> isize {
    //     s0.max(s1)
    // }
    let mapping = |&f: &isize ,&x: &isize| f + x;
    let composition = |&f: &isize ,&g: &isize| f + g;
    let id = || init_value;

    // let mut lazy_segment_tree = lazy_segment_tree::LazySegmentTree::new(
    //     list_size, 
    //     e,
    //     op,
    //     mapping,
    //     composition,
    //     id
    // );
    // let mut lazy_segment_tree = lazy_segment_tree::LazySegmentTree::<isize, fn(&isize, &isize) -> isize, fn() -> isize, isize, fn(&isize, &isize) -> isize, fn(&isize, &isize) -> isize, fn() -> isize>::new_range_increment_update_and_range_maximum_query(list_size);
    
    // こっちでも正解
    // let mut lazy_segment_tree = lazy_segment_tree::new_range_increment_update_and_range_maximum_query(list_size);
    
    let mut init_vector = vec![0; max_x + 1];
    let mut lazy_segment_tree = lazy_segment_tree::new_range_increment_update_and_range_maximum_query_from_vec(&init_vector);


    // 変数にせずに直接代入すると、型を全く明示しなくてもコンパイルが通る
    // let mut lazy_segment_tree = lazy_segment_tree::LazySegmentTree::new(
    //     max_x + 1, 
    //     ||  init_value,
    //     |&s0, &s1| s0.max(s1),
    //     |&f ,&x| f + x,
    //     |&f, &g| f + g,
    //     || init_value
    // );

    // lazy_segment_tree.print_tree();
    // println!("time_x = {:?}", time_x);

    let mut ans = 0;
    for time in 0..max_t+1 {
        for i in 0..time_x[time].len() {
            // リンゴを回収できるカゴの左端の座標の最大値
            let right = time_x[time][i].0;

            // リンゴを回収できるカゴの左端の座標の最小値
            let left = match right + 1 < w {
                true => 0,
                false => right + 1 - w
            };

            let diff = time_x[time][i].1;
            lazy_segment_tree.range_update(left, right, diff);
        }
        if time_x[time].len() == 0 {continue}
        let value = lazy_segment_tree.range_query(0, max_x);
        // println!("time = {time}, value = {value}-----------");
        // lazy_segment_tree.print_tree();
        ans = max(ans, value);
    }
    println!("{}", ans);
}

fn solve_3_rinc_rsum_aoj() {
    // 問題
    // https://judge.u-aizu.ac.jp/onlinejudge/description.jsp?id=DSL_2_G&lang=ja
    // 入力
    let (n, q, queries) = input_for_aoj();

    // これでもいい
    // let mut lazy_segment_tree = lazy_segment_tree::new_range_increment_update_and_range_sum_query(n);

    let init_vector = vec![0; n];
    let mut lazy_segment_tree = lazy_segment_tree::new_range_increment_update_and_range_sum_query_from_vec(&init_vector);

    for i in 0..q {
        let k = queries[i][0];
        let s = queries[i][1] as usize - 1;
        let t = queries[i][2] as usize - 1;

        if k == 0 {
            let x = queries[i][3];
            lazy_segment_tree.range_update(s, t, x);
        }
        else {
            let range_sum_node = lazy_segment_tree.range_query(s, t);
            let range_sum = range_sum_node.value;
            println!("{}", range_sum);
        }
    }
}

fn solve_1_rinc_rmin_aoj() {
    // 入力
    let (n, q, queries) = input_for_aoj();

    // // 遅延評価セグメント木のインスタンス化
    // let mut lazy_segment_tree = lazy_segment_tree::new_range_increment_update_and_range_minimum_query(n);
    // // lazy_segment_tree.print_tree();

    // // 遅延評価セグメント木の data 配列の初期化 (treeのdata配列は、isize::MAXで初期化しているが、元の配列は本問題ではa[i]=0(i=0,1,...)なので、-isize::MAXで打ち消して0にしている点に注意)
    // lazy_segment_tree.range_update(0, n -1, -std::isize::MAX);
    // // lazy_segment_tree.print_tree();

    let init_vector = vec![0; n];
    let mut lazy_segment_tree = lazy_segment_tree::new_range_increment_update_and_range_minimum_query_from_vec(&init_vector);

    for i in 0..q {
        let k = queries[i][0];
        let s = queries[i][1] as usize;
        let t = queries[i][2] as usize;

        if k == 0 {
            let x = queries[i][3];
            lazy_segment_tree.range_update(s, t, x);
        }
        else {
            let range_min = lazy_segment_tree.range_query(s, t);
            println!("{}", range_min);
        }
    }
    // lazy_segment_tree.print_tree();

}

fn solve_4_rass_rmin_aoj() {
    // 入力
    let (n, q, queries) = input_for_aoj();

    let init_value = (1 << 31) - 1;
    let init_vector = vec![init_value; n];
    let mut lazy_segment_tree = lazy_segment_tree::new_range_assignment_update_and_range_minimum_query_from_vec(&init_vector);

    for i in 0..q {
        let k = queries[i][0];
        let s = queries[i][1] as usize;
        let t = queries[i][2] as usize;

        if k == 0 {
            let x = queries[i][3];
            lazy_segment_tree.range_update(s, t, x);
        }
        else {
            let range_min = lazy_segment_tree.range_query(s, t);
            println!("{}", range_min);
        }
    }
}

fn solve_6_rass_rsum_aoj() {
    // https://judge.u-aizu.ac.jp/onlinejudge/description.jsp?id=DSL_2_I&lang=ja
    // 入力
    let (n, q, queries) = input_for_aoj();

    let init_value = 0;
    let init_vector = vec![init_value; n];
    let mut lazy_segment_tree = lazy_segment_tree::new_range_assignment_update_and_range_sum_query_from_vec(&init_vector);

    for i in 0..q {
        let k = queries[i][0];
        let s = queries[i][1] as usize;
        let t = queries[i][2] as usize;

        if k == 0 {
            let x = queries[i][3];
            lazy_segment_tree.range_update(s, t, x);
        }
        else {
            let range_sum_node = lazy_segment_tree.range_query(s, t);
            let range_sum = range_sum_node.value;
            println!("{}", range_sum);
        }
    }
}

fn solve_3mod_rinc_rsum_abc179d() {
    let nk: Vec<usize> = read_vec_usize();
    let n = nk[0];
    let k = nk[1];
    
    let mut l = vec![];
    let mut r = vec![];

    for i in 0..k {
        let lr_i: Vec<usize> = read_vec_usize();
        let li = lr_i[0];
        let ri = lr_i[1];
        l.push(li);
        r.push(ri);
    }

    let modulus = 998_244_353;


    // 遅延評価セグメント木 + Modint
    let mi_generator = ModintGenerator::new(modulus);
    let zero = mi_generator.generate(0);
    let one = mi_generator.generate(1);
    let mut init_vector = vec![zero; n];
    init_vector[0] = one;
    let mut lazy_segment_tree = lazy_segment_tree::mod_new_range_increment_update_and_range_sum_query_from_vec(&init_vector);

    for i in 0..n-1 {
        for j in 0..k {
            let l_ij = i + l[j];
            let r_ij = std::cmp::min(i + r[j], n-1);
            if l_ij > n - 1 {
                continue
            }
            let num_state_i = lazy_segment_tree.range_query(i, i).value;
            lazy_segment_tree.range_update(l_ij, r_ij, num_state_i);
        }
    }
    let ans = lazy_segment_tree.range_query(n-1, n-1).value.value;
    println!("{}", ans % modulus);
}

// Aizu Online Judge の遅延評価セグメント木の問題の入力受け取り用の関数
// AOJは、proconio が使えない
fn input_for_aoj() -> (usize, usize, Vec<Vec<isize>>) {
    let nq: Vec<usize> = read_vec_usize();
    let n: usize = nq[0];
    let q: usize = nq[1];
    let mut queries = vec![];
    for i in 0..q {
        let query: Vec<isize> = read_vec_isize();
        queries.push(query);
    }
    return (n, q, queries)
}

// インタラクティブな読み込みをする関数 (1行に複数のusize)
fn read_vec_usize() -> Vec<usize> {
    let mut line_string = String::new();
    std::io::stdin().read_line(&mut line_string).expect("入力エラー");
    let line_str_list: Vec<&str> = line_string.split_whitespace().collect();
    let line_usize_list: Vec<usize> = line_str_list.into_iter().map(|i| (i.parse().expect("変換エラー"))).collect();
    return line_usize_list
}

// インタラクティブな読み込みをする関数 (1行に複数のisize)
fn read_vec_isize() -> Vec<isize> {
    let mut line_string = String::new();
    std::io::stdin().read_line(&mut line_string).expect("入力エラー");
    let line_str_list: Vec<&str> = line_string.split_whitespace().collect();
    let line_isize_list: Vec<isize> = line_str_list.into_iter().map(|i| (i.parse().expect("変換エラー"))).collect();
    return line_isize_list
}


// 抽象化した遅延評価セグメント木を実装する
// 参考実装 (ACLのC++のコードと、kenkooooさんのRust化されたコード)
// https://github.com/atcoder/ac-library/blob/master/atcoder/lazysegtree.hpp
// https://github.com/kenkoooo/competitive-programming-rs/blob/master/src/data_structure/lazy_segment_tree.rs
pub mod lazy_segment_tree {
    #[derive(Debug, Clone)]
    pub struct LazySegmentTree<S, Op, E, F, Mapping, Composition, Id> {
        list_size: usize,           // 探索対象の配列の大きさ
        tree_size: usize,           // セグメント木の頂点の総数
        leaf_size: usize,           // セグメント木の葉の総数
        pub tree: Vec<S>,               // セグメント木
        lazy_tree: Vec<F>,          // 遅延評価用の木
        op: Op,                     // 区間取得演算
        e: E,                       // 区間取得演算の単位元
        mapping: Mapping,           // 区間更新演算 (lazy -> data への伝播に対応)
        composition: Composition,   // 合成関数f(g(x)) (親のlazy -> 子のlazy　への伝播に対応)
        id: Id                      // 区間操作演算mappingにおける恒等写像 (遅延配列の初期値に対応)
    }
        // Update (更新)
        // - Range Increment Update
        // - Range Assignment Update
        //   (Add と Replace の方がわかりやすいかも)

        // Query (取得)
        // - Range Minimum Query
        // - Range Maximum Query
        // - Range Sum Query

    // [1] 区間加算更新・区間最小値取得　の生成
    pub fn new_range_increment_update_and_range_minimum_query(list_size: usize) -> LazySegmentTree<isize, fn(&isize, &isize) -> isize, fn() -> isize, isize, fn(&isize, &isize) -> isize, fn(&isize, &isize) -> isize, fn() -> isize> {
        /// 区間取得演算 (opだけど、更新操作ではなく取得なので注意)
        /// s0: 左の子data (たぶん)
        /// s1: 右の子data (たぶん)
        /// 返り値: 2子から得られたdata
        fn op(&s0: &isize, &s1:  &isize) -> isize {
            s0.min(s1)
        }
        /// Lazy -> Data の更新操作
        /// f: Lazyによる操作
        /// x: 更新前のdata
        /// 返り値: 更新後のdata
        fn mapping(&f: &isize ,&x: &isize) -> isize {
            return f + x
        }
        /// Lazy -> Lazy の更新操作
        /// f: 追加する最新の更新操作
        /// g: これまでの更新操作
        /// 返り値: g -> f の順にする操作。gとfの合成関数: f(g(x))
        fn composition(&f: &isize ,&g: &isize) -> isize {
            return f + g
        }
        /// 区間取得演算opの単位元 (dataの初期化に使われる)
        /// op(a, e()) = op(e(), a) = a となるような、e()
        fn e() -> isize {
            let init_value: isize = std::isize::MAX;
            return init_value
        }
        /// 区間操作(更新)演算mappingにおける恒等写像 (lazyの初期化と、mappingとcompositionに使われる。区間代入更新のときは、正直なんでもいいと思う。)
        /// mapping(id(), a) = a となるようなid()
        fn id() -> isize {
            let init_value: isize = 0;
            return init_value
        }
        let lazy_segment_tree = LazySegmentTree::new(
            list_size,
            e as fn() -> isize, 
            op as fn(&isize, &isize) -> isize, 
            mapping as fn(&isize, &isize) -> isize, 
            composition as fn(&isize, &isize) -> isize, 
            id as fn() -> isize
        );
        return lazy_segment_tree
    }

    // [1] 区間加算更新・区間最小値取得 を vector から生成
    pub fn new_range_increment_update_and_range_minimum_query_from_vec(init_vector: &Vec<isize>) -> LazySegmentTree<isize, fn(&isize, &isize) -> isize, fn() -> isize, isize, fn(&isize, &isize) -> isize, fn(&isize, &isize) -> isize, fn() -> isize> {
        let n = init_vector.len();
        let mut lazy_segment_tree = new_range_increment_update_and_range_minimum_query(n);
        // 遅延評価セグメント木の data 配列の初期化 (treeのdata配列は、isize::MAXで初期化しているが、元の配列は本問題ではa[i]=0(i=0,1,...)なので、-isize::MAXで打ち消して0にしている点に注意)
        lazy_segment_tree.range_update(0, n -1, -std::isize::MAX);
        for i in 0..n {
            lazy_segment_tree.range_update(i, i, init_vector[i]);
        }
        return lazy_segment_tree
    }
    
    // [2] 区間加算更新・区間最大値取得　の生成
    pub fn new_range_increment_update_and_range_maximum_query(list_size: usize) -> LazySegmentTree<isize, fn(&isize, &isize) -> isize, fn() -> isize, isize, fn(&isize, &isize) -> isize, fn(&isize, &isize) -> isize, fn() -> isize> {
        fn op(&s0: &isize, &s1:  &isize) -> isize {
            s0.max(s1)
        }
        fn mapping(&f: &isize ,&x: &isize) -> isize {
            f + x
        }
        fn composition(&f: &isize ,&g: &isize) -> isize {
            f + g
        }
        fn e() -> isize {
            // let init_value: isize = 0; // <- 本当はminにするべき
            let init_value: isize = - std::isize::MAX;
            return init_value
        }
        fn id() -> isize {
            let init_value: isize = 0;
            return init_value
        }
        
        // 関数のかわりにクロージャーを使うこともできる。
        // let init_value: isize = 0;
        // let e = || init_value; // 返り値の型は、型推論によって省略可能
        // let op = |&s0: &isize, &s1: &isize| s0.max(s1);
        // let mapping = |&f: &isize ,&x: &isize| f + x;
        // let composition = |&f: &isize ,&g: &isize| f + g;
        // let id = || init_value;

        // ここで関数をpointerにcastしないと、"expected fn pointer, found fn item" というエラーメッセージが起きる。
        // fn item: 関数の実態
        // fn pointer: 関数の参照
        // Rustでは、関数の型アノテーションをする際に、fn item が使えず、 fn pointer として明示する必要がある。 cf. https://doc.rust-lang.org/beta/reference/types/function-item.html
        // (この関数の返り値の型の Op や E も、当然 fn pointer が使われている。)
        // また、キャストは不可逆なので注意 cf. https://stackoverflow.com/questions/71974428/dealing-with-expected-fn-pointer-found-fn-item
        // キャスト可能: fn item (fn ... {name})-> fn pointer 
        // キャスト不可：fn pointer -> fn item (fn ... {name})
        // キャストの例：
        // 可能な例：signer_func as fn(&[u8; 20], &[u8; 32]) -> &'a [u8; 32]
        // 不可な例：&signer_func as &fn(&[u8; 20], &[u8; 32]) -> &'a [u8; 32]
        let lazy_segment_tree = LazySegmentTree::new(
            list_size,
            e as fn() -> isize, 
            op as fn(&isize, &isize) -> isize, 
            mapping as fn(&isize, &isize) -> isize, 
            composition as fn(&isize, &isize) -> isize, 
            id as fn() -> isize
        );
        return lazy_segment_tree
    }

    // [2] 区間加算更新・区間最大値取得 を vector から生成
    pub fn new_range_increment_update_and_range_maximum_query_from_vec(init_vector: &Vec<isize>) -> LazySegmentTree<isize, fn(&isize, &isize) -> isize, fn() -> isize, isize, fn(&isize, &isize) -> isize, fn(&isize, &isize) -> isize, fn() -> isize> {
        let n = init_vector.len();
        let mut lazy_segment_tree = new_range_increment_update_and_range_maximum_query(n);
        // 遅延評価セグメント木の data 配列の初期化 (treeのdata配列は、-isize::MAXで初期化しているが、+isize::MAXで打ち消して0にしている点に注意)
        lazy_segment_tree.range_update(0, n -1, std::isize::MAX); // 本当は素直に-MINしたいけど、オーバーフローするのでMAXを足す。
        // isize::MIN := -9_223_372_036_854_775_808isize
        // isize::MAX :=  9_223_372_036_854_775_807isize
        for i in 0..n {
            lazy_segment_tree.range_update(i, i, init_vector[i]);
        }
        return lazy_segment_tree
    }

    // [3] 区間加算更新・区間和取得　の生成
    pub fn new_range_increment_update_and_range_sum_query(list_size: usize) -> LazySegmentTree<SSum, fn(&SSum, &SSum) -> SSum, fn() -> SSum, isize, fn(&isize, &SSum) -> SSum, fn(&isize, &isize) -> isize, fn() -> isize> {
        
        fn op(&s0: &SSum, &s1:  &SSum) -> SSum {
            return SSum {value: s0.value + s1.value, size: s0.size + s1.size}
        }
        fn mapping(&f: &isize ,&x: &SSum) -> SSum {
            return SSum {value: x.value + x.size * f, size: x.size}
        }
        fn composition(&f: &isize ,&g: &isize) -> isize {
            f + g
        }
        fn e() -> SSum {
            let init_value: isize = 0;
            let init_size: isize = 0;
            return SSum {value: init_value, size: init_size}
        }
        fn id() -> isize {
            let init_value: isize = 0;
            return init_value
        }
        
        let mut lazy_segment_tree = LazySegmentTree::new(
            list_size,
            e as fn() -> SSum, 
            op as fn(&SSum, &SSum) -> SSum, 
            mapping as fn(&isize, &SSum) -> SSum, 
            composition as fn(&isize, &isize) -> isize, 
            id as fn() -> isize
        );

        // 各ノードの、守備範囲の大きさ(tree.size)を適切に初期化
        init_node_size_for_range_sum_tree(&mut lazy_segment_tree);

        return lazy_segment_tree
    }
    
    // [3-mod] 区間加算更新・区間和取得　の生成
    pub fn mod_new_range_increment_update_and_range_sum_query(list_size: usize) -> LazySegmentTree<SSumMod, fn(&SSumMod, &SSumMod) -> SSumMod, fn() -> SSumMod, Modint, fn(&Modint, &SSumMod) -> SSumMod, fn(&Modint, &Modint) -> Modint, fn() -> Modint> {    
        fn op(&s0: &SSumMod, &s1:  &SSumMod) -> SSumMod {
            return SSumMod {value: s0.value + s1.value, size: s0.size + s1.size}
        }
        fn mapping(&f: &Modint ,&x: &SSumMod) -> SSumMod {
            return SSumMod {value: x.value + x.size * f, size: x.size}
        }
        fn composition(&f: &Modint ,&g: &Modint) -> Modint {
            f + g
        }
        fn e() -> SSumMod {
            let init_value: Modint = Modint{modulus: 998244353, value: 0};
            let init_size: isize = 0;
            return SSumMod {value: init_value, size: init_size}
        }
        fn id() -> Modint {
            let init_value: Modint = Modint{modulus: 998244353, value: 0};
            return init_value
        }
        
        let mut lazy_segment_tree = LazySegmentTree::new(
            list_size,
            e as fn() -> SSumMod, 
            op as fn(&SSumMod, &SSumMod) -> SSumMod, 
            mapping as fn(&Modint, &SSumMod) -> SSumMod, 
            composition as fn(&Modint, &Modint) -> Modint, 
            id as fn() -> Modint
        );

        // 各ノードの、守備範囲の大きさ(tree.size)を適切に初期化
        mod_init_node_size_for_range_sum_tree(&mut lazy_segment_tree);

        return lazy_segment_tree
    }

    // [3] 区間加算更新・区間和取得 を vector から生成
    pub fn new_range_increment_update_and_range_sum_query_from_vec(init_vector: &Vec<isize>) -> LazySegmentTree<SSum, fn(&SSum, &SSum) -> SSum, fn() -> SSum, isize, fn(&isize, &SSum) -> SSum, fn(&isize, &isize) -> isize, fn() -> isize> {
        let n = init_vector.len();
        let mut lazy_segment_tree = new_range_increment_update_and_range_sum_query(n);
        for i in 0..n {
            lazy_segment_tree.range_update(i, i, init_vector[i]);
        }
        return lazy_segment_tree
    }
    
    // [3-mod] 区間加算更新・区間和取得 を vector から生成
    use crate::Modint;
    pub fn mod_new_range_increment_update_and_range_sum_query_from_vec(init_vector: &Vec<Modint>) -> LazySegmentTree<SSumMod, fn(&SSumMod, &SSumMod) -> SSumMod, fn() -> SSumMod, Modint, fn(&Modint, &SSumMod) -> SSumMod, fn(&Modint, &Modint) -> Modint, fn() -> Modint> {
        let n = init_vector.len();
        let mut lazy_segment_tree = mod_new_range_increment_update_and_range_sum_query(n);
        for i in 0..n {
            lazy_segment_tree.range_update(i, i, init_vector[i]);
        }
        return lazy_segment_tree
    }

    // [4] 区間代入更新・区間最小値取得　の生成
    pub fn new_range_assignment_update_and_range_minimum_query(list_size: usize) -> LazySegmentTree<isize, fn(&isize, &isize) -> isize, fn() -> isize, isize, fn(&isize, &isize) -> isize, fn(&isize, &isize) -> isize, fn() -> isize> {
        /// 区間取得演算 (opだけど、更新操作ではなく取得なので注意)
        /// s0: 左の子data (たぶん)
        /// s1: 右の子data (たぶん)
        /// 返り値: 2子から得られたdata
        fn op(&s0: &isize, &s1:  &isize) -> isize {
            s0.min(s1)
        }
        /// Lazy -> Data の更新操作
        /// f: Lazyによる操作
        /// x: 更新前のdata
        /// 返り値: 更新後のdata
        fn mapping(&f: &isize ,&x: &isize) -> isize {
            if f == id() {
                return x
            }
            else {
                return f
            }
        }
        /// Lazy -> Lazy の更新操作
        /// f: 追加する最新の更新操作
        /// g: これまでの更新操作
        /// 返り値: g -> f の順にする操作。gとfの合成関数: f(g(x))
        fn composition(&f: &isize ,&g: &isize) -> isize {
            if f == id() {
                return g
            }
            else {
                return f
            }
        }
        /// 区間取得演算opの単位元 (dataの初期化に使われる)
        fn e() -> isize {
            let init_value: isize = std::isize::MAX;
            return init_value
        }
        /// 区間操作(更新)演算mappingにおける恒等写像 (lazyの初期化と、mappingとcompositionに使われる。区間代入更新のときは、正直なんでもいいと思う。)
        fn id() -> isize {
            let init_value: isize = std::isize::MAX; // 代入する値の範囲外に使われなければ、どんな値を入れてもいい。
            return init_value
        }
        let lazy_segment_tree = LazySegmentTree::new(
            list_size,
            e as fn() -> isize, 
            op as fn(&isize, &isize) -> isize, 
            mapping as fn(&isize, &isize) -> isize, 
            composition as fn(&isize, &isize) -> isize, 
            id as fn() -> isize
        );
        return lazy_segment_tree
    }

    // [4] 区間代入更新・区間最小値取得 を vector から生成
    pub fn new_range_assignment_update_and_range_minimum_query_from_vec(init_vector: &Vec<isize>) -> LazySegmentTree<isize, fn(&isize, &isize) -> isize, fn() -> isize, isize, fn(&isize, &isize) -> isize, fn(&isize, &isize) -> isize, fn() -> isize> {
        let n = init_vector.len();
        let mut lazy_segment_tree = new_range_assignment_update_and_range_minimum_query(n);
        for i in 0..n {
            lazy_segment_tree.range_update(i, i, init_vector[i]);
        }
        return lazy_segment_tree
    }

    // [5] 区間代入更新・区間最大値取得　の生成
    pub fn new_range_assignment_update_and_range_maximum_query(list_size: usize) -> LazySegmentTree<isize, fn(&isize, &isize) -> isize, fn() -> isize, isize, fn(&isize, &isize) -> isize, fn(&isize, &isize) -> isize, fn() -> isize> {
        /// 区間取得演算 (opだけど、更新操作ではなく取得なので注意)
        /// s0: 左の子data (たぶん)
        /// s1: 右の子data (たぶん)
        /// 返り値: 2子から得られたdata
        fn op(&s0: &isize, &s1:  &isize) -> isize {
            s0.max(s1)
        }
        /// Lazy -> Data の更新操作
        /// f: Lazyによる操作
        /// x: 更新前のdata
        /// 返り値: 更新後のdata
        fn mapping(&f: &isize ,&x: &isize) -> isize {
            if f == id() {
                return x
            }
            else {
                return f
            }
        }
        /// Lazy -> Lazy の更新操作
        /// f: 追加する最新の更新操作
        /// g: これまでの更新操作
        /// 返り値: g -> f の順にする操作。gとfの合成関数: f(g(x))
        fn composition(&f: &isize ,&g: &isize) -> isize {
            if f == id() {
                return g
            }
            else {
                return f
            }
        }
        /// 区間取得演算opの単位元 (dataの初期化に使われる)
        fn e() -> isize {
            let init_value: isize = std::isize::MIN;
            return init_value
        }
        /// 区間操作(更新)演算mappingにおける恒等写像 (lazyの初期化と、mappingとcompositionに使われる。区間代入更新のときは、正直なんでもいいと思う。)
        fn id() -> isize {
            let init_value: isize = std::isize::MIN; // 代入する値の範囲外に使われなければ、どんな値を入れてもいい。
            return init_value
        }
        let lazy_segment_tree = LazySegmentTree::new(
            list_size,
            e as fn() -> isize, 
            op as fn(&isize, &isize) -> isize, 
            mapping as fn(&isize, &isize) -> isize, 
            composition as fn(&isize, &isize) -> isize, 
            id as fn() -> isize
        );
        return lazy_segment_tree
    }

    // [5] 区間代入更新・区間最大値取得 を vector から生成
    pub fn new_range_assignment_update_and_range_maximum_query_from_vec(init_vector: &Vec<isize>) -> LazySegmentTree<isize, fn(&isize, &isize) -> isize, fn() -> isize, isize, fn(&isize, &isize) -> isize, fn(&isize, &isize) -> isize, fn() -> isize> {
        let n = init_vector.len();
        let mut lazy_segment_tree = new_range_assignment_update_and_range_maximum_query(n);
        for i in 0..n {
            lazy_segment_tree.range_update(i, i, init_vector[i]);
        }
        return lazy_segment_tree
    }

    // [6] 区間代入更新・区間和取得　の生成
    pub fn new_range_assignment_update_and_range_sum_query(list_size: usize) -> LazySegmentTree<SSum, fn(&SSum, &SSum) -> SSum, fn() -> SSum, isize, fn(&isize, &SSum) -> SSum, fn(&isize, &isize) -> isize, fn() -> isize> {
        /// 区間取得演算 (opだけど、更新操作ではなく取得なので注意)
        /// s0: 左の子data (たぶん)
        /// s1: 右の子data (たぶん)
        /// 返り値: 2子から得られたdata
        fn op(&s0: &SSum, &s1:  &SSum) -> SSum {
            return SSum {value: s0.value + s1.value, size: s0.size + s1.size}
        }
        /// Lazy -> Data の更新操作
        /// f: Lazyによる操作
        /// x: 更新前のdata
        /// 返り値: 更新後のdata
        fn mapping(&f: &isize ,&x: &SSum) -> SSum {
            if f == id() {
                return x
            }
            else {
                // x.value = f * x.size;
                // return x
                return SSum {value: f * x.size, size: x.size}
            }
        }
        /// Lazy -> Lazy の更新操作
        /// f: 追加する最新の更新操作
        /// g: これまでの更新操作
        /// 返り値: g -> f の順にする操作。gとfの合成関数: f(g(x))
        fn composition(&f: &isize ,&g: &isize) -> isize {
            if f == id() {
                return g
            }
            else {
                return f
            }
        }
        /// 区間取得演算opの単位元 (dataの初期化に使われる)
        fn e() -> SSum {
            let init_value: isize = 0;
            let init_size: isize = 0;
            return SSum {value: init_value, size: init_size}
        }
        /// 区間操作(更新)演算mappingにおける恒等写像 (lazyの初期化と、mappingとcompositionに使われる。区間代入更新のときは、範囲外であれば正直なんでもいいと思う。)
        fn id() -> isize {
            let init_value: isize = std::isize::MAX; // 代入更新する値の範囲外であれば、どんな値を入れてもいい。
            return init_value
        }
        let mut lazy_segment_tree = LazySegmentTree::new(
            list_size,
            e as fn() -> SSum, 
            op as fn(&SSum, &SSum) -> SSum, 
            mapping as fn(&isize, &SSum) -> SSum, 
            composition as fn(&isize, &isize) -> isize, 
            id as fn() -> isize
        );

        // 各ノードの、守備範囲の大きさ(tree.size)を適切に初期化
        init_node_size_for_range_sum_tree(&mut lazy_segment_tree);

        return lazy_segment_tree
    }

    // [6] 区間代入更新・区間和取得 を vector から生成
    pub fn new_range_assignment_update_and_range_sum_query_from_vec(init_vector: &Vec<isize>) -> LazySegmentTree<SSum, fn(&SSum, &SSum) -> SSum, fn() -> SSum, isize, fn(&isize, &SSum) -> SSum, fn(&isize, &isize) -> isize, fn() -> isize> {
        let n = init_vector.len();
        let mut lazy_segment_tree = new_range_assignment_update_and_range_sum_query(n);
        for i in 0..n {
            lazy_segment_tree.range_update(i, i, init_vector[i]);
        }
        return lazy_segment_tree
    }

    // 区間和取得用の遅延評価セグメント木の各ノードの、守備範囲の大きさ(tree.size)を適切に初期化する関数
    fn init_node_size_for_range_sum_tree(lazy_segment_tree: &mut LazySegmentTree<SSum, fn(&SSum, &SSum) -> SSum, fn() -> SSum, isize, fn(&isize, &SSum) -> SSum, fn(&isize, &isize) -> isize, fn() -> isize>) {
        // 葉のノードについてはtree.sizeを、元の配列部分だけ1にして、範囲外は0のままにする
        let first_list_index = lazy_segment_tree.tree_size / 2; // 木における配列先頭のindex
        for i in 0..lazy_segment_tree.list_size {
            lazy_segment_tree.tree[i + first_list_index].size = 1;
        }

        // 木の最下段から、根に向かって、親ノードの守備範囲の大きさを計算
        let mut current_stage_size = lazy_segment_tree.leaf_size;
        let mut current_stage_first_index = first_list_index;
        while current_stage_size != 1 {
            for i in 0..current_stage_size {
                let v = current_stage_first_index + i;
                // p := parent index
                let p = (v - 1) / 2;
                lazy_segment_tree.tree[p].size += lazy_segment_tree.tree[v].size;
            }
            current_stage_size /= 2;
            current_stage_first_index /= 2;
        }
    }

    // mod 区間和取得用の遅延評価セグメント木の各ノードの、守備範囲の大きさ(tree.size)を適切に初期化する関数
    fn mod_init_node_size_for_range_sum_tree(lazy_segment_tree: &mut LazySegmentTree<SSumMod, fn(&SSumMod, &SSumMod) -> SSumMod, fn() -> SSumMod, Modint, fn(&Modint, &SSumMod) -> SSumMod, fn(&Modint, &Modint) -> Modint, fn() -> Modint>) {
        // 葉のノードについてはtree.sizeを、元の配列部分だけ1にして、範囲外は0のままにする
        let first_list_index = lazy_segment_tree.tree_size / 2; // 木における配列先頭のindex
        for i in 0..lazy_segment_tree.list_size {
            lazy_segment_tree.tree[i + first_list_index].size = 1;
        }

        // 木の最下段から、根に向かって、親ノードの守備範囲の大きさを計算
        let mut current_stage_size = lazy_segment_tree.leaf_size;
        let mut current_stage_first_index = first_list_index;
        while current_stage_size != 1 {
            for i in 0..current_stage_size {
                let v = current_stage_first_index + i;
                // p := parent index
                let p = (v - 1) / 2;
                lazy_segment_tree.tree[p].size += lazy_segment_tree.tree[v].size;
            }
            current_stage_size /= 2;
            current_stage_first_index /= 2;
        }
    }

    /// 区間和取得用のdataノードの構造体
    #[derive(Clone, Debug, Copy)]
    pub struct SSum {
        pub value: isize,
        size: isize
    }
    #[derive(Clone, Debug, Copy)]
    pub struct SSumMod {
        pub value: Modint,
        size: isize
    }
    
    // メソッドの実装
    impl<S, Op, E, F, Mapping, Composition, Id> LazySegmentTree<S, Op, E, F, Mapping, Composition, Id>
    where
        S: Clone + std::fmt::Debug,                         // セグメント木のノードに格納されたデータ型
        Op: Fn(&S, &S) -> S,                                // 区間取得をどのような演算で行うか
        E: Fn() -> S,                                       // 区間取得演算の単位元
        F: Clone + std::cmp::PartialEq + std::fmt::Debug,   // 遅延評価用の木のノードに格納されたデータ型
        Mapping: Fn(&F, &S) -> S,                           // lazy -> data への操作(更新)
        Composition: Fn(&F, &F) -> F,                       // 親のlazy -> 子のlazy　への操作(更新)
        Id: Fn() -> F                                       // 区間操作(更新)演算 mapping における恒等写像
    {
        pub fn new(
            list_size: usize,                               // 入力配列の大きさ
            e: E,
            op: Op,
            mapping: Mapping,
            composition: Composition,
            id: Id
        ) -> Self {
            
            // セグメント木の頂点の総数tree_sizeを求める。
            // まずはセグメント木の葉の数leaf_sizeを、
            // (leaf_size / 2 < list_size <= leaf_size)
            // を満たす2のべき乗数となるように計算
            let mut leaf_size = 1;
            while (leaf_size < list_size) {
                leaf_size *= 2;
            }
            
            // セグメント木の頂点数 = セグメント木の葉の数 * 2 - 1
            let tree_size: usize = leaf_size * 2 - 1;
            
            // 木の各ノードの初期値
            let tree: Vec<S> = vec![e(); tree_size];
            let lazy_tree: Vec<F> = vec![id(); tree_size];
    
            return LazySegmentTree {list_size, tree_size, leaf_size, tree, lazy_tree, op, e, mapping, composition, id}
        }

        // 遅延木のv番目のノード(要素)について、遅延評価を行う(セグメント木には無かったメソッド)
        fn eval(&mut self, v: usize, v_l: usize, v_r: usize) {
            // v: 遅延評価したいノードのインデックス
            // v_l: ノードvの守備範囲の左端 (閉区間)
            // v_r: ノードvの守備範囲の右端 (閉区間)
            // 自ノードの値配列に値を伝播させる
            // 子ノードの遅延配列に値を伝播させる
            // 自分のノードの遅延配列を空にする
    
            // 遅延評価で空でない場合、自ノード及び子ノードへの値の伝播が起こる
            // let NEGATIVE_INF = (1 << 60) * (-1);
            // let init_value = 0;
            // if (self.lazy_tree[v] != NEGATIVE_INF) {
            
            if (self.lazy_tree[v] != (self.id)()) {
                // self.tree[v] = self.lazy_tree[v];
                // self.tree[v] += self.lazy_tree[v];
                // mapping: Lazy node -> Data node の伝播
                self.tree[v] = (self.mapping)(&(self.lazy_tree[v].clone()), &self.tree[v]);
                
                // 最下段ではない場合、子ノードへ伝播
                if v_r - v_l >= 1 {
                    self.lazy_tree[2*v+1] = (self.composition)(&(self.lazy_tree[v].clone()), &(self.lazy_tree[2*v+1]));
                    self.lazy_tree[2*v+2] = (self.composition)(&(self.lazy_tree[v].clone()), &(self.lazy_tree[2*v+2]));
                    // self.lazy_tree[2*v+1] += self.lazy_tree[v];
                    // self.lazy_tree[2*v+2] += self.lazy_tree[v];
                    // self.lazy_tree[2*v+1] += self.lazy_tree[v] / 2;
                    // self.lazy_tree[2*v+2] += self.lazy_tree[v] / 2;
                }
                // 伝播が終わったので、自ノードの遅延配列を空にする
                self.lazy_tree[v] = (self.id)();
            }
        }
    
        pub fn range_update(&mut self, update_l: usize, update_r: usize, f: F) {
            // update_l:    更新区間の左端
            // update_r:    更新区間の右端(閉区間)
            // f:      更新後の値
            // println!("start to update between {}-{} to {:?}", update_l, update_r, f);
            self._range_update(update_l, update_r, f, 0, 0, self.leaf_size - 1);
        }
    
        // 探索対象の配列の区間[l,r]の要素を、値xに変更する (1つの要素ではなく、複数要素を含む区間の更新)
        // 根から下に下がっていく。(セグメント木の1つの要素の更新のときは下から根に向かって更新していた)
        fn _range_update(&mut self, update_l: usize, update_r: usize, f: F, v: usize, v_l: usize, v_r: usize) {
            // update_l:    探索区間の左端
            // update_r:    探索区間の右端(閉区間)
            // x:      更新後の値
            // v:      現在の頂点のインデックス
            // v_l:    現在の頂点の守備範囲の左端
            // v_r:    現在の頂点の守備範囲の右端(閉区間)
            // 外からは、self._range_update(update_l, update_r, x, 0, 0, self.leaf_size - 1)として呼ぶ。特にv_rは、self.list_sizeではないので注意
            
            // v番目の頂点の遅延評価
            self.eval(v, v_l, v_r);
    
            // (1)更新範囲が、その頂点が持つ守備範囲と、交差しない
            if v_r < update_l || update_r < v_l {
                // 何もしない
                return;
            }
            // (2)更新範囲が、その頂点が持つ守備範囲を、完全に含む:Query ⊃ Vertex
            else if update_l <= v_l && v_r <= update_r {
                // 遅延木に値を入れた後に評価
                // self.lazy_tree[v] = x;
                // self.lazy_tree[v] += x;
                self.lazy_tree[v] = (self.composition)(&f, &self.lazy_tree[v]);
                // self.lazy_tree[v] += (update_r + 1 - update_l) as isize * x;
                // ここで評価を入れないと、(3)で子の配列が更新前の状態で、max()を呼び出すことになる
                self.eval(v, v_l, v_r);
                return;
            }
            // (3)更新範囲が、その頂点が持つ守備範囲と、部分一致:Query ∩ Vertex ≠ ∅
            else {
                // 2つの子頂点の内、大きい方に更新
                self._range_update(update_l, update_r, f.clone(), 2 * v + 1, v_l, (v_l + v_r) / 2);
                self._range_update(update_l, update_r, f.clone(), 2 * v + 2, (v_l + v_r) / 2 + 1, v_r);
                // self.tree[v] = max(self.tree[2 * v + 1], self.tree[2 * v + 2]);
                self.tree[v] = (self.op)(&self.tree[2 * v + 1], &self.tree[2 * v + 2]);
                return;
            }
        }
    
        // クラスの外からクエリを行うときのメソッド
        pub fn range_query(&mut self, q_l: usize, q_r: usize) -> S {
            // println!("start to query between {}-{} !!", q_l, q_r);
            return self._range_query(q_l, q_r, 0, 0, self.leaf_size - 1);
        }
    
        // 閉区間[q_l, q_r]の最大値を求める。右端が開区間')'ではなく、閉区間']'にしているので注意
        fn _range_query(&mut self, q_l: usize, q_r: usize, v: usize, v_l: usize, v_r: usize) -> S {
            // q_l:    探索区間の左端
            // q_r:    探索区間の右端(閉区間)
            // v:      現在の頂点のインデックス
            // v_l:    現在の頂点の守備範囲の左端
            // v_r:    現在の頂点の守備範囲の右端(閉区間)
            // 外からは、self._range_query(q_l, q_r, 0, 0, self.leaf_size - 1)として呼ぶ。特にv_rは、self.list_sizeではないので注意
    
            // 遅延評価!
            self.eval(v, v_l, v_r);
    
            // (1)探索範囲が、その頂点が持つ守備範囲と、交差しない
            if v_r < q_l || q_r < v_l {
                // let NEGATIVE_INF = (1 << 60) * (-1);
                // return NEGATIVE_INF
                // let init_value = 0;
                // return init_value
                return (self.e)()
            }
            // (2)探索範囲が、その頂点が持つ守備範囲を、完全に含む
            else if q_l <= v_l && v_r <= q_r {
                return self.tree[v].clone()
            }
            // (3)探索範囲が、その頂点が持つ守備範囲と、部分一致
            else {
                // 2つの子頂点の内、小さい方を返す
                let child_0 = self._range_query(q_l, q_r, 2 * v + 1, v_l, (v_l + v_r) / 2);
                let child_1 = self._range_query(q_l, q_r, 2 * v + 2, (v_l + v_r) / 2 + 1, v_r);
                return (self.op)(&child_0, &child_1);
            }
        }
    
        // 配列を確認(デバッグ用)
        pub fn print_list(&self) {
            println!("Print Array: ");
            for i in 0..self.list_size {
                let tree_index = i + self.tree_size / 2;
                print!("{:?}, ", self.tree[tree_index]);
            }
            println!("");
        }
    
        // セグメント木を確認(デバッグ用)
        fn _print_tree(&self, kind: &str) {
            let mut next_depth_index = 1;
            for i in 0..self.tree_size {
                if i == next_depth_index {
                    println!("");
                    next_depth_index = next_depth_index * 2 + 1;
                }
                match kind {
                    "segment_tree" => print!("{:?} ", self.tree[i]),
                    "lazy_tree" => print!("{:?} ", self.lazy_tree[i]),
                    _ => ()
                }
            }
            println!("");
        }
    
        pub fn print_tree(&self) {
            println!("==== Print Segment Tree ====");
            self._print_tree("segment_tree");
            println!("==== Print Lazy Tree ====");
            self._print_tree("lazy_tree");
            println!("==== ==== ==== ==== ==== =====");
        }
    }    
}


// ModIntの実装参考
// https://qiita.com/namn1125/items/5100cb85021a1d6e8f6c
// https://github.com/rust-lang-ja/ac-library-rs/blob/master/src/modint.rs
// https://github.com/kenkoooo/competitive-programming-rs/blob/master/src/math/mod_int.rs
// AtCoderの公式(C++): https://github.com/atcoder/ac-library/blob/master/document_ja/modint.md
struct ModintGenerator {
    modulus: isize
}

impl ModintGenerator {
    fn new(modulus: isize) -> ModintGenerator {
        ModintGenerator {modulus: modulus}
    }
    fn generate(&self, value: isize) -> Modint {
        let modint = Modint::new(self.modulus, value);
        return modint
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct Modint {
    modulus: isize,
    value: isize,
}

impl Modint {
    fn new(modulus: isize, value: isize) -> Modint {
        Modint{modulus: modulus, value: value % modulus}
    }
    // mod p を法とした時の逆数(逆元という) 1 / a の値
    fn inverse(&self) -> Self {
        // フェルマーの小定理
        //     a^(p-1) = 1     (mod p)
        // <=> a * a^(p-2) = 1 (mod p)
        // <=> 1 / a = a^(p-2) (mod p)
        // (ただし、法pは素数)
        self.pow(self.modulus - 2)
    }

    // mod p を法とした時の累乗
    // base^(x) % mod を繰り返し二乗法により、O(log2(x))の計算量で求める　(O(x)だとTLE)
    // No.69参照
    fn pow(&self, mut exponent: isize) -> Self {
        // 例: 3^4= (3^2)^2 = 9^2 = 81^1
        // 初期
        // 3^4
        // remainder=1
        // base=3
        // exp=4

        // i=0:
        // remainder = 1
        // base = 3 * 3 = 9
        // exp = 4 / 2 = 2

        // i=1:
        // remainder = 1
        // base = 9 * 9 = 81
        // exp = 2 / 2 = 1

        // i=2:
        // remainder = remainder * base = 81
        // base = 81 * 81
        // exp = 1 / 2 = 0
        let mut base = self.value;

        let mut remainder = 1;
        while exponent != 0 {
            if exponent % 2 == 1 {
                remainder = (remainder * base) % self.modulus;
            }
            base = (base * base) % self.modulus;
            exponent /= 2;
        }
        Self {
            modulus: self.modulus,
            value: remainder
        }
    }

}


use std::fmt;

// To use the `{}` marker, the trait `fmt::Display` must be implemented
// manually for the type.
// `{}` というマーカーを使用するためには、
// この型専用の`fmt::Display`というトレイトが実装されていなくてはなりません。
impl fmt::Display for Modint {
    // This trait requires `fmt` with this exact signature.
    // このトレイトは`fmt`が想定通りのシグネチャであることを要求します。
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Write strictly the first element into the supplied output
        // stream: `f`. Returns `fmt::Result` which indicates whether the
        // operation succeeded or failed. Note that `write!` uses syntax which
        // is very similar to `println!`.
        // 厳密に最初の要素を、与えられた出力ストリーム `f` に書き込みます。
        // `fmt::Result`を返します。これはオペレーションが成功したか否か
        // を表します。
        // `write!`は`println!`に非常によく似た文法を使用していることに注目。
        write!(f, "{}", self.value)
    }
}

// impl トレイト for 構造体 {}
// +演算子
impl std::ops::Add for Modint {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            modulus: self.modulus,
            value: (self.value + other.value) % self.modulus
        }
    }
}

// += 演算子
impl std::ops::AddAssign for Modint {
    fn add_assign(&mut self, other: Self) {
        *self = Self {
            modulus: self.modulus,
            value: (self.value + other.value) % self.modulus
        }
    }
}

// *演算子
impl std::ops::Mul for Modint {
    // The multiplication of rational numbers is a closed operation.
    type Output = Self;

    fn mul(self, other: Self) -> Self {
        Self {
            modulus: self.modulus,
            value: (self.value * other.value) % self.modulus 
        }
    }
}

// *=演算子
impl std::ops::MulAssign for Modint {
    fn mul_assign(&mut self, other: Self) {
        *self = Self {
            modulus: self.modulus,
            value: (self.value * other.value) % self.modulus
        }
    }
}

// -演算子
impl std::ops::Sub for Modint {
    type Output = Self;

    fn sub(self, other: Self) -> Self::Output {
        Self {
            modulus: self.modulus,
            // 引き算が負にならないようにmodulusを足しておく
            value: (self.modulus + self.value - other.value) % self.modulus
        }
    }
}

// -=演算子
impl std::ops::SubAssign for Modint {
    fn sub_assign(&mut self, other: Self) {
        *self = Self {
            modulus: self.modulus,
            // 引き算が負にならないようにmodulusを足しておく
            value: (self.modulus + self.value - other.value) % self.modulus
        };
    }
}

// /演算子
impl std::ops::Div for Modint {
    // The division of rational numbers is a closed operation.
    type Output = Self;

    fn div(self, other: Self) -> Self {
        if other.value == 0 {
            panic!("Cannot divide by zero-valued `Rational`!");
        }
        Self {
            modulus: self.modulus,
            value: (self.value * other.inverse().value) % self.modulus 
        }
    }
}

// /=演算子
impl std::ops::DivAssign for Modint {
    fn div_assign(&mut self, other: Self) {
        *self = Self {
            modulus: self.modulus,
            value: (self.value * other.inverse().value) % self.modulus
        };
    }
}


// isize型との演算
// ModInt * 整数
impl std::ops::Mul<isize> for Modint {
    type Output = Modint;

    fn mul(self, other: isize) -> Modint {
        Modint {
            modulus: self.modulus,
            value: (self.value * (other % self.modulus)) % self.modulus,
        }
    }
}

// 整数 * ModInt
impl std::ops::Mul<Modint> for isize {
    type Output = Modint;

    fn mul(self, other: Modint) -> Modint {
        Modint {
            modulus: other.modulus,
            value: ((self % other.modulus) * other.value) % other.modulus,
        }
    }
}

// ModInt *= 整数
impl std::ops::MulAssign<isize> for Modint {
    fn mul_assign(&mut self, other: isize) {
        *self = Self {
            modulus: self.modulus,
            value: (self.value * (other % self.modulus)) % self.modulus
        }
    }
}


// ModInt + 整数
impl std::ops::Add<isize> for Modint {
    type Output = Modint;

    fn add(self, other: isize) -> Modint {
        Modint {
            modulus: self.modulus,
            value: (self.value + other) % self.modulus,
        }
    }
}

// 整数 + ModInt
impl std::ops::Add<Modint> for isize {
    type Output = Modint;

    fn add(self, other: Modint) -> Modint {
        Modint {
            modulus: other.modulus,
            value: ((self % other.modulus) + other.value) % other.modulus,
        }
    }
}

// ModInt += 整数
impl std::ops::AddAssign<isize> for Modint {
    fn add_assign(&mut self, other: isize) {
        *self = Self {
            modulus: self.modulus,
            value: (self.value + (other % self.modulus)) % self.modulus
        }
    }
}







// ModInt - 整数
impl std::ops::Sub<isize> for Modint {
    type Output = Modint;

    fn sub(self, other: isize) -> Modint {
        Modint {
            modulus: self.modulus,
            value: (self.modulus + self.value - other) % self.modulus,
        }
    }
}

// 整数 - ModInt
impl std::ops::Sub<Modint> for isize {
    type Output = Modint;

    fn sub(self, other: Modint) -> Modint {
        Modint {
            modulus: other.modulus,
            value: (other.modulus + (self % other.modulus) - other.value) % other.modulus,
        }
    }
}

// ModInt -= 整数
impl std::ops::SubAssign<isize> for Modint {
    fn sub_assign(&mut self, other: isize) {
        *self = Self {
            modulus: self.modulus,
            value: (self.modulus + self.value - (other % self.modulus)) % self.modulus
        }
    }
}






// ModInt / 整数
impl std::ops::Div<isize> for Modint {
    type Output = Modint;

    fn div(self, other: isize) -> Modint {
        let other_modint = Modint::new(self.modulus, other);
        return self / other_modint
    }
}

// 整数 / ModInt
impl std::ops::Div<Modint> for isize {
    type Output = Modint;

    fn div(self, other: Modint) -> Modint {
        let self_modint = Modint::new(other.modulus, self);
        return self_modint / other
    }
}

// ModInt /= 整数
impl std::ops::DivAssign<isize> for Modint {
    fn div_assign(&mut self, other: isize) {
        let other_modint = Modint::new(self.modulus, other);
        *self = *self / other_modint;
    }
}





// mod p を法とした時の割り算 a / b の値
fn mod_dev(a: isize, b: isize, modulo: isize) -> isize {
    return a * mod_inverse(b, modulo) % modulo
}

// mod p を法とした時の逆数(逆元という) 1 / b の値
fn mod_inverse(a: isize, modulo: isize) -> isize {
    // フェルマーの小定理
    //     a^(p-1) = 1     (mod p)
    // <=> a * a^(p-2) = 1 (mod p)
    // <=> 1 / a = a^(p-2) (mod p)
    // (ただし、法pは素数)

    return mod_pow(a, modulo - 2, modulo)
}

// mod p を法とした時の累乗
// base^(x) % mod を繰り返し二乗法により、O(log2(x))の計算量で求める　(O(x)だとTLE)
// No.69参照
fn mod_pow(mut base: isize, mut exponent: isize, modulo: isize) -> isize {
    // 例: 3^4= (3^2)^2 = 9^2 = 81^1
    // 初期
    // 3^4
    // remainder=1
    // base=3
    // exp=4

    // i=0:
    // remainder = 1
    // base = 3 * 3 = 9
    // exp = 4 / 2 = 2

    // i=1:
    // remainder = 1
    // base = 9 * 9 = 81
    // exp = 2 / 2 = 1

    // i=2:
    // remainder = remainder * base = 81
    // base = 81 * 81
    // exp = 1 / 2 = 0

    let mut remainder = 1;
    while exponent != 0 {
        if exponent % 2 == 1 {
            remainder = (remainder * base) % modulo;
        }
        base = (base * base) % modulo;
        exponent /= 2;
    }
    return remainder
}