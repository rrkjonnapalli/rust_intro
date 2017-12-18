pub fn calc(m:f64, n:f64) -> Vec<f64> {
    let mut result = Vec::new();
    result.push(m+n);
    result.push(m-n);
    result.push(m*n);
    result.push(m/n);
    result
}