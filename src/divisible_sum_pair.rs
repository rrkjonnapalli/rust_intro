pub fn handle_divisible_sum_pairs(){
    let n:usize = read!();
    let k:usize = read!();
    let mut arr:Vec<usize> = Vec::with_capacity(n);
    let mut result:usize = 0;
    for _i in 0..n{
        arr.push(read!());
    }

    for _i in 0..n-1 {
        for _j in _i+1..n{
            if (arr[_i] + arr[_j])%k == 0 {
                result = result+1;
                // println!("{} {}", arr[_i] + arr[_j], result);
            }
        }
    }

    println!("{}", result);
}