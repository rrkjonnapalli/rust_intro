fn search_pattern (mut text:String, mut pattern:String) -> Vec<isize> {
    let text_bytes = text.as_bytes();
    let pattern_bytes = pattern.as_bytes();
    let text_len = text_bytes.len();
    let pattern_len = pattern_bytes.len();
    let mut result:Vec<isize> = Vec::with_capacity(0);

    // println!("{:?}", text.as_bytes());
    // println!("{:?}", pattern.as_bytes());
    for _i in 0..text_len-pattern_len+1 {
        let mut check_count = 0;
        for _j in 0..pattern_len {
            // text.as_bytes()[_i+_j].as_char();
            // println!("{}\t{}", text.as_bytes()[_i+_j], pattern.as_bytes()[_j]);

            if text.as_bytes()[_i+_j] == pattern.as_bytes()[_j] {
                check_count += 1;
            }else {
                break;
            }
            
        }
        // println!("{} {}", check_count, pattern_len);
        if check_count == pattern_len {
            result.push(_i as isize);
        }
    }
    if result.len() > 0 {
        result
    }else{
        vec![-1]
    }
}
pub fn handle_search_pattern() {
    let mut text: String = read!("{}\n");
    let mut pattern: String = read!("{}\n");
    println!("{:?}", search_pattern(text, pattern));
}