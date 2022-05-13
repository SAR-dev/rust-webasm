pub fn run(){
    // Immutable
    let numbers: [i32; 5] = [1,2,3,4,5];
    println!("Numbers: {}-{}", numbers[0], numbers[1]);

    // Mutable
    let mut evens: [i32; 5] = [2,4,6,8,10];
    evens[4] = 12;
    println!("Evens: {:?}", evens);

    // Slice
    let slice: &[i32] = &evens[0..2];
    println!("Sliced: {:?}", slice);
}