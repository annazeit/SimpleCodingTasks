pub fn combine_sort(list1: Vec<i32>, list2: Vec<i32>) -> Vec<i32>{
    let mut combined_list = Vec::new();
    for i in list1 {
        combined_list.push(i);
    }
    for i in list2 {
        combined_list.push(i);
    }
    combined_list.sort();

    return combined_list;
}
pub fn combine_alternatingly(list1: Vec<i32>, list2: Vec<i32>) -> Vec<i32> {
    let mut combined_list = Vec::<i32>::with_capacity(
        list1.len() + list2.len());

    if list1.len() != list2.len() { panic!("The function requires two lists of the same length.") }

    for i in 0 .. list1.len(){
        combined_list.push(list1[i]);
        combined_list.push(list2[i]);

    }

    return combined_list;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn list_combine_sort_works() {
        let list1 = Vec::from([1,2,3]);
        let list2 = Vec::from([6,5,4]);
        let expected_result = Vec::from([1,2,3,4,5,6]);
        let actual_result = combine_sort(list1, list2);

        assert_eq!(expected_result, actual_result);
    }

    #[test]
    fn combine_alternatingly_works() {
        let list1 = Vec::from([1,3,5]);
        let list2 = Vec::from([2,4,6]);

        let expected_result = Vec::from([1,2,3,4,5,6]);
        let actual_result = combine_sort(list1, list2);

        assert_eq!(expected_result, actual_result);
    }
}
