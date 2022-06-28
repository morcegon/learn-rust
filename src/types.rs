pub fn run() {
    // Default is "132"
    let x = 1;

    // Default is "f64"
    let y = 2.5;

    // Find max size
    println!("Max i32: {}", std::i32::MAX);
    println!("Max i64: {}", std::i64::MAX);

    // Boolean
    let is_active = true;

    // Get boolean from expression
    let is_greater = 10 > 5;

    println!("{:?}", (x, y, is_active, is_greater));
}
