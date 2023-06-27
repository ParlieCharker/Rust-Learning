fn main() {
    let mut x = 5;
    println!("The value of x is: {x}");
    {
        let x = "Hello World";
        println!("The value of x is: {x}");
    }
    x = 6;
    println!("The value of x is: {x}");

    // Data types

    // Scalar Types - A quantity that is completely described by it's quantity- they represent a single value
    // Integers
    // floats -> all are signed f32 and f64
    // bools
    // characters single quotes ->

    // Integer type - a number without a fractional component
    // signed ints use 2's complement = -(2^n-1) to 2^n-1 - 1 inclusive. N is the number of bits involved

    // signed and unsigned
    // i8 u8
    // i16 u16
    // i32 u32
    // i64 u64
    // i128 u128
    // isize usize -> architecture specific

    // decimal 98_222
    // hex 0xff
    // Octal 0077
    // binary 0b1111_0000
    // Byte (u8 only) b'A'

    // compound types. -> Group multiple values into one type

    // Tuples
    // fixed length but can contain a number of values in a single type

    let tup: (i32, f64, u8) = (500, 6.4, 1);
    let (x, y, z) = &tup;

    println!("x is {x}, Y is {y} and Z is {z}");
    println!("the first value of tup is { }", tup.0);

    let my_array: [i8; 5] = [1, 2, 3, 4, 5];
    println!("The first element of my array is: { }", my_array[0]);
}
