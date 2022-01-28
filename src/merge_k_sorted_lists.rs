#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }
}
struct Solution;

// use std::collections::BTreeMap;

// impl Solution {
//     pub fn merge_k_lists(lists: Vec<Option<Box<ListNode>>>) -> Option<Box<ListNode>> {
//         let mut result: Option<Box<ListNode>> = None;
//         let mut elements: BTreeMap<i32, i32> = BTreeMap::new();
//         for l_option in lists {
//             match l_option {
//                 Some(l) => {
//                     let mut temp = l;
//                     loop {
//                         *elements.entry(temp.val).or_insert(0) += 1;
//                         match temp.next {
//                             Some(n) => {
//                                 temp = n;
//                             }
//                             None => {
//                                 break;
//                             }
//                         }
//                     }
//                 }
//                 None => {}
//             }
//         }
//         for (&key, &value) in elements.iter().rev() {
//             for _n in 0..value {
//                 let mut node = Box::new(ListNode::new(key));
//                 node.next = result;
//                 result = Some(node);
//             }
//         }
//         result
//     }
// }

use std::{cmp::Reverse, collections::BinaryHeap};

impl PartialOrd for ListNode {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.val.partial_cmp(&other.val)
    }
}

impl Ord for ListNode {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.val.cmp(&other.val)
    }
}

impl Solution {
    pub fn merge_k_lists(lists: Vec<Option<Box<ListNode>>>) -> Option<Box<ListNode>> {
        let mut min_heap = BinaryHeap::new();
        for list in lists {
            if let Some(node) = list {
                min_heap.push(Reverse(node));
            }
        }
        let mut dummy_head = ListNode::new(0);
        let mut cur = &mut dummy_head;
        while let Some(Reverse(node)) = min_heap.pop() {
            cur.next = Some(Box::new(ListNode::new(node.val)));
            cur = cur.next.as_mut().unwrap();
            if let Some(next) = node.next {
                min_heap.push(Reverse(next));
            }
        }
        return dummy_head.next;
    }
}
