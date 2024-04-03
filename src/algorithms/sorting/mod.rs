mod bubble;

pub fn sort() {
    let get_array = || vec![2, 1, 7, 3, 6, 4, 5];
    let mut arr = get_array();
    bubble::sort(&mut arr);
    println!("Sorted array: {:?}", arr);
}
