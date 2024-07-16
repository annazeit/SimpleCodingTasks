pub fn digits (num: i32) -> Vec<i32>{
    let string = format!("{num}");
    println!("{string}");

    let list = string
        .into_bytes()
        .into_iter()
        .map(|b| b as i32 - 48)
        .collect::<Vec<i32>>();
    println!("{:?}", list);

    return list;
} 

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let num = 1234567;

        let expected_ans = Vec::from([1,2,3,4,5,6,7]);
        let actual_ans = digits(num);
    }
}
