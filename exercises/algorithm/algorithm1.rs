/*
	single linked list merge
	This problem requires you to merge two ordered singly linked lists into one ordered singly linked list
*/

use std::fmt::{self, Display, Formatter};
use std::ptr::NonNull;
use std::vec::*;

#[derive(Debug)]
struct Node<T> {
    val: T,
    next: Option<NonNull<Node<T>>>,
}

impl<T> Node<T> {
    fn new(t: T) -> Node<T> {
        Node {
            val: t,
            next: None,
        }
    }
}
#[derive(Debug)]
struct LinkedList<T> {
    length: u32,
    start: Option<NonNull<Node<T>>>,
    end: Option<NonNull<Node<T>>>,
}

impl<T> Default for LinkedList<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T > LinkedList<T> {
    pub fn new() -> Self {
        Self {
            length: 0,
            start: None,
            end: None,
        }
    }

    pub fn add(&mut self, obj: T) {
        let mut node = Box::new(Node::new(obj));
        node.next = None;
        let node_ptr = Some(unsafe { NonNull::new_unchecked(Box::into_raw(node)) });
        match self.end {
            None => self.start = node_ptr,
            Some(end_ptr) => unsafe { (*end_ptr.as_ptr()).next = node_ptr },
        }
        self.end = node_ptr;
        self.length += 1;
    }

    pub fn get(&mut self, index: i32) -> Option<&T> {
        self.get_ith_node(self.start, index)
    }

    fn get_ith_node(&mut self, node: Option<NonNull<Node<T>>>, index: i32) -> Option<&T> {
        match node {
            None => None,
            Some(next_ptr) => match index {
                0 => Some(unsafe { &(*next_ptr.as_ptr()).val }),
                _ => self.get_ith_node(unsafe { (*next_ptr.as_ptr()).next }, index - 1),
            },
        }
    }

    pub fn merge(mut list_a: LinkedList<T>, mut list_b: LinkedList<T>) -> Self
    where
        T: PartialOrd, // 只需要 PartialOrd
    {
        // 从 list_a 和 list_b 中取出头节点，使它们变为空链表。
        // 这样可以防止它们的 Drop 实现释放我们即将移动到新链表中的节点。
        let mut p = list_a.start.take();
        let mut q = list_b.start.take();

        // 处理其中一个或两个链表为空的初始情况
        if p.is_none() {
            return list_b;
        }
        if q.is_none() {
            return list_a;
        }

        // 创建最终要返回的新链表
        let mut result = LinkedList::new();
        result.length = list_a.length + list_b.length;

        // 使用一个 "tail" 引用来指向新链表尾部的 `next` 字段，
        // 这样我们就可以在 O(1) 时间内追加节点。
        let mut tail = &mut result.start;

        unsafe {
            // 循环直到其中一个链表被完全合并
            while let (Some(p_node), Some(q_node)) = (p, q) {
                if p_node.as_ref().val < q_node.as_ref().val {
                    // p 的值更小，将 p 移动到新链表
                    *tail = p;
                    // p 指向它的下一个节点
                    p = (*p_node.as_ptr()).next;
                    // 更新 tail，使其指向新链表尾部节点的 `next` 字段
                    tail = &mut (*p_node.as_ptr()).next;
                } else {
                    // q 的值更小或相等，将 q 移动到新链表
                    *tail = q;
                    // q 指向它的下一个节点
                    q = (*q_node.as_ptr()).next;
                    // 更新 tail
                    tail = &mut (*q_node.as_ptr()).next;
                }
            }

            // 此时，p 或 q 中最多只有一个还有剩余节点。
            // 将剩余的整个链表部分直接链接到新链表的尾部。
            *tail = p.or(q);
        }

        // 更新新链表的尾指针 end
        if let Some(mut current) = result.start {
            while let Some(next) = unsafe { (*current.as_ptr()).next } {
                current = next;
            }
            result.end = Some(current);
        }


        result
    }
}


impl<T> Display for LinkedList<T>
where
    T: Display,
{
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self.start {
            Some(node) => write!(f, "{}", unsafe { node.as_ref() }),
            None => Ok(()),
        }
    }
}

impl<T> Display for Node<T>
where
    T: Display,
{
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self.next {
            Some(node) => write!(f, "{}, {}", self.val, unsafe { node.as_ref() }),
            None => write!(f, "{}", self.val),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::LinkedList;

    #[test]
    fn create_numeric_list() {
        let mut list = LinkedList::<i32>::new();
        list.add(1);
        list.add(2);
        list.add(3);
        println!("Linked List is {}", list);
        assert_eq!(3, list.length);
    }

    #[test]
    fn create_string_list() {
        let mut list_str = LinkedList::<String>::new();
        list_str.add("A".to_string());
        list_str.add("B".to_string());
        list_str.add("C".to_string());
        println!("Linked List is {}", list_str);
        assert_eq!(3, list_str.length);
    }

    #[test]
    fn test_merge_linked_list_1() {
		let mut list_a = LinkedList::<i32>::new();
		let mut list_b = LinkedList::<i32>::new();
		let vec_a = vec![1,3,5,7];
		let vec_b = vec![2,4,6,8];
		let target_vec = vec![1,2,3,4,5,6,7,8];
		
		for i in 0..vec_a.len(){
			list_a.add(vec_a[i]);
		}
		for i in 0..vec_b.len(){
			list_b.add(vec_b[i]);
		}
		println!("list a {} list b {}", list_a,list_b);
		let mut list_c = LinkedList::<i32>::merge(list_a,list_b);
		println!("merged List is {}", list_c);
		for i in 0..target_vec.len(){
			assert_eq!(target_vec[i],*list_c.get(i as i32).unwrap());
		}
	}
	#[test]
	fn test_merge_linked_list_2() {
		let mut list_a = LinkedList::<i32>::new();
		let mut list_b = LinkedList::<i32>::new();
		let vec_a = vec![11,33,44,88,89,90,100];
		let vec_b = vec![1,22,30,45];
		let target_vec = vec![1,11,22,30,33,44,45,88,89,90,100];

		for i in 0..vec_a.len(){
			list_a.add(vec_a[i]);
		}
		for i in 0..vec_b.len(){
			list_b.add(vec_b[i]);
		}
		println!("list a {} list b {}", list_a,list_b);
		let mut list_c = LinkedList::<i32>::merge(list_a,list_b);
		println!("merged List is {}", list_c);
		for i in 0..target_vec.len(){
			assert_eq!(target_vec[i],*list_c.get(i as i32).unwrap());
		}
	}
}