fn swap(mut x:isize, mut y:isize) {
    let temp = x;
    x = y;
    y = temp;
}
fn quick_sort(arr:&Vec<isize>, low:usize, high:usize) -> Vec<isize> {
    if low < high {
        let pi = partition(&arr, low, high);

        quick_sort(&arr, low, pi-1);
        quick_sort(&arr, pi+1, high);
    }
    arr.clone()
}

fn partition(arr:&Vec<isize>, low:usize, high:usize) -> usize {
    let pivot = arr[high];

    let mut _i = low-1;

    for _j in low..high-1 {
        if arr[_j] <= pivot {
            _i += 1;
            swap(arr[_i], arr[_j]);
        }
    }
    swap(arr[_i + 1], arr[high]);
    _i+1
}

pub fn handle_quick_sort() {
    let mut arr = vec![10, 7, 8, 9, 1, 5];
    quick_sort(;mut &arr, 0, 5);
}