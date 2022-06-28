pub fn run() {
    let name = "Renan";
    let mut age = 36;
    println!("My name is {} and I am {}", name, age);

    age = 38;

    println!("My name is {} and I am {}", name, age);

    // Define constants
    const ID: i32 = 001;
    println!("ID: {}", ID);

    // Assign multiple vars
    let (my_name, my_age) = ("Bradd", 37);
    println!("{} is {}", my_name, my_age);
}
