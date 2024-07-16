fn finding_largest_num () -> i32{
    let list = [3,67,900,6,54,21,9];
    let mut largest_num = 0;

    for i in list {
        if largest_num >= i {
            println!("{largest_num}")
        }
        else {
            largest_num = i;
        }
    }
    return largest_num;
}

#[cfg(test)]
mod tests {
    use super::*;
    use finding_largest_num;

    #[test]
    fn it_works() {
        let result = finding_largest_num();
        assert_eq!(900, result);
    }
}