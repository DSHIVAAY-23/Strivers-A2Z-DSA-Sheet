

impl Solution {
    pub fn reverse_list(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut curr = head;
        let mut next = None;
        while let Some(mut inner) = curr {
            curr = inner.next.take();
            inner.next = next;
            next = Some(inner);
        }
        next
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_206() {
        assert_eq!(
            Solution::reverse_list(linked![1, 2, 3, 4, 5]),
            linked![5, 4, 3, 2, 1]
        );
    }
}
