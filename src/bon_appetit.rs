pub fn handle_bon_appetit() {
    let n = read!();
    let k = read!();

    // let mut total_price = 0;
    let mut actual_price = 0;

    for _i in 0..n {
        let price:usize = read!();
        if _i == k{
            continue;
        }
        actual_price = actual_price + price;
    }
    let b:usize = read!();

    println!("{} {}", b, actual_price/2);
    if b == actual_price/2 {
        println!("Bon Appetit");
    }else{
        println!("{}", b-(actual_price/2));
    }
}