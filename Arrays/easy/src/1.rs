// fn main()
// {
// 	let arr = [2,4,3,7,9,10];
// 	let mut max = arr[0];
// 	let mut smax = arr[0];
// 	let n = arr.len();
// 	for i in 0..n{
// 		if arr[i]>max {
// 			max = arr[i];
// 			}
			
// 	}
	
//     println!("{}",max);
// }


fn main(){
	let mut arr = [2,3,4,5,6,68,9];
	let n = arr.len();
	let mut max = 0;
	let mut smax = 0;

	for i in 0..n{
		if arr[i]>max{
         max = arr[i];
		}

		
	}

	for i in 0..n{
		if (arr[i]>smax && arr[i]<max){
         smax = arr[i];
		}
	}
	println!("{}",smax);
}