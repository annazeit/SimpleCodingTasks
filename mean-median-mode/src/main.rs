use std::collections::HashMap;

fn main () {
    let mut list = vec![6,10,8,10];

    let mut total = 0;
    //calculate mean
    let mut mean = 0;
    for i in &list {
        total += i;
    }//fshasjkvhkshjczdgfxhkdzffjfjgjdfxhfsdsvx
    mean = total / list.len();
    println!("total: {total}");
    println!("mean: {mean}");

    //calculate median
    let int_length = list.len();
    list.sort_by(|a, b| a.cmp(b));

    let mut answer = int_length / 2;
    let median_num = list[answer];

    if (int_length % 2 == 0){
        let median = (list[answer] + list[answer - 1]) / 2;
    }
    println!("median: {median_num}");

    //calculate mode
    let mut num_count = HashMap::new();
    for i in &list {
        let count = num_count.entry(i).or_insert(0);
        *count += 1;
    }
    let mut mode = 0;
    let mut prev_value = 0;
    for (key, value) in &num_count {
        if *value > prev_value {
            prev_value = *value;
            mode = **key;
        }
    }
    println!("mode: {mode}")
}
