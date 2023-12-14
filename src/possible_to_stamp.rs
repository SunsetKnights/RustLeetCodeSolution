pub struct Solution;
/**
 * 2132. 用邮票贴满网格图
 * 给你一个 m x n 的二进制矩阵 grid ，每个格子要么为 0 （空）要么为 1 （被占据）。
 * 给你邮票的尺寸为 stampHeight x stampWidth 。我们想将邮票贴进二进制矩阵中，且满足以下 限制 和 要求 ：
 *
 *     覆盖所有 空 格子。
 *     不覆盖任何 被占据 的格子。
 *     我们可以放入任意数目的邮票。
 *     邮票可以相互有 重叠 部分。
 *     邮票不允许 旋转 。
 *     邮票必须完全在矩阵 内 。
 *
 * 如果在满足上述要求的前提下，可以放入邮票，请返回 true ，否则返回 false 。
 */
impl Solution {
    pub fn possible_to_stamp(grid: Vec<Vec<i32>>, stamp_height: i32, stamp_width: i32) -> bool {
        let height = grid.len();
        let width = grid[0].len();
        let stamp_height = stamp_height as usize;
        let stamp_width = stamp_width as usize;
        // 方便判断区域是否有被占据的格子，若有，区域和为1
        let mut prefix_sum = vec![vec![0; width + 2]; height + 2];
        // 方便给区域内的所有格子增加数字
        let mut diff = vec![vec![0; width + 2]; height + 2];
        for i in 1..=height {
            for j in 1..=width {
                prefix_sum[i][j] = grid[i - 1][j - 1] + prefix_sum[i - 1][j] + prefix_sum[i][j - 1]
                    - prefix_sum[i - 1][j - 1];
            }
        }

        if height >= stamp_height && width >= stamp_width {
            for i in 1..=height - stamp_height + 1 {
                for j in 1..=width - stamp_width + 1 {
                    let x = i + stamp_height - 1;
                    let y = j + stamp_width - 1;
                    // 以i,j为起始格子的邮票大小的格子区域为空
                    if prefix_sum[x][y] - prefix_sum[i - 1][y] - prefix_sum[x][j - 1]
                        + prefix_sum[i - 1][j - 1]
                        == 0
                    {
                        // 区域整体+1
                        diff[i][j] += 1;
                        diff[x + 1][y + 1] += 1;
                        diff[x + 1][j] -= 1;
                        diff[i][y + 1] -= 1;
                    }
                }
            }
        }
        for i in 1..=height {
            for j in 1..=width {
                diff[i][j] += diff[i - 1][j] + diff[i][j - 1] - diff[i - 1][j - 1];
                if diff[i][j] == 0 && grid[i - 1][j - 1] == 0 {
                    return false;
                }
            }
        }
        true
    }
}
