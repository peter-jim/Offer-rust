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

    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        //输入一个递增排序的数组和一个数字s，在数组中查找两个数，使得它们的和正好是s。如果有多对数字的和等于s，则输出任意一对即可。
        // for i in 0..nums.len(){
        //     for j in 0..nums.len(){
        //         if nums[i] + nums[j] == target{
        //             return [nums[i],nums[j] ].to_vec();
        //         }
        //     }
        // }
        // [].to_vec()
        // 此暴力方法效率低 

        //因此可使用 双指针法 将空间复杂度降低至 O(1)O(1) 
        let mut i = 0;
        let mut j = nums.len() - 1;

        let ret =  Vec::new();

        while i<j {
           let s = nums[i]+nums[j];
           if s < target{
               i+=1;
           } else if s>target {
               j -=1 ;
           }else {
               return [nums[i],nums[j]].to_vec();
           }  
        }
        ret
    }

    pub fn find_continuous_sequence(target: i32) -> Vec<Vec<i32>> {
        // 输入一个正整数 target ，输出所有和为 target 的连续正整数序列（至少含有两个数）。
        // 序列内的数字由小到大排列，不同序列按照首个数字从小到大排列


        Vec::new()
    }

    pub fn sum_nums(n: i32) -> i32 {
        //求 1+2+...+n ，要求不能使用乘除法、for、while、if、else、switch、case等关键字及条件判断语句（A?B:C）。
        let mut n = n;
        n > 0 && {
            n += Solution::sum_nums(n - 1);
            true
        };
        n
    }

    pub fn reverse_words(s: String) -> String {
//输入一个英文句子，翻转句子中单词的顺序，但单词内字符的顺序不变。为简单起见，标点符号和普通字母一样处理。例如输入字符串"I am a student. "，则输出"student. a am I"。
        
        ' '.to_string()
    }


    pub fn single_numbers(nums: Vec<i32>) -> Vec<i32> {


        Vec::new()
    }

    pub fn search(nums: Vec<i32>, target: i32) -> i32 {
        //统计一个数字在排序数组中出现的次数。
        let mut count = 0;
        let mut lag = 0;
        
        if nums.len() != 1{
                for i in 0..nums.len(){
                    if nums[i] == target {
                        count += 1;
                        lag += 1;
                    }else if lag > 0 &&  nums[i] != target{
                        return count
                    }
                }
                
                return count
        }else{
            if nums[0] == target{
                return 1
            }else{
                return 0
            }
        }
    }

    pub fn search2(nums: Vec<i32>, target: i32) -> i32 {
        //使用二分查找
        
        
        0
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

    //Solution::replace_space("We are happy.".to_string());

    //Solution::two_sum([2,7,11,15].to_vec(), 9);

    //Solution::find_continuous_sequence(15);


    Solution::sum_nums(100);
}
