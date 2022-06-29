pub fn run() {
    let person: (&str, &str, i8) = ("Renan", "Andrade", 36);

    println!("{} from {} and is {}", person.0, person.1, person.2);
}
