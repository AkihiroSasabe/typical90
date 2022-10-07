"""90題分のrsファイル作成とCargo.tomlの編集([[bin]]の追加)を行う"""

rs_file_contents = """use proconio::input;
use itertools::Itertools;
use std::cmp::{max, min};
use std::cmp::Ordering;
use std::collections::VecDeque;
use std::collections::BinaryHeap;
use std::collections::HashMap;
use proconio::marker::Chars;
use std::f64::consts::PI;
use std::mem::swap;
fn main() {
    input! {
        
    }
}"""

cargo_toml_content = '''
[[bin]]
name = "{:0=3}"
path = "src/{:0=3}.rs"
'''

for i in range(90):
    title = i + 1
    filename = "{:0=3}.rs".format(title)
    # .rsファイルの作成
    with open(filename, mode="w") as f:
        f.write(rs_file_contents)
    
    # Cargo.tomlの編集
    with open("../Cargo.toml", mode="a") as f:
        f.write(cargo_toml_content.format(title, title))