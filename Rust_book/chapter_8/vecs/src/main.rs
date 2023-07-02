enum SpreadsheetCell {
    Int(i32),
    Float(f64),
    Text(String),
}


fn main() {
    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("Blue")),
        SpreadsheetCell::Float(23.5),
    ];
    // vectors can store more than one value in a data structure, only values of the same type

    // let v: Vec<i32> = Vec::new();

    let mut v = vec![100, 32, 57];

    for i in &mut v {
        *i += 50;
    }
}

