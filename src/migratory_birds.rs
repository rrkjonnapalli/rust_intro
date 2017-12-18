pub fn handle_migratory_birds() {
    let n:usize = read!();
    let mut b_type_counts = vec![0; n];
    // let mut b_type_counts:Vec<usize> = Vec::with_capacity(5);
    let mut max = 0;
    let mut index = 0;
    for _i in 0..n {
        let mut a:usize = read!();
        a = a-1;
        b_type_counts[a] = b_type_counts[a] + 1;
        if max < b_type_counts[a]{
            max = b_type_counts[a];
            index = a;
        }else if max == b_type_counts[a] && index > a{
            index = a;
        }
    }
    println!("{}", index+1);
}