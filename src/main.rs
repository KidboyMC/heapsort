fn heapify(arr: &mut [i32], n: usize, i: usize) {
    let mut smallest = i; // Change from largest to smallest
    let left = 2 * i + 1;
    let right = 2 * i + 2;

    if left < n && arr[left] < arr[smallest] {
        smallest = left;
    }
    if right < n && arr[right] < arr[smallest] {
        smallest = right;
    }
    if smallest != i {
        arr.swap(i, smallest);
        heapify(arr, n, smallest);
    }
}

fn heap_sort_desc(arr: &mut [i32]) {
    let n = arr.len();

    // Build Min Heap (instead of Max Heap)
    for i in (0..n / 2).rev() {
        heapify(arr, n, i);
    }

    // Extract elements one by one
    for i in (1..n).rev() {
        arr.swap(0, i);
        heapify(arr, i, 0);
    }
}

fn main() {
    let mut arr = [4, 10, 3, 5, 1];
    heap_sort_desc(&mut arr);
    println!("{:?}", arr); // Output: [10, 5, 4, 3, 1]
}