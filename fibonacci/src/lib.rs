pub fn fibonacci() -> Vec<i32>{
    let mut list = Vec::with_capacity(10);
    let mut previous_num = 0;
    let mut current_num = 1;

    loop {
        if list.len() == 10 {
            break
        }
        let sum_num = previous_num + current_num;
        previous_num = current_num;
        current_num = sum_num;
        list.push(previous_num);
    }
    return list;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
    let expected_result = Vec::from([1,1,2,3,5,8,13,21,34,55]);
    let actual_result = fibonacci();
    }
}
