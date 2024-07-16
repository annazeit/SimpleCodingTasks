use std::io;

fn main() {
}

pub fn greet(input: String) {
    println!("Please enter your name:");
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");

    if input.contains("Anna") | input.contains("Kate"){
        println!("Hello {input}")
    }
    else{
        println!("Go away {input}");
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_greets() {
        let result = greet(String::from("Anna"));
        assert!(result.contains("Anna"));
    }
}