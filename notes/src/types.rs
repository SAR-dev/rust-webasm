pub fn run(){
    // Max size of a integer type u8, i8, u16, i16, u32, i32, u64, i64, u128, i128
    println!("Max of i32: {}", std::i32::MAX);
    println!("Max of i64: {}", std::i64::MAX);

    // Max size of a float type f32, f64
    println!("Max of f32: {}", std::f32::MAX);

    let int1 = 8;
    let int2:i32 = 16;
    println!("int1: {}, int2: {}", int1, int2);

    let bool1 = true;
    let bool2:bool = false;
    println!("bool1: {}, bool2: {}", bool1, bool2);
}