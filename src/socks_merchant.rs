use std::collections::HashMap;
pub fn handle_sock_merchant() {
    let n:usize = read!();
    let mut socks:HashMap<usize, usize> = HashMap::new();
    let mut result:usize = 0;
    for _i in 0..n{
        let key:usize = read!();
        let val = match socks.get(&key,) {
            Some(expr) => expr+1,
            None => 1,
        };
        socks.insert(key, val);
    }

    for key in socks.keys(){
        let val = match socks.get(&key,) {
            Some(expr) => *expr,
            None => 0,
        };
        result = result + val/2;
    }
    println!("{}", result);
}