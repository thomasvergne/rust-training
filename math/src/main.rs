fn add (number1: i32, number2: i32) -> i32 {
    return number1 + number2;
}

fn sub (number1: i32, number2: i32) -> i32 {
    return number1 - number2;
}

fn main() {
    println!("3 + 5 = {}", add(3, 5));
    println!("6 - 2 = {}", sub(6, 2));
}
