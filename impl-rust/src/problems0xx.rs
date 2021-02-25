struct Solution;
impl Solution {
    pub fn largest_rectangle_area(heights: Vec<i32>) -> i32 {
        // let (mut p1, mut p2) = (0, heights.len());
        // let mut largest = 0;
        // while p2 > p1 {
        //     // 这种计算是不对的，因为中间可能有凹点。
        //     let area = (p2 - p1) as i32 * heights[p1].min(heights[p2 - 1]);
        //     if area > largest {
        //         largest = area;
        //     }
        //     if heights[p2 - 1] < heights[p1] {
        //         p2 -= 1;
        //     } else {
        //         p1 += 1;
        //     }
        // }
        // largest

        if heights.is_empty() {
            return 0;
        }

        // ======= 方法一：在栈中保存下标和高度。
        // // 在后面加入哨兵。
        // let heights_iter = heights.into_iter().chain(0..1);
        //
        // let mut largest = 0;
        // // (下标，高度)
        // let mut stack = vec![];
        // // 前面的哨兵，下标一定是 -1，不然计算宽度时就会不对了。
        // // 这种保存了高度的做法效率可能不高。
        // stack.push((-1_i32, 0));
        //
        // for (i, height) in heights_iter.enumerate() {
        //     while height < stack.last().unwrap().1 {
        //         // 找到一个比当前下标的高度大的。
        //         // 计算它的面积。
        //         let prev = stack.pop().unwrap();
        //         // 宽度是 prev 前面那个和当前下标之间的宽度。
        //         let area = (i as i32 - stack.last().unwrap().0 - 1) * prev.1;
        //         largest = largest.max(area);
        //     }
        //     stack.push((i as i32, height));
        // }

        // ======= 方法二：下面的实现是只在后面加哨兵，没有在前面加，但是编译不过。
        // 在后面加入哨兵。
        // let heights: Vec<_> = heights.into_iter().chain(0..1).collect();
        // let mut largest = 0;
        // let mut stack = vec![];
        // // 前面的哨兵，下标一定是 -1，不然计算宽度时就会不对了。
        // stack.push(-1_isize);
        //
        // for (i, &height) in heights.iter().enumerate() {
        //     // ERROR: 如果 stack.last == -1，那么这里就会 panic.
        //     while height < heights[*stack.last().unwrap() as usize] {
        //         // 找到一个比当前下标的高度大的。
        //         // 计算它的面积。
        //         let prev = stack.pop().unwrap() as usize;
        //         // 宽度是 prev 前面那个和当前下标之间的宽度。
        //         let area = (i as i32 - *stack.last().unwrap() as i32 - 1) * heights[prev];
        //         largest = largest.max(area);
        //     }
        //     stack.push(i as isize);
        // }

        // 方法三：把链转换成 Vec。速度上并没有提升多少。内在占用比方法一提升了些。
        // 在前后加哨兵。
        let heights: Vec<_> = (0..1).chain(heights.into_iter().chain(0..1)).collect();
        let mut largest = 0;
        let mut stack = vec![];
        // 前面的哨兵。
        stack.push(0);

        for (i, &height) in heights.iter().enumerate() {
            while height < heights[*stack.last().unwrap()] {
                // 找到一个比当前下标的高度大的。
                // 计算它的面积。
                let prev = stack.pop().unwrap();
                // 宽度是 prev 前面那个和当前下标之间的宽度。
                let area = (i - *stack.last().unwrap() - 1) as i32 * heights[prev];
                largest = largest.max(area);
            }
            // 前面的哨兵会被添加两次，不过没有关系。
            stack.push(i);
        }

        largest
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_largest_rectangle_area() {
        assert_eq!(Solution::largest_rectangle_area(vec![2, 1, 5, 6, 2, 3]), 10);
        assert_eq!(Solution::largest_rectangle_area(vec![2, 4]), 4);
        assert_eq!(Solution::largest_rectangle_area(vec![1]), 1);
    }
}
