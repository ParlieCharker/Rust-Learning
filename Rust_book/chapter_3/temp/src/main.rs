fn main() {
    let x: f32 = 68.0;

    let x_in_c = convert_to_celsius(x);
    let x_in_f = convert_to_fahrenheit(x);
    println!("{x} degrees fahrenheit in celsius is {x_in_c}");
    println!("{x} degrees celsius in fahrenheit is {x_in_f}");
}

fn convert_to_fahrenheit(x: f32) -> f32 {
    (x * 9.0 / 5.0) + 32.0
}

fn convert_to_celsius(x: f32) -> f32 {
    (x - 32.0) * 5.0 / 9.0
}
