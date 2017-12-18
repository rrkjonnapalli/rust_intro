// #[macro_use] extern crate text_io;

pub fn calc_min_book_pages() {
    let n:usize = read!();
    let p:usize = read!();
    let p_avg = p/2;
    let result = if p_avg < n/2-p_avg { p_avg } else  { n/2-p_avg };
    println!("{}", result);
}