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
pub fn run() {
    pub fn add_two_numbers(

        l1: Option<Box<ListNode>>,
        l2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        fn list_to_number(list: Option<Box<ListNode>>) -> i32 {
            let mut l = list;
            let mut end = false;
            let mut number = 0;
            while !(end) {
                match &l {
                    Some(val) => {
                        number = number * 10 + (**val).val;
                        l = (**val).clone().next
                    }
                    None => end = true,
                }
            }
            number.to_string().chars().rev().collect::<String>().parse::<i32>().unwrap()
        }

        let mut sum = list_to_number(l1) + list_to_number(l2);
        let mut digit; 
        let mut node;
        let mut prev_node:Option<Box<ListNode>> = None;
        while sum != 0 {
            
            digit = sum % 10;
            node = ListNode::new(digit);
            node.next = prev_node.clone();
            prev_node = Some(Box::new(node));
            digit = digit / 10;
        }
        prev_node
    }
}
