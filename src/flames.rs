fn remove_index(s:String, index:usize) -> String{
    let mut n12: String = s.chars().skip(0).take(index).collect();
        let mut n21: String = s.chars().skip(index+1).take(s.len()).collect();
        n12 = n12.to_owned();
        n21 = n21.to_owned();
        let new_name : String =  format!("{}{}",n12,n21);
        new_name
}
pub fn handle_flames() {
    let mut name1: String = read!();
    let mut name2: String = read!();

    let mut c = name1.len() + name2.len();
    let mut len1 = name1.len();
    while len1 > 0 {

        let ch1 = match name1.chars().next() {
            Some(expr) => expr,
            None => '\0',
        };

        let check = match name2.find(ch1) {
            Some(result) => result as isize,
            None => -1
        };

        name1 = remove_index(name1, 0);   
        if check != -1 {
            c -= 2;
            name2 = remove_index(name2, check as usize);
        }

        len1 = len1 - 1;
        
    }

    let mut test_string:String = String::from("FLAMES");
    let mut p = 0;
    len1 = test_string.len();

    while len1 > 1{
        p = if (c+p)%len1 == 0  {len1-1}else {((c+p)%len1)-1};
        test_string = remove_index(test_string.clone(), p);
        len1 = test_string.len();
    }

    println!("{}", test_string);
}