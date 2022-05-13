pub fn run(){
    // Immutable
    let immutable_str = "Immutable";
    // When we want to modify string later
    let mut mutable_str = String::from("Mutable");

    println!("Immutable -{}-, Mutable -{}-", immutable_str, mutable_str);

    // Get length
    println!("Length: {}", mutable_str.len());

    // Push
    mutable_str.push('s');
    println!("Character Pushed: {}",mutable_str);

    mutable_str.push_str(" Strings");
    println!("String Pushed: {}",mutable_str);

    // Bytes Capacity
    println!("Capacity: {} bytes", mutable_str.capacity())

}