impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
       let mut i: usize = 0;

       //incriment througout the entire vector 
       while i < nums.len() {
        let mut j: usize = i + 1;
         
          //incriment througout the vector past the current integer  
        while j < nums.len() {
            let two_sum = nums[i] + nums[j];
            if two_sum == target {
                return vec![i as i32,j as i32];
            }
             j += 1;
        }

        i += 1;
       }
       return vec![];
    }
}
