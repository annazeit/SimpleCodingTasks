pub fn palindrome() -> bool{

    let mut boolean = false;
    let string = String::from("poop");
    let char_list: Vec<char> = string.chars().collect();
    let mut count = 1;

    let mut last_char = char_list[char_list.len() - 1];
    let mut first_char = char_list[0];

    for i in &char_list {
        if &first_char == &last_char {
            last_char = char_list[char_list.len() - count];
            first_char = char_list[0 + count];
            count += 1;
            boolean = true;
        }
    }
    return boolean;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let expected_result = true;
        let actual_result = palindrome();

        assert_eq!(expected_result, actual_result);}
}