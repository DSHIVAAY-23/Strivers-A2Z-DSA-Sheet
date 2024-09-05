use std::cmp::max;

fn main() {
    let arr = [1, 1, 2, 4, 3, 1, 1, 2, 7, 9, 10];
    let n = arr.len();
    let mut left = 0;
    let mut right = 0;
    let mut count = 0;
    let mut maxlen = 0 as i32;
    let k = 5;
    let mut sum = 0;

    while right < n {
        sum += arr[right];

        while left <= right && sum > k {
            sum -= arr[left];
            left += 1;
        }

        if sum == k {
            count += 1;
            maxlen = max(maxlen, (right - left + 1) as i32);
        }

        right += 1;
    }

    println!("Maximum length of subarray with sum {}: {}", k, maxlen);
    println!("Number of subarrays with sum {}: {}", k, count);
}
