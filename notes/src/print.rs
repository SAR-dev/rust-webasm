pub fn run() {
    // Basic printing
    println!("Age of Rafi is 27");
    // Formatted printing
    println!("Age of {} is {}", "Rafi", 27);
    // Positional arguments printing
    println!(
        "{0} has scored {1} marks more than {0}'s brother",
        "Rafi", 25
    );
    // Named arguments printing
    println!(
        "{name} has scored {mark} marks more than {name}'s brother",
        name = "Rafi",
        mark = 25
    );
}
