use std::io;

pub fn check() -> bool {

    let list = [1,2,3,4,5,6,7,8,9];
    println!("Please print the number you want to check for: ");

    let mut element = String::new();
    io::stdin().read_line(&mut element).expect("Failed to read line");
    let element: i32 = match element.trim().parse() {
        Ok(num) => num,
        Err(_) => panic!(),
    };

    if list.contains(&element) {
        println!("Yes! The number {element} is in the list!");
        return true;
    }
    else { return false; }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {

    let expected_result = true;
    let actual_result = check();

    assert_eq!(expected_result, actual_result);
    }
}
