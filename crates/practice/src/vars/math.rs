use std::collections::HashMap;

pub fn practice_math() {
    //let mut int_list = vec![1, 2, 3, 4, 5, 6];
    let mut int_list = vec![4, 2, 5, 8, 1, 9, 0, 2, 111, 111, 111];

    // Mean
    let mut sum = 0;
    for i in &int_list {
        sum += i;
    }
    let mean = sum as f32 / int_list.len() as f32;
    println!("[Mean] {}", mean);

    // Median
    int_list.sort();
    match int_list.len() % 2 {
        0 => println!("[Median] {}", int_list[int_list.len() / 2 - 1]),
        1 => println!("[Median] {}", int_list[int_list.len() / 2]),
        _ => (),
    }

    // Mode
    let mut map = HashMap::new();
    for i in &int_list {
        let i_str: String = i.to_string();
        let count = map.entry(i_str).or_insert(0);
        *count += 1;
    }
    println!("[Map] {:?}", map);

    let max = map.iter()
        .max_by(|a, b| a.1.cmp(&b.1))
        .unwrap();
    println!("[Mode] Value: {}, Count: {}", max.0, max.1);
}