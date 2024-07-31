use num::integer::Roots;

// O(n)
fn linear_search(haystack: &[u8], needle: u8) -> bool {
    for item in haystack {
        if *item == needle {
            return true;
        }
    }
    false
}

fn binary_search(haystack: &[u8], needle: u8) -> bool {
    let mut low = 0;
    let mut high = haystack.len();

    while low < high {
        let mid = low + (high - low) / 2;
        let value = haystack[mid];

        if value == needle {
            return true;
        }

        if value > needle {
            high = mid;
        } else if value < needle {
            low = mid + 1;
        }
    }

    false
}

fn two_crystal_balls(breaks: &[bool]) -> Option<usize> {
    let jump_amount = breaks.len().sqrt();
    let mut i = jump_amount;

    while i < breaks.len() {
        if breaks[i] {
            break;
        }

        i += jump_amount;
    }

    let mut j = 0;
    i -= jump_amount;


    while j <= jump_amount && i < breaks.len() {
        if breaks[i] {
            return Some(i);
        }

        j += 1;
        i += 1;
    }

    None
}

#[cfg(test)]
mod tests {
    use crate::search::{linear_search, two_crystal_balls};

    use super::binary_search;

    #[test]
    fn test_linear_search() {
        assert!(linear_search(&[1, 2, 3, 4], 3));
        assert!(!linear_search(&[1, 2, 3, 4], 6));
    }

    #[test]
    fn test_binary_search() {
        let arr = &[
            1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1,
            1, 1, 1, 7, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8,
            8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8,
            8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8,
        ];

        assert!(binary_search(arr, 7));
        assert!(!binary_search(arr, 9));
    }

    #[test]
    fn test_two_crystal_ball() {
        assert_eq!(two_crystal_balls(&[false, false, false, true, true]), Some(3));
        assert_eq!(two_crystal_balls(&[false, false, false, false, false, false, true, true]), Some(6));
        assert_eq!(two_crystal_balls(&[false, false, false, false, false]), None);
    }
}
