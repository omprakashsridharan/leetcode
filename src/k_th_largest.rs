use std::cmp::Reverse;
use std::collections::BinaryHeap;

struct KthLargest {
    heap: BinaryHeap<Reverse<i32>>,
    k: i32,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl KthLargest {
    fn new(k: i32, nums: Vec<i32>) -> Self {
        let mut heap = BinaryHeap::with_capacity(k as usize);
        for n in nums {
            if (heap.len() as i32) < k {
                heap.push(Reverse(n));
            } else {
                match heap.peek() {
                    Some(&v) => {
                        if v.0 < n {
                            heap.pop().unwrap();
                            heap.push(Reverse(n));
                        }
                    }
                    None => {}
                }
            }
        }
        KthLargest { k, heap }
    }

    fn add(&mut self, val: i32) -> i32 {
        if self.heap.len() < self.k as usize {
            self.heap.push(Reverse(val));
        } else {
            match self.heap.peek() {
                Some(&v) => {
                    if v.0 < val {
                        self.heap.pop().unwrap();
                        self.heap.push(Reverse(val));
                    }
                }
                None => {}
            }
        }
        match self.heap.peek() {
            Some(v) => v.0,
            None => -1,
        }
    }
}
