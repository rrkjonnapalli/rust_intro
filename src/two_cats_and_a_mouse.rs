pub fn cats_and_mouse() {
    let n:usize = read!();

    for _i in 0..n {
        let a:isize = read!();
        let b:isize = read!();
        let c:isize = read!();

        let a_c:isize = a-c;
        let b_c:isize = b-c;

        if a_c.abs() < b_c.abs() {
            println!("Cat A");
        }else if a_c.abs() > b_c.abs() {
            println!("Cat B");
        }else {
            println!("Mouse C");
        }
    }
}