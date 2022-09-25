fn main() {
    let x = 4;
    println!("x is: {}", x);

    {
        let x = 2;
        println!("x is: {}", x);
    }

    let x = "Hello";
    println!("x is: {}", x); // test comment

    // CONsTS
    const SECONDS_IN_MINUTE: u128 = 60; // Consts are to be defined in capital snake case
                                        //
    println!("x is: {}", SECONDS_IN_MINUTE);
    println!("x is: {}", SECONDS_IN_MINUTE);
}
