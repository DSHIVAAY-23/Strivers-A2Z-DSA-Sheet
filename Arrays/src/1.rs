fn main()
{
	let arr = [2,4,3,7,9,10];
	let mut max = arr[0];
	let mut smax = arr[0];
	let n = arr.len();
	for i in 0..n{
		if arr[i]>max {
			max = arr[i];
			}
			
	}
	
    println!("{}",max);
}
