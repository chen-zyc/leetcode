struct Solution;
impl Solution {
    pub fn video_stitching(clips: Vec<Vec<i32>>, t: i32) -> i32 {
        // maxn[i] = j 表示以 i 为左端点的子区间的最远位置是 j。
        let mut maxn = vec![0 as usize; t as usize];
        for clip in clips {
            let (l, r) = (clip[0] as usize, clip[1] as usize);
            if l < t as usize && r > maxn[l] {
                maxn[l] = r;
            }
        }

        let (mut prev, mut last, mut ans) = (0, 0, 0);
        for (i, v) in maxn.into_iter().enumerate() {
            if v > last {
                last = v;
            }
            if i == last {
                return -1;
            }
            if i == prev {
                ans += 1;
                prev = last;
            }
        }
        ans
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_video_stitching() {
        let clips = vec![
            vec![0, 2],
            vec![4, 6],
            vec![8, 10],
            vec![1, 9],
            vec![1, 5],
            vec![5, 9],
        ];
        assert_eq!(Solution::video_stitching(clips, 10), 3);
        let clips = vec![vec![0, 1], vec![1, 2]];
        assert_eq!(Solution::video_stitching(clips, 5), -1);
        let clips = vec![
            vec![0, 1],
            vec![6, 8],
            vec![0, 2],
            vec![5, 6],
            vec![0, 4],
            vec![0, 3],
            vec![6, 7],
            vec![1, 3],
            vec![4, 7],
            vec![1, 4],
            vec![2, 5],
            vec![2, 6],
            vec![3, 4],
            vec![4, 5],
            vec![5, 7],
            vec![6, 9],
        ];
        assert_eq!(Solution::video_stitching(clips, 9), 3);
        let clips = vec![vec![0, 4], vec![2, 8]];
        assert_eq!(Solution::video_stitching(clips, 5), 2);
    }
}
