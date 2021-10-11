use core::num;
use std::collections::HashSet;
struct Solution{

}

impl Solution {
    

    //
    pub fn find_repeate_number(nums: Vec<i32>) -> i32 {
        /*
        由于对语法的不熟悉,用for很不友好
        */
        let mut n =0;
        while n<nums.len() {
            let mut j = n + 1 ;
            while j < nums.len() {
               if nums[n] == nums[j]{
                //println!("{}",nums[n])
               return  nums[n];
              } 
                j=j+1;
            }
            n=n+1;
        }
        0
    }

    pub fn find_repeate_number2(nums: Vec<i32>) -> i32 {
        let mut store: HashSet<i32> = HashSet::new();
        let mut res = -1;
        
                for i in 0..nums.len() {
                    if store.contains(&nums[i]) {
                        res = nums[i];
                        return res;
                        
                    }
                    else {
                        store.insert(nums[i]);
                    }
                }
        
                for i in store.into_iter(){
                    println!("{}" , &i)
                }
        
        res
    }
}

fn main() {
    
    Solution::find_repeate_number2([1,1,2,3,4,2,2].to_vec());
}
