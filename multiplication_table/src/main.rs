fn main() {
    multiply();
}
fn multiply() {
    let list: Vec<i32> = (1..13).collect();

    for i in &list {

        let first_num = i;

        for i in &list{

            let second_num = i;
            let ans = first_num * second_num;
            println!("{first_num} * {second_num} = {ans}");
        }
    }
}
