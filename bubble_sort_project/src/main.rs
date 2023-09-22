fn bubble_sort<T: PartialOrd>(arr: &mut [T]) {
    let len = arr.len();
    for i in 0..len {
        for j in 0..len-i-1 {
            if arr[j] > arr[j+1] {
                arr.swap(j, j+1);
            }
        }
    }
}

fn main() {
    let mut list = vec![64, 34, 25, 12, 22, 11, 90];
    bubble_sort(&mut list);
    println!("{:?}", list);

    let mut list = vec!["boy", "apple", "dog", "cat"];
    bubble_sort(&mut list);
    println!("{:?}", list);
}
