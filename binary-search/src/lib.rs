fn binary_search(list: Vec<i32>, target: i32) -> usize {
    let mut left: usize = 0;
    let mut right = list.len() - 1;

    while left <= right {
        let mid = (left + right) / 2;

        if list[mid] == target {
            return mid;
        }
        else if list[mid] < target {
            left = mid + 1;
        }
        else {
            right = mid - 1;
        }
    }
    return 0;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let mut list: Vec<i32> = Vec::from([1,4,5,7,9,12]);
        list.sort();
        let expected = 3;
        assert_eq!(expected, binary_search(list, 7))
    }
}
