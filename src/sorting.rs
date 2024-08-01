
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

#[cfg(test)]
mod tests {
    use super::bubble_sort;

    #[test]
    fn test_bubble_sort() {
        let mut arr = [1,9, 2, 4, 10, 3, 8, 4, 2];

        bubble_sort(&mut arr);
        assert_eq!(arr, [1, 2, 2, 3, 4, 4, 8, 9, 10]);
    }
}
