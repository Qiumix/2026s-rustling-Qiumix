/*
    sort
    This problem requires you to implement a sorting algorithm
    you can use bubble sorting, insertion sorting, heap sorting, etc.
*/
fn sort<T: PartialOrd + Clone>(slice: &mut [T]) {
    let len = slice.len();
    if len <= 1 {
        return;
    }

    let mid = len / 2;

    // Split and recurse
    sort(&mut slice[..mid]);
    sort(&mut slice[mid..]);

    // Merge logic
    let mut temp = Vec::with_capacity(len);
    let (mut left, mut right) = (0, mid);

    while left < mid && right < len {
        if slice[left] <= slice[right] {
            temp.push(slice[left].clone());
            left += 1;
        } else {
            temp.push(slice[right].clone());
            right += 1;
        }
    }

    // Push remaining elements
    if left < mid {
        temp.extend_from_slice(&slice[left..mid]);
    }
    if right < len {
        temp.extend_from_slice(&slice[right..len]);
    }

    // Copy back to original slice
    slice.clone_from_slice(&temp);
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sort_1() {
        let mut vec = vec![37, 73, 57, 75, 91, 19, 46, 64];
        sort(&mut vec);
        assert_eq!(vec, vec![19, 37, 46, 57, 64, 73, 75, 91]);
    }
    #[test]
    fn test_sort_2() {
        let mut vec = vec![1];
        sort(&mut vec);
        assert_eq!(vec, vec![1]);
    }
    #[test]
    fn test_sort_3() {
        let mut vec = vec![99, 88, 77, 66, 55, 44, 33, 22, 11];
        sort(&mut vec);
        assert_eq!(vec, vec![11, 22, 33, 44, 55, 66, 77, 88, 99]);
    }
}
