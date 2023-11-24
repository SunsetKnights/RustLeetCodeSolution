pub mod merge_k_lists;
use merge_k_lists::*;

fn main() {
    let mut param: Vec<Option<Box<ListNode>>> = Vec::new();
    let a0 = Some(Box::new(ListNode::new(1)));
    let a1 = Some(Box::new(ListNode::new(4)));
    let a2 = Some(Box::new(ListNode::new(5)));
    a0.clone().unwrap().next = a1.clone();
    a1.unwrap().next = a2;
    param.push(a0);
    let b0 = Some(Box::new(ListNode::new(1)));
    let b1 = Some(Box::new(ListNode::new(3)));
    let b2 = Some(Box::new(ListNode::new(4)));
    b0.clone().unwrap().next = b1.clone();
    b1.unwrap().next = b2;
    param.push(b0);
    let c0 = Some(Box::new(ListNode::new(2)));
    let c1 = Some(Box::new(ListNode::new(6)));
    c0.clone().unwrap().next = c1;
    param.push(c0);
    let result = Solution::merge_k_lists(param);
    println!("{:?}", result);
}
