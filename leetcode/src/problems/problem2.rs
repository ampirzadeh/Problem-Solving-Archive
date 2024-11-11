pub struct Solution;

#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    // #[inline]
    pub fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }
}

impl Solution {
    pub fn add_two_numbers(
        l1: Option<Box<ListNode>>,
        l2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        let mut l1 = l1;
        let mut l2 = l2;
        let mut res_head = Some(Box::new(ListNode::new(0)));
        let mut current = res_head.as_mut();

        let mut carry = 0;

        while l1.is_some() || l2.is_some() || carry != 0 {
            if let Some(node) = current {
                let sum =
                    carry + l1.as_ref().map_or(0, |n| n.val) + l2.as_ref().map_or(0, |n| n.val);
                carry = sum / 10;

                node.next = Some(Box::new(ListNode::new(sum % 10)));
                current = node.next.as_mut();
            }

            l1 = l1.and_then(|node| node.next);
            l2 = l2.and_then(|node| node.next);
        }

        res_head.unwrap().next
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn list_node_maker(val: i32, next: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        Some(Box::new(ListNode { val, next }))
    }

    #[test]
    fn test_1() {
        let l1 = list_node_maker(2, list_node_maker(4, list_node_maker(3, None)));
        let l2 = list_node_maker(5, list_node_maker(6, list_node_maker(4, None)));
        let output = list_node_maker(7, list_node_maker(0, list_node_maker(8, None)));

        assert_eq!(Solution::add_two_numbers(l1, l2), output);
    }

    #[test]
    fn test_2() {
        let l1 = list_node_maker(0, None);
        let l2 = list_node_maker(0, None);
        let output = list_node_maker(0, None);

        assert_eq!(Solution::add_two_numbers(l1, l2), output);
    }

    #[test]
    fn test_3() {
        let l1 = list_node_maker(
            9,
            list_node_maker(
                9,
                list_node_maker(
                    9,
                    list_node_maker(
                        9,
                        list_node_maker(9, list_node_maker(9, list_node_maker(9, None))),
                    ),
                ),
            ),
        );
        let l2 = list_node_maker(
            9,
            list_node_maker(9, list_node_maker(9, list_node_maker(9, None))),
        );
        let output = list_node_maker(
            8,
            list_node_maker(
                9,
                list_node_maker(
                    9,
                    list_node_maker(
                        9,
                        list_node_maker(
                            0,
                            list_node_maker(0, list_node_maker(0, list_node_maker(1, None))),
                        ),
                    ),
                ),
            ),
        );

        assert_eq!(Solution::add_two_numbers(l1, l2), output);
    }
}
