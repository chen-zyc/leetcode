struct Solution;
impl Solution {
    pub fn flood_fill(mut image: Vec<Vec<i32>>, sr: i32, sc: i32, new_color: i32) -> Vec<Vec<i32>> {
        let (sr, sc) = (sr as usize, sc as usize);
        let init_color = image[sr][sc];
        // 这个是我没有想到的，如果颜色相同，那么就不用标记，因为标记后 image 并没有改变。
        // 同时下面只需要判断是不是和 init_color 一样就能避免重复入队了，因为标记过后颜色和 init_color 一定不一样。
        if init_color == new_color {
            return image;
        }
        // 队列中的是需要标记成新颜色的，但还未标的.
        let mut stack = Vec::new();
        stack.push((sr, sc));
        let (m, n) = (image.len(), image[0].len()); // 长度在 [1, 50]
        while let Some((x, y)) = stack.pop() {
            image[x][y] = new_color;
            // 上
            if x > 0 && image[x - 1][y] == init_color {
                stack.push((x - 1, y));
            }
            // 右
            if y + 1 < n && image[x][y + 1] == init_color {
                stack.push((x, y + 1));
            }
            // 下
            if x + 1 < m && image[x + 1][y] == init_color {
                stack.push((x + 1, y));
            }
            // 左
            if y > 0 && image[x][y - 1] == init_color {
                stack.push((x, y - 1));
            }
        }
        image
    }

    pub fn flood_fill1(
        mut image: Vec<Vec<i32>>,
        sr: i32,
        sc: i32,
        new_color: i32,
    ) -> Vec<Vec<i32>> {
        let (m, n) = (image.len(), image[0].len()); // 长度在 [1, 50]
        let mut flag = vec![vec![false; n]; m];
        // 队列中的是需要标记成新颜色的，但还未标的.
        let mut stack = Vec::new();
        let (sr, sc) = (sr as usize, sc as usize);
        stack.push((sr, sc));
        let init_color = image[sr][sc];
        while let Some((x, y)) = stack.pop() {
            image[x][y] = new_color;
            flag[x][y] = true;
            // 上
            if x > 0 && image[x - 1][y] == init_color && !flag[x - 1][y] {
                stack.push((x - 1, y));
            }
            // 右
            if y + 1 < n && image[x][y + 1] == init_color && !flag[x][y + 1] {
                stack.push((x, y + 1));
            }
            // 下
            if x + 1 < m && image[x + 1][y] == init_color && !flag[x + 1][y] {
                stack.push((x + 1, y));
            }
            // 左
            if y > 0 && image[x][y - 1] == init_color && !flag[x][y - 1] {
                stack.push((x, y - 1));
            }
        }
        image
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_flood_fill1() {
        let image = vec![vec![1, 1, 1], vec![1, 1, 0], vec![1, 0, 1]];
        let new_image = Solution::flood_fill(image, 1, 1, 2);
        let want = vec![vec![2, 2, 2], vec![2, 2, 0], vec![2, 0, 1]];
        assert_eq!(new_image, want);
    }
}
