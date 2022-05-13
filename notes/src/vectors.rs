pub fn run(){
    let mut numbers: Vec<i32> = vec![1,2,3,4,5];
    println!("Original: {:?}", numbers);

    // Push
    numbers.push(50);
    numbers.push(10);
    println!("Pushed: {:?}", numbers);

    // Pop
    numbers.pop();
    println!("Popped: {:?}", numbers);

    // Slice
    let slice: &[i32] = &numbers[0..2];
    println!("Sliced: {:?}", slice);
    
    // Loop
    for x in numbers.iter() {
        println!("{}", x);
    }

    // Map
    for x in numbers.iter_mut() {
        *x*=2;
    }
    println!("Multiplied: {:?}", numbers);
}