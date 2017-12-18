pub fn remove_index(s:String, index:usize) -> String{
    let mut n12: String = s.chars().skip(0).take(index).collect();
        let mut n21: String = s.chars().skip(index+1).take(s.len()).collect();
        n12 = n12.to_owned();
        n21 = n21.to_owned();
        let new_name : String =  format!("{}{}",n12,n21);
        new_name
}