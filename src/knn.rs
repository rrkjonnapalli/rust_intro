use quickersort;
use std::cmp::Ordering;

#[derive(Copy,Clone)]
struct Point {
    val:isize,
    x:f64,
    y:f64,
    distance: f64,
}

fn comparision(p:Point, q:Point) -> bool{
    p.distance < q.distance
}

fn knn_classify(mut train_set:Vec<Point>, k:usize, mut test_point:Point) -> usize{
    let n = train_set.len();
    for _i in 0..n {
        train_set[_i].distance = ((train_set[_i].x - test_point.x).powi(2) + (train_set[_i].y - test_point.y).powi(2)).sqrt()
    }
    
    quickersort::sort_by(&mut train_set, &|p, q| if p.distance < q.distance {Ordering::Less} else {Ordering::Greater});
    
    let mut freq1:isize = 0;
    let mut freq2:isize = 0;

    for _i in 0..k {
        if train_set[_i].val == 0 {
            freq1 += 1;
        } else if train_set[_i].val == 1 {
            freq2 += 1
        }
    }
    if freq1 > freq2 { 0 } else { 1 }
}

pub fn handle_knn() {
    let mut n:usize = read!();
    // let mut arr:Vec<Point> = Vec::with_capacity(17);
    let mut arr = vec![Point {x:0.0, y:0.0, distance:0.0, val:0};n];
    let mut k:usize = read!();
    let mut test_point = Point {x:2.5, y:7.0, distance:0.0, val:0};

    // test_point.x = 2.5;
    // test_point.y = 7.0;

    arr[0].x = 1.0;
    arr[0].y = 12.0;
    arr[0].val = 0;

    arr[1].x = 2.0;
    arr[1].y = 5.0;
    arr[1].val = 0;

    arr[2].x = 5.0;
    arr[2].y = 3.0;
    arr[2].val = 1;

    arr[3].x = 3.0;
    arr[3].y = 2.0;
    arr[3].val = 1;

    arr[4].x = 3.0;
    arr[4].y = 6.0;
    arr[4].val = 0;

    arr[5].x = 1.5;
    arr[5].y = 9.0;
    arr[5].val = 1;

    arr[6].x = 7.0;
    arr[6].y = 2.0;
    arr[6].val = 1;

    arr[7].x = 6.0;
    arr[7].y = 1.0;
    arr[7].val = 1;


    arr[8].x = 3.8;
    arr[8].y = 3.0;
    arr[8].val = 1;

    arr[9].x = 3.0;
    arr[9].y = 10.0;
    arr[9].val = 0;


    arr[10].x = 5.6;
    arr[10].y = 4.0;
    arr[10].val = 1;


    arr[11].x = 4.0;
    arr[11].y = 2.0;
    arr[11].val = 1;


    arr[12].x = 3.5;
    arr[12].y = 8.0;
    arr[12].val = 0;


    arr[13].x = 2.0;
    arr[13].y = 11.0;
    arr[13].val = 0;


    arr[14].x = 2.0;
    arr[14].y = 5.0;
    arr[14].val = 1;


    arr[15].x = 2.0;
    arr[15].y = 9.0;
    arr[15].val = 0;


    arr[16].x = 1.0;
    arr[16].y = 7.0;
    arr[16].val = 0;

    println!("{}", knn_classify(arr, k, test_point));
}   