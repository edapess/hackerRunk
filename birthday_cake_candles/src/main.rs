fn birthdayCakeCandles(candles: &[i32]) -> i32 {
    let mut copy_candles = candles.to_vec();
    copy_candles.sort();
    let max_value = copy_candles[copy_candles.len() - 1];
    let mut tallest_count = 0;
    for i in 0..copy_candles.len() {
        if (copy_candles[i] == max_value) {
            tallest_count += 1;
        }
    }
    return tallest_count;
}

fn main() {
    let result = birthdayCakeCandles(&vec![3, 2, 1, 3]);
    println!("{}", result);
}
