fn union_sorted_arrays(arr1: &[i32], arr2: &[i32]) -> Vec<i32> {
    let n = arr1.len();
    let m = arr2.len();
    let mut i = 0;
    let mut j = 0;
    let mut union = Vec::new();

    while i < n && j < m {
        if arr1[i] < arr2[j] {
            if union.last() != Some(&arr1[i]) {
                union.push(arr1[i]);
            }
            i += 1;
        } else if arr1[i] > arr2[j] {
            if union.last() != Some(&arr2[j]) {
                union.push(arr2[j]);
            }
            j += 1;
        } else {
            if union.last() != Some(&arr1[i]) {
                union.push(arr1[i]);
            }
            i += 1;
            j += 1;
        }
    }

    // Add remaining elements of arr1, if any
    while i < n {
        if union.last() != Some(&arr1[i]) {
            union.push(arr1[i]);
        }
        i += 1;
    }

    // Add remaining elements of arr2, if any
    while j < m {
        if union.last() != Some(&arr2[j]) {
            union.push(arr2[j]);
        }
        j += 1;
    }

    union
}

fn main() {
    let arr1 = [1, 2, 4, 5, 6];
    let arr2 = [2, 3, 5, 7];
    let union = union_sorted_arrays(&arr1, &arr2);
    println!("{:?}", union);
}
