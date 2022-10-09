mod invert_binary_tree_solution;
mod merge_two_sorted_lists_solution;
mod reverse_linked_list_solution;
mod two_sum_solution;
mod valid_palindrome_solution;
use reverse_linked_list_solution::{reverse_list, ListNode};
use two_sum_solution::two_sum;

fn main() {
    let v = two_sum(vec![3, 2, 4], 6);
    reverse_list(Option::Some(Box::new(ListNode { val: 2, next: None })));
}
