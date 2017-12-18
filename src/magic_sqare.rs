pub fn magic_sqare() {
    let mut mat:Vec<Vec<isize>> = Vec::with_capacity(0);
    let mut conrers:Vec<isize> = vec![1,3,7,9];
    let mut middles:Vec<isize> = vec![2,4,6,8];
    let mut center:isize = 5;
    let mut g_corners:Vec<isize> = Vec::with_capacity(0);
    let mut g_middles:Vec<isize> = Vec::with_capacity(0);
    let mut g_center:isize = 0;
    let mut count = 0;
    for _i in 0..3 {
        mat.push(Vec::with_capacity(0));
        for _j in 0..3 {
            let elem:isize = read!();
            mat[_i].push(elem);
            if (_i == 0 && _j == 0) || (_i == 0 && _j == 2) || (_i == 2 && _j == 0) || (_i == 2 && _j == 2) {
                g_corners.push(elem);
            }else if (_i == 0 && _j == 1) || (_i == 1 && _j == 0) || (_i == 1 && _j == 2) || (_i == 2 && _j == 1) {
                g_middles.push(elem);
            }else{
                g_center = elem;
            }
        }
    }
    println!("{:?}", mat);
    println!("{:?} {:?} {}", conrers, middles, center);


    // let s = match conrers.iter().position(|&x| x == elem) {
    //     Some(_expr) => _expr as isize,
    //     None => -1
    // };
}