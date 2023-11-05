use std::f64;

pub struct Solution {}
impl Solution {
    /**
     *给你一个整数数组 gifts ，表示各堆礼物的数量。每一秒，你需要执行以下操作：
     *选择礼物数量最多的那一堆。
     *如果不止一堆都符合礼物数量最多，从中选择任一堆即可。
     *选中的那一堆留下平方根数量的礼物（向下取整），取走其他的礼物。
     *返回在 k 秒后剩下的礼物数量。
     */
    pub fn pick_gifts(gifts: Vec<i32>, k: i32) -> i64 {
        let mut sortable_gifts = gifts.clone();
        sortable_gifts.sort_unstable();
        let mut temp: i32;
        //此方法应当不是最优解！！！
        for _i in 0..k {
            temp = sortable_gifts.pop().unwrap();
            temp = f64::sqrt(temp as f64) as i32;
            sortable_gifts.push(temp);
            sortable_gifts.sort_unstable();
        }
        let mut result: i64 = 0;
        for i in sortable_gifts {
            result = result + i as i64;
        }
        result
    }
}
