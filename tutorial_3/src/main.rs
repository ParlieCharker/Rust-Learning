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
    const seconds_in_minute: u128 = 60; // Consts are to be defined in capital snake case
                                        //
    println!("x is: {}", seconds_in_minute);
    println!("x is: {}", seconds_in_minute);
}
