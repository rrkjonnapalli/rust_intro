pub fn count_valleys(){
    let n:usize = read!();
    let s:String = read!();

    let mut sea_level = 0;
    let mut valleys = 0;
    for ch in s.chars() {
        if ch == 'U' {
           sea_level = sea_level+1;
           if sea_level == 0 {
               valleys = valleys + 1;
           } 
        }else{
            sea_level = sea_level -1;
        }
    }
    println!("{}", valleys);
}