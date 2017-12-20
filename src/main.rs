#[macro_use] extern crate text_io;
extern crate chrono;
extern crate time;

mod min_book_page;
mod counting_valleys;
mod electronics_shop;
mod two_cats_and_a_mouse;
mod string_remove_index;
mod flames;
mod programmers_day;
mod magic_sqare;
mod lps_array;
mod kmp_pattern_search;
mod naive_search_pattern;

fn main() {
    // magic_sqare::magic_sqare();
    // lps_array::handle_lps_array();
    kmp_pattern_search::handle_kmp_search();
    naive_search_pattern::handle_search_pattern();
}