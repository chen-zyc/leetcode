struct Solution;
impl Solution {
    pub fn count_smaller(nums: Vec<i32>) -> Vec<i32> {
        let mut result = Vec::new();

        let a = Self::discretization(&nums);
        // c 是树状数组
        let mut c = vec![0; a.len() + 1];
        for num in nums.iter().rev() {
            let id = Self::get_id(&a, num);
            result.push(Self::query(&c, id - 1));
            Self::update(&mut c, id);
        }

        result.reverse();
        result
    }

    // 将 nums 去重并排序
    fn discretization(nums: &Vec<i32>) -> Vec<i32> {
        // 去重
        let set: std::collections::HashSet<&i32> = nums.iter().collect();
        let mut a: Vec<i32> = set.iter().map(|x| **x).collect();
        // 排序
        a.sort();
        a
    }

    fn get_id(a: &Vec<i32>, num: &i32) -> isize {
        a.binary_search(num).unwrap() as isize + 1
    }

    fn query(c: &Vec<i32>, mut pos: isize) -> i32 {
        let mut ret = 0;
        while pos > 0 {
            ret += c[pos as usize];
            pos -= Self::lowbit(pos);
        }
        ret
    }

    fn update(c: &mut Vec<i32>, mut pos: isize) {
        while pos < c.len() as isize {
            c[pos as usize] += 1;
            pos += Self::lowbit(pos);
        }
    }

    fn lowbit(n: isize) -> isize {
        n & (-n)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_count_smaller() {
        let res = Solution::count_smaller(vec![5, 2, 6, 1]);
        assert_eq!(res, vec![2, 1, 1, 0]);
    }
}
