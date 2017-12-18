pub fn handle_grades(){
    let n:usize = read!();
    let mut result:Vec<usize> = Vec::with_capacity(n);

    for _i in 0..n {
        let mut grade:usize = read!();
        if grade > 37{
            let new_grade = (grade/5 + 1)*5;
            if new_grade <= grade+2{
                grade = new_grade;
            }
        }
        result.push(grade);
    }
    for g in result{
        println!("{}", g);
    }
}