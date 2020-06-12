pub fn three_sum(nums: Vec<i32>) -> Vec<Vec<i32>> {
    let mut res = Vec::new();

    // 排序
    let mut nums = nums;
    nums.sort();
    let n = nums.len();

    for a in 0..n {
        // 过滤掉重复的
        if a != 0 && nums[a] == nums[a - 1] {
            continue;
        }

        let mut c = n - 1;
        let target = -nums[a];

        for b in a + 1..n {
            if b != a + 1 && nums[b] == nums[b - 1] {
                continue;
            }

            while b < c && nums[b] + nums[c] > target {
                c -= 1;
            }

            if b == c {
                break;
            }

            if nums[b] + nums[c] == target {
                res.push(vec![nums[a], nums[b], nums[c]]);
            }
        }
    }

    res
}
