fn plusMinus(arr: &[i32]) {
    //proportion of positive values
    //proportion of negative values
    //proportion of zeros
    let mut positive_count = 0;
    let mut negative_count = 0;
    let mut zero_count = 0;
    let arr_length = arr.len() as f64;
    for i in 0..arr.len() {
        let value = arr[i];
        if value > 0 {
            positive_count += 1;
        } else if value < 0 {
            negative_count += 1;
        } else {
            zero_count += 1;
        }
    }
    let positive_proportion = positive_count as f64 / arr_length;
    let negative_proportion = negative_count as f64 / arr_length;
    let zero_proportion = zero_count as f64 / arr_length;

    println!("{:.6}", positive_proportion);
    println!("{:.6}", negative_proportion);
    println!("{:.6}", zero_proportion);
}

fn main() {
    let arr = [1, 1, 0, -1, -1];

    plusMinus(&arr);
}
