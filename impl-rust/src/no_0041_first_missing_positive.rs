struct Solution;
impl Solution {
    pub fn first_missing_positive(mut nums: Vec<i32>) -> i32 {
        // 把 [1, N] 的数放到对应的位置上
        let n = nums.len();
        for i in 0..n {
            let mut num = nums[i];
            while num > 0 && num <= n as i32 {
                let temp = nums[num as usize - 1];
                if temp == num {
                    break; // 防止死循环
                }
                nums[num as usize - 1] = num;
                num = temp; // 交换完之后，可能当前位置也是个 [0, n] 内的数，继续交换
            }
        }
        // 没有出现在其位置上的数是缺失的数
        for (i, num) in nums.iter().enumerate() {
            if *num != i as i32 + 1 {
                return i as i32 + 1;
            }
        }
        // 都出现在其位置上了，那么 n+1 就是缺失的
        n as i32 + 1
    }

    pub fn first_missing_positive2(mut nums: Vec<i32>) -> i32 {
        // 1. 先把负数和0标记成 n+1。
        let n = nums.len() as i32;
        for num in nums.iter_mut() {
            if *num <= 0 {
                *num = n + 1;
            }
        }

        // 2. 把 >0 的数对应的位置标记成负数
        for i in 0..n {
            let num = nums[i as usize].abs(); // 可能已经被标记成负数了
            if num <= n {
                let v = nums[num as usize - 1];
                nums[num as usize - 1] = -v.abs();
            }
        }

        // 3. 第一个正数的位置就是缺失的数
        for (i, num) in nums.iter().enumerate() {
            if *num > 0 {
                return i as i32 + 1;
            }
        }
        // 4. 没有的话，那就是 n+1
        n + 1
    }

    pub fn first_missing_positive1(nums: Vec<i32>) -> i32 {
        let mut bits = Vec::<u128>::new();
        bits.push(1); // 第 0 个 bit 置 1，因为没有用到
        for n in nums {
            if n <= 0 {
                continue;
            }
            let bucket = n as usize / 128;
            if bucket >= bits.len() {
                bits.resize(bucket + 1, 0);
            }
            let bit = bits.get_mut(bucket).unwrap();
            *bit = (*bit) | 1 << (n as u128 & 127);
            println!("n = {}, bucket = {}, bit = {:b}", n, bucket, *bit);
        }
        for (i, bit) in bits.iter().enumerate() {
            println!("i = {}, bit = {:b}", i, bit);
            for j in 0..128 {
                println!("{:b} & (1<<{}) = {}", bit, j, bit & (1 << j));
                if bit & (1 << j) == 0 {
                    return (i * 128 + j) as i32;
                }
            }
        }
        // 没有露掉的，那就返回最后一个
        (bits.len() * 128) as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_first_missing_positive() {
        assert_eq!(Solution::first_missing_positive(vec![1, 2, 0]), 3);
        assert_eq!(Solution::first_missing_positive(vec![3, 4, -1, 1]), 2);
        assert_eq!(Solution::first_missing_positive(vec![7, 8, 9, 11, 12]), 1);
        assert_eq!(
            Solution::first_missing_positive(vec![
                0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22,
                23, 24, 25, 26, 27, 28, 29, 30, 31, 32, 33, 34, 35, 36, 37, 38, 39, 40, 41, 42, 43,
                44, 45, 46, 47, 48, 49, 50, 51, 52, 53, 54, 55, 56, 57, 58, 59, 60, 61, 62, 63, 64,
                65, 66, 67, 68, 69, 70, 71, 72, 73, 74, 75, 76, 77, 78, 79, 80, 81, 82, 83, 84, 85,
                86, 87, 88, 89, 90, 91, 92, 93, 94, 95, 96, 97, 98, 99, 100, 101, 102, 103, 104,
                105, 106, 107, 108, 109, 110, 111, 112, 113, 114, 115, 116, 117, 118, 119, 120,
                121, 122, 123, 124, 125, 126, 127, 128, 129, 130, 131, 132, 133, 134, 135, 136,
                137, 138, 139, 140, 141, 142, 143, 144, 145, 146, 147, 148, 149, 150, 151, 152,
                153, 154, 155, 156, 157, 158, 159, 160, 161, 162, 163, 164, 165, 166, 167, 168,
                169, 170, 171, 172, 173, 174, 175, 176, 177, 178, 179, 180, 181, 182, 183, 184,
                185, 186, 187, 188, 189, 190, 191, 192, 193, 194, 195, 196, 197, 198, 199, 200,
                201, 202, 203, 204, 205, 206, 207, 208, 209, 210, 211, 212, 213, 214, 215, 216,
                217, 218, 219, 220, 221, 222, 223, 224, 225, 226, 227, 228, 229, 230, 231, 232,
                233, 234, 235, 236, 237, 238, 239, 240, 241, 242, 243, 244, 245, 246, 247, 248,
                249, 250, 251, 252, 253, 254, 255, 256, 257, 258
            ]),
            259
        );
    }
}
