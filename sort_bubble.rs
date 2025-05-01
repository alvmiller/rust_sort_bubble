// rustc sort_bubble.rs
// cargo run
// cargo test

pub fn sort_bubble<T>(arr: &mut [T])
where
    T: Ord,
{
    if arr.is_empty() {
        return;
    }

    let len = arr.len();
    let mut is_sorted;
    for i in 0..len {
        is_sorted = true;
        for j in 0..(len - 1 - i) {
            if arr[j] > arr[j + 1] {
                arr.swap(j, j + 1);
                is_sorted = false;
            }
        }
        if is_sorted {
            break;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sort_bubble() {
        let mut v = vec![3, 5, 1, 3, 2];
        sort_bubble(&mut v);
        assert!(v.is_sorted());
    }

    #[test]
    fn test_sort_bubble_presorted() {
        let mut v = vec![1, 2, 3, 4, 5, 6];
        sort_bubble(&mut v);
        assert!(v.is_sorted());
    }

    #[test]
    fn test_sort_bubble_empty() {
        let mut v: Vec<usize> = vec![];
        sort_bubble(&mut v);
        assert!(v.is_sorted());
    }
}

fn main() {
    let mut v = vec![4, 3, 3, 2, 1, 8, 10];
    println!("Array before sort: {:?}", v);
    sort_bubble(&mut v);
    println!("Array after sort: {:?}", v);
}
