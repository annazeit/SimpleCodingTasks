use std::io::{self, BufRead};
use std::collections::HashMap;

fn main() {
    while true {
        println!("Please enter your name: ");
        let mut name = String::new();
        std::io::stdin().read_line(&mut name);println!("Please enter your department: ");
        let mut department = String::new();
        std::io::stdin().read_line((&mut department));

        if (name != "quit" && department != "quit") {
            let mut employee_department = HashMap::new();
            employee_department.insert(&name, &department);

            for (key, value) in &employee_department {
                println!("Person: {key}Department: {value}")
            }
        }
        else {
            break
        }
    }
}
