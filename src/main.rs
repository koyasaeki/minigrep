use std::env;
use std::fs::File;
use std::io::Read;

fn main() {
    let args: Vec<_> = env::args().collect();

    let query = &args[1];
    let filename = &args[2];

    println!("ファイル {} の中にある", filename);
    println!("{} を探す", query);

    let mut f = File::open(filename).expect("file not found");

    let mut contents = String::new();
    f.read_to_string(&mut contents)
        .expect("something went wrong reading the file");

    println!("With text:\n{}", contents);
}
