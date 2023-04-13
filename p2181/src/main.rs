fn main() {
    let ref_vec = vec![0,3,1,0,4,5,2,0];
    let mut node = ListNode::new(0);
}

#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
  pub val: i32,
  pub next: Option<Box<ListNode>>
}

impl ListNode {
  #[inline]
  fn new(val: i32) -> Self {
    ListNode {
      next: None,
      val
    }
  }
}

struct Solution {}

impl Solution {
    pub fn merge_nodes(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut result = None;

        let mut node = head;

        let mut trigger = false;
        let mut sum = 0;

        loop {
            if let Some(v) = node {
                if v.val == 0 {
                    trigger = !trigger;
                    result = Some(Box::new(ListNode::new(sum)));
                    sum = 0;
                } else {
                    sum += v.val;
                }

                node = v.next;
            } else {
                break;
            }
        }

        result
    }
}