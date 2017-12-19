pub fn compute_lps_array(pat:String) -> Vec<usize> {
    let pat_len = pat.len();
    let mut len = 0;
    let mut lps = vec![0;pat_len];
    let mut _i = 1;
    
    while _i < pat_len {
        if pat.as_bytes()[_i] == pat.as_bytes()[len] {
            len += 1;
            lps[_i] = len;
            _i += 1;
        }else {
            if len != 0 {
                len = lps[len-1];
            }else{
                lps[_i] = 0;
                _i += 1;
            }
        }
    }
    lps
}
pub fn handle_lps_array() {
    let mut pat:String = read!();
    compute_lps_array(pat);
    // -> Vec
}