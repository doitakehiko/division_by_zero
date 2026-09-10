use std::io;

fn main() {
    //let x = 1 / 0; //コンパイルエラー
    let mut input = String::new();
    println!("数字を入力してください:");
    io::stdin().read_line(&mut input).unwrap();
    
    // ユーザーが「0」と入力した場合、コンパイル時には検知できない
    let denominator: i32 = input.trim().parse().unwrap_or(0); 
    let x = 1 / denominator; 
    
    println!("{}", x);
}
/*
/projects/divisionbyzero$ cargo run
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.01s
     Running `target/debug/divisionbyzero`
数字を入力してください:
0

thread 'main' (13709) panicked at src/main.rs:11:13:
attempt to divide by zero
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace

*/
