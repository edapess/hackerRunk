fn is_matrix_square(_arr: &[Vec<i32>]) -> bool {
    let mut is_square = true;
    let arr_length = _arr.len();

    for row in _arr {
        if arr_length != row.len() {
            is_square = false;
            break;
        }
    }
    return is_square;
}

fn diagonal_difference(arr: &[Vec<i32>]) -> i32 {
    if !is_matrix_square(arr) {
        panic!("Matrix is not square");
    } else {
        let mut sum_of_left_to_right = 0;
        let mut sum_of_right_to_left = 0;
        for i in 0..arr.len() {
            println!("{:?}", i);

            sum_of_left_to_right += arr[i][i];
            sum_of_right_to_left += arr[i][arr.len() - 1 - i]
        }

        return (sum_of_left_to_right - sum_of_right_to_left).abs();
    }
}

fn main() {
    let arr = vec![vec![11, 2, 4], vec![4, 5, 6], vec![10, 8, -12]];
    let result = diagonal_difference(&arr);
    println!("{}", result);
}
