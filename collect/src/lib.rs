pub fn bubble_sort(arr: &mut [i32]) {
    let mut end_offset = 0;

    loop {
        let mut swaps = false;

        for i in 1..arr.len() - end_offset {
            if arr[i - 1] > arr[i] {
                arr.swap(i - 1, i);
                swaps = true;
            }
        }

        if !swaps {
            break;
        }
        end_offset += 1;
    }
}
