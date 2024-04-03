// Bubble Sort implementation

pub fn sort(arr: &mut Vec<i32>) {
    let n = arr.len();

    for i in 0..n {
        // we don't need the last element because it gets compared with (n - 1) element
        for j in 0..(n - i - 1) {
            if arr[j] > arr[j + 1] {
                arr.swap(j, j + 1)
            }
        }
    }
}
