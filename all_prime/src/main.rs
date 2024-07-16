

fn main() {
    let list: Vec<i32> = (1..1000000).collect();

    for i in list {
        let number = i;
        if is_prime(number) {
            println!("{number}");
        }
    }
}

fn is_prime(num: i32) -> bool{
    if num <= 1 {
        return false;
    }

    for i in 2..num {
        if num % i == 0 {
            return false;
        }
    }

    return true;
}