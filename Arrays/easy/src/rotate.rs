// std::fmt::Display;
fn main()
{
	let mut arr = [2,4,3,7,9,10];
		let n = arr.len();

	let k = 3;
	let mut max = vec![0; n];
	for i in 0..k {
		max[i]=arr[n-k+i];
	}
	for i in 0..(n-k){
		max[k+i] = arr[i];
	}
	for i in 0..n{
	    arr[i]=max[i];
	}
    let a = max;
    println!("{:#?}",a);
}
