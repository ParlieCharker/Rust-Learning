use std::io;

fn main() {
    let mut input = String::new();


    io::stdin().read_line(&mut input).expect("fail to read line");

    println!("{}", input);
}
