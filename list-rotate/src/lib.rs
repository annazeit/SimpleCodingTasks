use std::io;

pub fn rotate (list: &mut Vec<i32>, rotate_num: usize) {
    let mut source_index: usize = 0;
    let list_len = list.len();
    let mut source_value: i32 = list[source_index];

    loop {
        let mut destination_index: usize = list_len - (rotate_num) + source_index;
        if destination_index >= list_len {
            destination_index -= list_len;
        }

        let destination_value = list[destination_index];
        list[destination_index] = source_value;
        source_index = destination_index;
        source_value = destination_value;

        if destination_index == 0 {
            break
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let mut list: Vec<i32> = Vec::from([0, 1, 2, 3, 4, 5, 6]);

        let expected_result: Vec<i32> = Vec::from([2, 3, 4, 5, 6, 0, 1]);
        rotate(&mut list, 2);

        assert_eq!(expected_result, list);
    }

    #[test]
    fn it_works2() {
        let mut list: Vec<i32> = Vec::from([0, 1, 2, 3, 4, 5, 6]);

        let expected_result: Vec<i32> = Vec::from([4,5,6,0,1,2,3]);
        rotate(&mut list, 4);

        assert_eq!(expected_result, list);
    }

}
