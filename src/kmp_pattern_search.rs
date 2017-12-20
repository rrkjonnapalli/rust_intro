use lps_array;
pub fn kmp_pattern_search(text: String, pattern: String){
    let lps = lps_array::compute_lps_array(pattern.clone());
    let text_len = text.len();
    let pattern_len = pattern.len();

    let mut _i = 0;
    let mut _j = 0;

    while _i < text_len {
        if pattern.as_bytes()[_j] == text.as_bytes()[_i] {
            _i+=1;
            _j+=1;
        }

        if _j == pattern_len {
            println!("pattern found at {}", _i-_j);
            _j = lps[_j-1];
        }else if _i < text_len && pattern.as_bytes()[_j] != text.as_bytes()[_i] {
            if _j != 0 {
                _j = lps[_j-1];
            }else{
                _i += 1;
            }
        }
    }
}

pub fn handle_kmp_search() {
    let mut text:String = read!("{}\n");
    let mut pattern: String = read!("{}\n");

    kmp_pattern_search(text, pattern);
}