struct Solution;
impl Solution {
    pub fn can_complete_circuit(gas: Vec<i32>, cost: Vec<i32>) -> i32 {
        let n = gas.len();
        let mut start = 0; // 起始点
        while start < n {
            let (mut cnt, mut sum_gas, mut sum_cost) = (0, 0, 0);
            while cnt < n {
                let j = (start + cnt) % n;
                sum_gas += gas[j];
                sum_cost += cost[j];
                if sum_cost > sum_gas {
                    break;
                }
                cnt += 1;
            }
            if cnt == n {
                return start as i32;
            }
            start += cnt + 1;
        }

        -1
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_can_complete_circuit() {
        assert_eq!(
            Solution::can_complete_circuit(vec![1, 2, 3, 4, 5], vec![3, 4, 5, 1, 2]),
            3
        );
        assert_eq!(
            Solution::can_complete_circuit(vec![2, 3, 4], vec![3, 4, 3]),
            -1
        );
    }
}
