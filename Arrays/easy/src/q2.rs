fn find_second_largest(arr: &[i32]) -> Option<i32> {
  if arr.len() < 2 {
      return None; // Return None if there are less than 2 elements
  }

  let (mut first, mut second) = if arr[0] > arr[1] {
      (arr[0], arr[1])
  } else {
      (arr[1], arr[0])
  };

  for &item in &arr[2..] {
      if item > first {
          second = first;
          first = item;
      } else if item > second && item != first {
          second = item;
      }
  }

  Some(second)
}

fn main() {
  let array = [3, 5, 7, 2, 8, -1, 4, 10, 12];
  match find_second_largest(&array) {
      Some(second_max) => println!("The second largest element in the array is: {}", second_max),
      None => println!("The array does not have enough elements."),
  }
}
