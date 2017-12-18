pub fn handle_scores(){
    let n = read!();
    let mut init:usize = 0;
    let mut max = 0;
    let mut min = usize::max_value();
    let mut best_scores : Vec<usize> = Vec::with_capacity(0);
    let mut worst_scores : Vec<usize> = Vec::with_capacity(0);
    for _i in 0..n {
        if _i == 0{
            init = read!();
            continue;
        }
        let s: usize = read!();

        if s > init {
            if max < s{
                max = s;
                /**
                 * find position
                 * let index = test.iter().position(|&r| r == "two").unwrap();
                 */
                //check existance
                let found = match best_scores.iter().find(|&&x| x == s) {
                    Some(_expr) => true,
                    None => false,
                };
                if !found {
                    best_scores.push(s);
                }
            }
        }else if s < init {
            if min > s{
                min = s;
                let found = match worst_scores.iter().find(|&&x| x == s) {
                    Some(_expr) => true,
                    None => false,
                };
                if !found {
                    worst_scores.push(s);
                }
            }
        }
    }
    println!("{:?}", best_scores);
    println!("{:?}", worst_scores);
}