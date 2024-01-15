pub mod delete_duplicates;
use delete_duplicates::*;

fn main() {
    let link_list_val = [1, 2, 3, 3, 4, 4, 5];
    let mut head = None;
    for &val in link_list_val.iter().rev() {
        let mut new_node = ListNode::new(val);
        new_node.next = head;
        head = Some(Box::new(new_node));
    }
    let mut result = Solution::delete_duplicates(head);
    let mut print_string = String::new();
    while let Some(node) = result {
        print_string.push_str(node.val.to_string().as_str());
        print_string.push_str("->");
        result = node.next;
    }
    print_string.push_str("None");
    println!("{print_string}");
}
