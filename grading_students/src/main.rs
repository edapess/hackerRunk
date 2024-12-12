fn next_multiple_of_5(number: i32) -> i32 {
    if number % 5 == 0 {
        return  number;
    }else{
        return number + (5 - number % 5)
    }
}

fn grading_students(grades: &[i32]) -> Vec<i32> {
    let mut rounded_grades = Vec::new();
    for grade in grades {
        let smallest_grade= 38;
        let next_multiple_of_five =  next_multiple_of_5(*grade);
        let difference = next_multiple_of_five - grade;
        if *grade < smallest_grade { 
            rounded_grades.push(*grade);
        }else if  difference < 3 {
            rounded_grades.push(next_multiple_of_five);
        }else {
            rounded_grades.push(*grade);
        }
        
    };
    return rounded_grades;
}

fn main() {
    let num_vectors =   vec![73,67,38,33];
    let result = grading_students(&num_vectors);
    println!("{:?}",result);
}
