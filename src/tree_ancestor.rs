/**
 * 1483. 树节点的第 K 个祖先
 * 给你一棵树，树上有 n 个节点，按从 0 到 n-1 编号。树以父节点数组的形式给出，其中 parent[i] 是节点 i 的父节点。树的根节点是编号为 0 的节点。
 * 树节点的第 k 个祖先节点是从该节点到根节点路径上的第 k 个节点。
 * 实现 TreeAncestor 类：
 *     TreeAncestor（int n， int[] parent） 对树和父数组中的节点数初始化对象。
 *     getKthAncestor(int node, int k) 返回节点 node 的第 k 个祖先节点。如果不存在这样的祖先节点，返回 -1 。
 */
pub struct TreeAncestor {
    inner: Vec<Vec<i32>>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl TreeAncestor {
    pub fn new(n: i32, parent: Vec<i32>) -> Self {
        let mut m = 32;
        for i in 0..31 {
            if 1 << i > n {
                m = i;
                break;
            }
        }
        let mut inner = vec![Vec::with_capacity(m as usize); n as usize];
        parent
            .iter()
            .enumerate()
            .for_each(|(idx, &n)| inner[idx].push(n));
        for i in 1..m as usize {
            for j in 0..n as usize {
                let father = inner[j][i - 1];
                if father == -1 {
                    inner[j].push(-1);
                } else {
                    let grand_father = inner[father as usize][i - 1];
                    inner[j].push(grand_father);
                }
            }
        }
        Self { inner }
    }

    pub fn get_kth_ancestor(&self, node: i32, k: i32) -> i32 {
        let mut p = self.inner[node as usize][0];
        for i in 1..31 {
            if k >> i == 0 {
                break;
            }
            if k >> i & 1 == 1 {
                p = self.inner[p as usize][i];
                if p == -1 {
                    return -1;
                }
            }
        }
        p
    }
}
