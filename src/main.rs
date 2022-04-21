use std::env;

fn main() {
    let args: Vec<_> = env::args().collect();

    let query = &args[1];
    let filename = &args[2];

    println!("ファイル {} の中にある", filename);
    println!("{} を探す", query);
}
