pub struct Solution {}
/**
 * 2258. 逃离火灾
 * 给你一个下标从 0 开始大小为 m x n 的二维整数数组 grid ，它表示一个网格图。每个格子为下面 3 个值之一：
 * 0 表示草地。
 * 1 表示着火的格子。
 * 2 表示一座墙，你跟火都不能通过这个格子。
 * 一开始你在最左上角的格子 (0, 0) ，你想要到达最右下角的安全屋格子 (m - 1, n - 1) 。每一分钟，你可以移动到 相邻 的草地格子。
 * 每次你移动 之后 ，着火的格子会扩散到所有不是墙的 相邻 格子。
 * 请你返回你在初始位置可以停留的 最多 分钟数，且停留完这段时间后你还能安全到达安全屋。如果无法实现，请你返回 -1 。
 * 如果不管你在初始位置停留多久，你 总是 能到达安全屋，请你返回 10^9 。
 * 注意，如果你到达安全屋后，火马上到了安全屋，这视为你能够安全到达安全屋。
 * 如果两个格子有共同边，那么它们为 相邻 格子。
 */
impl Solution {
    pub fn maximum_minutes(grid: Vec<Vec<i32>>) -> i32 {
        let person_time = Self::min_time(vec![(0, 0)], &grid);
        let mut fire_locations = Vec::new();
        for i in 0..grid.len() {
            for j in 0..grid[0].len() {
                if grid[i][j] == 1 {
                    fire_locations.push((i as i32, j as i32));
                }
            }
        }
        let fire_time = Self::min_time(fire_locations, &grid);
        let distance = fire_time.0 - person_time.0;
        if person_time.0 == 0x7FFFFFFF || distance < 0 {
            return -1;
        } else if fire_time.0 == 0x7FFFFFFF {
            return 1000000000;
        } else {
            if distance < fire_time.1 - person_time.1 || distance < fire_time.2 - person_time.2 {
                return distance;
            }
            return distance - 1;
        }
    }

    //求出location中的格子到右下角，右下角的左边，右下角的上面的最短时间
    pub fn min_time(mut location: Vec<(i32, i32)>, grid: &Vec<Vec<i32>>) -> (i32, i32, i32) {
        let row = grid.len();
        let col = grid[0].len();
        let mut result = (0x7FFFFFFF, 0x7FFFFFFF, 0x7FFFFFFF);
        let mut times = vec![vec![0x7FFFFFFF; col]; row];
        for p in location.iter() {
            times[p.0 as usize][p.1 as usize] = 0;
        }
        let mut time = 1;
        while !location.is_empty() {
            let mut layer = Vec::new();
            for p in location {
                for (i, j) in [
                    (p.0 + 1, p.1),
                    (p.0 - 1, p.1),
                    (p.0, p.1 + 1),
                    (p.0, p.1 - 1),
                ] {
                    if i >= 0
                        && (i < row as i32)
                        && j >= 0
                        && (j < col as i32)
                        && grid[i as usize][j as usize] != 2
                        && times[i as usize][j as usize] == 0x7FFFFFFF
                    {
                        times[i as usize][j as usize] = time;
                        layer.push((i, j));
                    }
                }
            }
            time += 1;
            location = layer;
        }
        result.0 = times[row - 1][col - 1];
        result.1 = times[row - 1][col - 2];
        result.2 = times[row - 2][col - 1];
        result
    }
}
