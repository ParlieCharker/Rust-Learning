fn main() {
    // arithmentic

    // let x = (i32::MAX as i64) +1; // unsigned 8 bit, can be positive or negative
    // let y: i32 = 10; // signed 8 bit, can be positive or negative
    //                 //
    //                 //
    // let z = x as i32 / y;
    // println!("{}", z)
    //
    //

    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("could not read line");

    println!("{}", input);

    let int_input: i64 = input.trim().parse().unwrap();

    println!("{}", int_input + 5);


}
