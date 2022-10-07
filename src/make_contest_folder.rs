// What this code does.
// 1. AtcoderBeginnerContestの特定の回用のフォルダ、ソースコード(*.rs)、ソースコードを短縮名でcargo runする為のshellscriptを作成
// 2. Cargo.tomlに[[bin]]を追記（上記で生成したソースコードをcargo runで実行できるようにするのが目的）

// How to use
// $ cargo run --bin make_contest_folder <AtcoderBeginnerContest Number>
// (Example)
// $ cargo run --bin make_contest_folder 77

use std::{path, fs, env};
use std::fs::File;
use std::io::{Write, BufWriter};
use std::fs::OpenOptions;

fn main() {
    // コマンドライン引数を得る
    let args: Vec<String> = env::args().collect();
    println!("Your inputs are {:?}.", args);

    // フォルダー名を生成
    let folder_name = path::PathBuf::from(
        format!("abc{:03}", args[1].parse::<usize>()
        .expect("please input contest number. For example, if you treat abc013, input 13.")));
    
    // フォルダを作成するか?
    if folder_name.exists() {
        // フォルダを作成しない
        println!("{:?} already exists.", folder_name);
    } 
    else {
        // フォルダを作成する
        println!("making... {:?}", folder_name);
        fs::create_dir(folder_name.clone());

        // ***.rsに書き込む内容
        // proconioを必ず使うので用意
        let content0 = String::from("use proconio::input;\nuse itertools::Itertools;\nuse std::cmp::{max, min};\nuse std::cmp::Ordering;\nuse std::collections::VecDeque;\nuse std::collections::BinaryHeap;\nuse proconio::marker::Chars;\nfn main() {\n    input! {\n        \n    }\n}");
        let content0_b = content0.as_bytes();

        // Cargo.tomlに書き込む内容
        let content1 = "\n\n[[bin]]\n";
        let content1_b = content1.as_bytes();
        // Cargo.tomlを開く
        let cargo_toml_path = "../Cargo.toml";
        let file = OpenOptions::new()
            .append(true)
            .open(cargo_toml_path)
            .expect("Cannnot open Cargo.toml");
        let mut writer = BufWriter::new(file);

        // ファイルを生成
        let filename_list = "abcdefgh".chars();
        for i in filename_list {
            // ***.rsを生成
            let file_path = folder_name.clone().join(i.to_string() + ".rs");
            {
                // ***.rsを生成
                let mut fp = File::create(file_path.clone()).expect(&format!("cannnot make file {:?}", file_path.clone()));
                println!("making... {:?}", file_path);
                // ***.rsに書き込み
                fp.write_all(content0_b).expect(&format!("cannot write contents to the script ({:?}).", file_path));
            }

            // Cargo.tomlの末尾に追記
            let src_path = path::PathBuf::from("src/");
            let file_path_from_src_buf = src_path.join(file_path.clone());
            let file_path_from_src = file_path_from_src_buf.to_string_lossy();
            let content2 = String::from(format!("name = \"{}_{}\"\n", folder_name.to_string_lossy(), i));
            let content2_b = content2.as_bytes();
            let content3 = String::from(format!("path = \"{}\"", file_path_from_src));
            let content3_b = content3.as_bytes();
            writer.write(content1_b).expect("cannot write to Cargo.toml");
            writer.write(content2_b).expect("cannot write to Cargo.toml");
            writer.write(content3_b).expect("cannot write to Cargo.toml");
        }
        println!("Cargo.toml has been edited successfully.");
        
        // run.shを生成
        let template_run_file_path = path::PathBuf::from("template_run.sh");
        let run_file_path = folder_name.clone().join("run.sh");
        let mut fp = File::create(run_file_path.clone()).expect(&format!("cannnot make file {:?}", run_file_path.clone()));
        fs::copy(template_run_file_path, run_file_path.clone());
        println!("making... {:?}", run_file_path);       
    }
}