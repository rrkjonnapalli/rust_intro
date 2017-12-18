pub fn handle_candles(){
    let n:usize = read!();
    let mut candles:Vec<usize> = Vec::with_capacity(n);
    for _i in 0..n {
        candles.push(read!())
    }

    let mut max = 0;
    let mut count = 0;
    for height in candles{
        if max == height{
            count = count + 1;
        }else if max < height {
            max = height;
            count = 1;
        }
    }
    println!("{}", count);
}