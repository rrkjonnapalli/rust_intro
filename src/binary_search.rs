fn recursive_binary_search(data:Vec<isize>, mut l:usize, mut r:usize, e:isize) -> isize{
    if l <= r {
        let mid = l+(r-l)/2;
        if data[mid] == e {
            mid as isize
        }else if data[mid] > e{
            r = mid - 1;
            recursive_binary_search(data, l, r, e)
        }else{
            l = mid + 1;
            recursive_binary_search(data, l, r, e)
        }
    }else{
        return -1
    }

}
fn iterative_binary_search (data:Vec<isize>, e:isize) -> isize {
    let mut l = 0;
    let mut r = data.len()-1;
    while l <= r && l >= 0 {
        let mid = l+(r-l)/2;
        if data[mid] == e {
            return mid as isize
        }else if data[mid] < e {
            l = mid+1;
            // println!("greater");
        }else{
            r = mid-1;
            // println!("lesser");
        }
    }
    -1
}

pub fn handle_binary_search() {
    println!("{}", iterative_binary_search(vec![-1, 1,2,3,4], 12));
    // let s = vec![-1, 1,2,3,4,5];
    // println!("{:?}", s.binary_search(&-1));
}