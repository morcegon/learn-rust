pub fn run() {
    let mut numbers: Vec<i32> = vec![1, 2, 3, 4];

    // Add on to vectors
    numbers.push(5);

    println!("Add: {:?}", numbers);

    // Pop off the last number
    numbers.pop();
    println!("Add: {:?}", numbers);

    // Loop through vector values
    for number in numbers.iter() {
        println!("Number {}", number);
    }

    // Loop and mutate values
    for number in numbers.iter_mut() {
        *number *= 2;
    }

    println!("Numbers Vec: {:?}", numbers);
}
