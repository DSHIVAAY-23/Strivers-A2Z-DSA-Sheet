pub fn subsets(nums: Vec<i32>) -> Vec<Vec<i32>> {
    let mut n = nums.len();
    let mut subsets_ct = 1<<n;
    let mut subsets: Vec<Vec<i32>> = Vec::new(); // Store all subsets

    for i in 0..subsets_ct{
        let mut subset =Vec::new();
        for j in 0..n{
            if (i & (1<<j)) !=0 {
                subset.push(nums[j]);
            }
            
        }
        subsets.push(subset);
    }
    subsets
}
fn main(){
    let nums = vec![1,2,3,4,5];
    let subsets = subsets(nums);
    for subset in subsets{
        println!("{:?}", subset);
    }
}