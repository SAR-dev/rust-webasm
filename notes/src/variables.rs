pub fn run(){
    // let is immutable by default
    let num1 = 10;
    let num2 = 20;
    println!("num1: {}, num2: {}", num1, num2);

    // let can be converted into mutable
    let name1 = "Rafi";
    let mut name2 = "Sayed";
    println!("Old => name1: {}, name2: {}", name1, name2);
    name2 = "Taki";
    println!("New => name1: {}, name2: {}", name1, name2);

    // constants need data type to be declared
    const ID: i32 = 1;
    println!("ID: {}", ID);


    // multiple variable declaration
    let (country_1, country_2) = ("Bangladesh", "Japan");
    println!("{} and {}", country_1, country_2);
}