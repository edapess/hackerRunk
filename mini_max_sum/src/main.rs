fn minimax_sum(arr: &[i32]) {
    println!("------{:?}", arr);
    let mut sorted_arr = arr.to_vec();
    sorted_arr.sort();
    println!("++++++{:?}", sorted_arr);
    let mut min_sum: i64 = 0;
    let mut max_sum: i64 = 0;
    for i in 0..sorted_arr.len() {
        if i < sorted_arr.len() - 1 {
            min_sum += sorted_arr[i] as i64;
        }
        if i > 0 {
            max_sum += sorted_arr[i] as i64;
        }
    }
    println!("{} {}", min_sum, max_sum);
}

fn main() {
    minimax_sum(&vec![1, 3, 5, 7, 9]);
    minimax_sum(&vec![1, 2, 3, 4, 5]);
    minimax_sum(&vec![256741038, 623958417, 467905213, 714532089, 938071625]);
}
