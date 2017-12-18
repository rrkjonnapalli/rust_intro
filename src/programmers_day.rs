use chrono::prelude::*;
// use chrono::offset::LocalResult;
use time::Duration;

fn is_julian_leap_year(year: i32) -> bool {
    if year%4 == 0{
        true
    }else{
        false
    }
}
fn is_gregorian_leap_year(year: i32) -> bool {
    if year%400 == 0 || (year%4 == 0 && year%100 !=0) {
        true
    }else{
        false
    }
}
pub fn handle_programmers_day() {
    let year = read!();
    let mut dt = Utc.ymd(year, 1, 1) + Duration::days(255);
    if year == 1918{
        dt = dt + Duration::days(13);
    }else if year < 1918 && is_julian_leap_year(year) && !is_gregorian_leap_year(year){
        dt = dt - Duration::days(1);
    }
    println!("{}.{}.{}", dt.day(), dt.month(), dt.year());
}