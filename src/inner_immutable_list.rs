#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }
}

impl PartialOrd for ListNode {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(&other))
    }
}

impl Ord for ListNode {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other.val.cmp(&self.val)
    }
}

pub struct Solution;
impl Solution {
    /**
     * 82. 删除排序链表中的重复元素 II
     * 给定一个已排序的链表的头 head，
     * 删除原始链表中所有重复数字的节点，只留下不同的数字 。
     * 返回已排序的链表 。
     */
    pub fn delete_duplicates(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut result = Box::new(ListNode::new(0));
        result.next = head;
        let mut curr = &mut result;
        let mut next = curr.next.take();
        'outer: while let Some(node) = next.as_mut() {
            let mut p = node.next.take();
            if p.is_none() {
                curr.next = next;
                break;
            }
            if p.as_ref().unwrap().val != node.val {
                curr.next = next;
                curr = curr.next.as_mut().unwrap();
                next = p;
                continue;
            }
            while let Some(mut n) = p {
                if n.val != node.val {
                    next = Some(n);
                    continue 'outer;
                }
                p = n.next.take();
            }
            break;
        }
        result.next
    }

    /**
     * 23. 合并 K 个升序链表
     * 给你一个链表数组，每个链表都已经按升序排列。
     * 请你将所有链表合并到一个升序链表中，返回合并后的链表。
     */
    pub fn merge_k_lists(lists: Vec<Option<Box<ListNode>>>) -> Option<Box<ListNode>> {
        use std::collections::BinaryHeap;
        let mut node = None;
        let mut cur = &mut node;
        let mut heap = BinaryHeap::new();
        for list in lists.into_iter() {
            if let Some(x) = list {
                heap.push(x);
            }
        }
        while let Some(mut x) = heap.pop() {
            if let Some(y) = x.next.take() {
                heap.push(y);
            }
            cur = &mut cur.insert(x).next;
        }
        node
    }
    /**
     * 148. 排序链表
     * 给你链表的头结点 head ，请将其按 升序 排列并返回 排序后的链表 。
     */
    pub fn sort_list(mut head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut addresses = vec![];
        while head.is_some() {
            let mut temp = head.unwrap();
            head = temp.next.take();
            addresses.push(temp);
        }
        addresses.sort_unstable_by(|a, b| b.val.cmp(&a.val));
        for mut node in addresses {
            node.next = head;
            head = Some(node);
        }
        head
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_solution() {
        let link_list_val = [1, 2, 3, 3, 4, 4, 5];
        let mut head = None;
        for &val in link_list_val.iter().rev() {
            let mut new_node = ListNode::new(val);
            new_node.next = head;
            head = Some(Box::new(new_node));
        }
        let mut head = Solution::delete_duplicates(head);
        let mut result = Vec::new();
        while let Some(node) = head {
            result.push(node.val);
            head = node.next;
        }
        assert_eq!(result, vec![1, 2, 5], "The results are not as expected.");
    }
}
