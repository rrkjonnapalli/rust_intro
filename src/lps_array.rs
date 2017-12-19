fn compute_lps_array(pat:String){
    let mut len = 0;
    let mut pat_len = pat.len();
    let mut lps = vec![0;pat_len];
    println!("{:?}", lps);
    for _i in 1..pat_len {
        if pat.as_bytes()[_i] == pat.as_bytes()[len] {
            len += 1;
            lps[_i] = len;
            _i += 1;
        }
    }
}
pub fn handle_lps_array() {
    let mut pat:String = read!();
    compute_lps_array(pat);
    // -> Vec
}