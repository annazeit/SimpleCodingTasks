fn leap() -> bool {
    println!("These are the leap years or the next 20 years.");
    let mut current_leap_year = 2024;
    let mut leap_year_count = 0;

    loop {
        leap_year_count += 1;
        if leap_year_count <= 20 {

            if current_leap_year % 4 == 0 {
                println!("{current_leap_year}");
                current_leap_year += 4;
            }
        }
        else {
            break
            return false;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let expected_result = false;
        let actual_result = leap();
        assert_eq!(expected_result, actual_result);
    }
}
