pub fn run() {
    // Print to console
    println!("Hello from print.rs file");

    // Formatting
    println!("{} is from {}", "Bradd", "Masss");

    // Positional arguments
    println!(
        "{0} is from {1} and {0} likes to {2}",
        "Bradd", "Mass", "code"
    );

    // Named Arguments
    println!(
        "{name} likes to play {activity}",
        name = "john",
        activity = "baseball"
    );

    // Placeholder traits
    println!("Binary: {:b} - Hex: {:x} - Octal: {:o}", 10, 10, 10);

    // Placeholder for debug trait
    println!("{:?}", (12, true, "Renan"));

    // Basic Math
    println!("10 + 10 = {}", 10 + 10);
}
