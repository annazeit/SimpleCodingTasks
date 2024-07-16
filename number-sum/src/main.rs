use std::io;

fn main() {
    number(String::new())
}
pub fn number(input: String) {
    println!("Enter the number:");

    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line.");
    let input: i32 = match input.trim().parse() {
        Ok(num) => num,
        Err(_) => panic!(),
    };

    println!("\nSum or product?");
    let mut answer = String::new();
    io::stdin().read_line(&mut answer).expect("Failed to read line.");

    let list: Vec<i32> = (1..input + 1).collect();
    let mut sum  = 0;
    let mut product = 0;

    println!("\nNumbers:");

    for i in list {
        if i % 3 == 0 || i % 5 == 0 {
            if answer.contains("sum") {
                sum += i;
                println!("{i}");
            }
            else if answer.contains("product") {
                product *= i;
                println!("{i}");
            }
        }
    }
    println!("\nThe sum of the numbers is {sum}");
    println!("\nThe product of the numbers is {product}");
}
