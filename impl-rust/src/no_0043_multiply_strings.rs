struct Solution;
impl Solution {
    pub fn multiply(mut num1: String, mut num2: String) -> String {
        if num1.len() == 0 {
            return num2;
        }
        if num2.len() == 0 {
            return num1;
        }
        // 处理乘以 0 的情况
        if (num1.len() == 1 && num1 == "0") || (num2.len() == 1 && num2 == "0") {
            return "0".to_owned();
        }

        // 倒序，并且转换成数字
        let num1 = unsafe {
            let num = num1.as_mut_vec();
            for n in num.iter_mut() {
                *n = *n - '0' as u8;
            }
            num
        };
        let num2 = unsafe {
            let num = num2.as_mut_vec();
            for n in num.iter_mut() {
                *n = *n - '0' as u8;
            }
            num
        };
        // 每一轮的乘积之和，为了方便扩展，倒序存放
        let mut ans = Vec::new();
        for (i, y) in num2.into_iter().rev().enumerate() {
            // 用 num1 乘以 num2[i] 得到一个乘积，累加到 ans 中。
            Self::multiply_one_num(&num1, *y, &mut ans, i);
        }

        // 倒序后转换成字符串
        ans.iter().rev().map(|n| (*n + '0' as u8) as char).collect()
    }

    fn multiply_one_num(nums: &[u8], n: u8, ans: &mut Vec<u8>, off: usize) {
        // 这里就保证 ans 中的每一位都是小于 10 的，否则可能会超过 u8 的范围。
        let mut carry = 0;
        for (i, num) in nums.iter().rev().enumerate() {
            // idx 是应该存放在 ans 的哪个索引位置，如果 ans 长度不够，扩展到够为止。
            let idx = i + off;
            if ans.len() < idx + 1 {
                ans.resize(idx + 1, 0);
            }
            let v = ans.get_mut(idx).unwrap();
            *v += num * n + carry;
            if *v > 9 {
                carry = *v / 10;
                *v %= 10;
            } else {
                carry = 0;
            }
        }
        while carry > 0 {
            ans.push(carry % 10);
            carry /= 10;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_multiply1() {
        assert_eq!(
            Solution::multiply("2".to_owned(), "3".to_owned()),
            "6".to_owned()
        );
    }

    #[test]
    fn test_multiply2() {
        assert_eq!(
            Solution::multiply("123".to_owned(), "456".to_owned()),
            "56088".to_owned()
        );
    }

    #[test]
    fn test_multiply3() {
        assert_eq!(
            Solution::multiply("999".to_owned(), "999".to_owned()),
            "998001".to_owned()
        );
    }

    #[test]
    fn test_multiply4() {
        assert_eq!(
            Solution::multiply("9133".to_owned(), "0".to_owned()),
            "0".to_owned()
        );
    }
}
