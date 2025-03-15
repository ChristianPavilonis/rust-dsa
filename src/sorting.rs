
fn bubble_sort(values: &mut [u8]) {
    let mut i = 0;
    let mut value;
    let mut next_value;
    let mut len = values.len();

    loop {
        if i + 1 == len {
            i = 0;
            len -= 1;
        }
        if len == 1 {
            break;
        }

        value = values[i];
        next_value = values[i + 1];

        if value > next_value {
            values[i] = next_value;
            values[i + 1] = value;
        }

        i += 1;
    }
}


fn quick_sort(values: &mut [u8]) {
    qs(values, 0, values.len() - 1);

}

fn qs(values: &mut [u8], lo: usize, hi: usize) {
    if lo >= hi {
        return;
    }

    let pivot_idx = partition(values, lo, hi);

    qs(values, lo, pivot_idx - 1);
    qs(values, pivot_idx + 1, hi);
}

fn partition(values: &mut [u8], lo: usize, hi: usize) -> usize {
    let pivot = hi;

    let mut i = lo;
    let mut j = hi - 1;

    loop {
        while values[i] < values[pivot] {
            i += 1;
        }

        while j > 0 && values[j] > values[pivot] {
            j -= 1;
        }

        if j == 0 || i >= j {
            println!("i: {i}, j: {j}");
            break;
        } else if values[i] == values[j] {
            i += 1;
            j -= 1;
        } else {
            println!("swapping {}({}) with {}({})", values[i], i, values[j], j);
            values.swap(i, j);
        }
    }

    println!("swapping {}({}) with {}({})", values[i], i, values[pivot], pivot);
    values.swap(i, pivot);

    println!("partition at {i}");
    println!("{:?}", values);
    i
}

#[cfg(test)]
mod tests {
    use crate::sorting::quick_sort;

    use super::bubble_sort;

    #[test]
    fn test_bubble_sort() {
        let mut arr = [1,9, 2, 4, 10, 3, 8, 4, 2];

        bubble_sort(&mut arr);
        assert_eq!(arr, [1, 2, 2, 3, 4, 4, 8, 9, 10]);
    }

    #[test]
    fn test_quick_sort() {
        let mut arr = [1, 9, 2, 4, 10, 3, 8, 4, 2];

        quick_sort(&mut arr);
        assert_eq!(arr, [1, 2, 2, 3, 4, 4, 8, 9, 10]);
    }
}
