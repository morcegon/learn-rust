use std::mem;

pub fn run() {
    let mut numbers: [i32; 5] = [1, 2, 3, 4, 5];

    println!("{:?}", numbers);

    // get single val
    println!("Single value: {}", numbers[0]);

    // Reassign a value
    numbers[2] = 20;
    println!("Single value: {}", numbers[2]);

    // Get array length
    println!("Array length: {}", numbers.len());

    // Arrays are stack allocated
    println!("Array occupies: {} bites", mem::size_of_val(&numbers));

    // Get slice
    let slice: &[i32] = &numbers[1..3];
    println!("Slice {:?}", slice);
}
