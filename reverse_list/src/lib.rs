pub fn reverse() -> Vec<i32>{
    let old_list = [1,2,3,4,5];
    let mut new_list = Vec::new();
    let mut element= old_list.len();

    for i in old_list {
        element -= 1;
        new_list.push(old_list[element]);
    }
    return new_list;

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let actual = reverse();
        let expected = vec![5, 4, 3, 2, 1];
        assert_eq!(expected, actual);
    }
}
