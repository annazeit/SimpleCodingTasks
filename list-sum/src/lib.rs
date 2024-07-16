pub fn add(list: Vec<i32> ) -> i32 {
    let mut sum = 0;
    for i in list {
        sum += i;
    }
    return sum;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let list = Vec::from([1,2,3,4,5]);
        let expected_result = 15;
        let actual_result = add(list);
        assert_eq!(expected_result, actual_result);
    }
}
