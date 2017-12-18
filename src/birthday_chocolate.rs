pub fn handle_birthday_chocolate(){
    let n:usize = read!();
    let mut ch_bar:Vec<usize> = Vec::with_capacity(n);
    for _i in 0..n {
        ch_bar.push(read!());
    }
    let d:usize = read!();
    let m:usize = read!();

    let mut count = 0;
    for _i in 0..n-m+1 {
        let mut c = 0;
        for _j in _i.._i+m {
            c = c+ch_bar[_j];
        }
        if c == d{
            count = count + 1;
        }
    }
    println!("{}", count);
}