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

    pub fn find_number_in2_d_array(matrix: Vec<Vec<i32>>, target: i32) -> bool {

        for i in matrix{
            if i.contains(&target){
                print!("true");
                return true;
            }
        }

        false
    }

    pub fn find_number_in2_d_array2(matrix: Vec<Vec<i32>>, target: i32) -> bool {

        let rows = matrix.len();
        let columns = matrix[0].len();
        let mut row:usize = 1;
        let mut column:usize = columns;


        while row <= rows && column > 0 {
            
            if (*&matrix[row-1][column-1] == target){
                return true
            }else if *&matrix[row-1][column-1] < target{
                row+=1;
            }else if target < *&matrix[row-1][column-1]{
                column -= 1;
            }

        }
        false
    }

    pub fn replace_space(s: String) -> String {


        s.replace(' ', "%20")
    }

}

fn main() {
    
    //Solution::find_repeate_number2([1,1,2,3,4,2,2].to_vec());





    // let mut matrix:Vec<Vec<i32>> =[
    //     [1,   4,  7, 11, 15].to_vec(),
    //     [2,   5,  8, 12, 19].to_vec(),
    //     [3,   6,  9, 16, 22].to_vec(),
    //     [10, 13, 14, 17, 24].to_vec(),
    //     [18, 21, 23, 26, 30].to_vec()
    //   ].to_vec();
    // //Solution::find_number_in2_d_array(matrix, 20i32);
    // Solution::find_number_in2_d_array2(matrix, 20i32);

    Solution::replace_space("We are happy.".to_string());

}
