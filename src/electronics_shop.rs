pub fn calc_money() {
    let s:usize = read!();
    let n:usize = read!();
    let m:usize = read!();

    let mut keyboards:Vec<usize> = Vec::with_capacity(0);
    let mut mouses:Vec<usize> = Vec::with_capacity(0);

    for _i in 0..n {
        let k_price:usize = read!();
        if k_price < s {
            keyboards.push(k_price);
        }
    }
    for _i in 0..m {
        let m_price:usize = read!();
        if m_price < s {
            mouses.push(m_price);
        }
    }
    keyboards.sort();
    mouses.sort();
    // println!("{:?} {:?}", keyboards, mouses);
    let mut price = -1;
    for _i in keyboards {
        if _i < s {
            // println!("{:?}", mouses);
            for _j in &mouses {
                if *_j <= s-_i {
                    if price < (_i + *_j) as isize {
                        price = (_i + *_j) as isize;
                    }
                }else{
                    break;
                }
            }
        }else {
            break;
        }
    }

    println!("{}", price);
}