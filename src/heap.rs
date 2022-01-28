pub struct Heap<T> {
    data: Vec<T>,
}

impl<T: PartialOrd> Heap<T> {
    pub fn new() -> Self {
        Heap {
            data: Vec::<T>::new(),
        }
    }

    pub fn with_capacity(capacity: usize) -> Self {
        Heap {
            data: Vec::<T>::with_capacity(capacity),
        }
    }

    pub fn insert(&mut self, value: T) {
        if self.data.len() == self.data.capacity() {
            let cap = self.data.capacity();
            self.data.reserve(cap);
        }
        self.data.push(value);
        let i = self.data.len() - 1;
        self.sift_up(i);
    }

    fn sift_up(&mut self, i: usize) {
        match parent_index(i) {
            None => {}
            Some(parent) => {
                if self.data.get(i) < self.data.get(parent) {
                    self.data.swap(i, parent);
                    self.sift_up(parent);
                }
            }
        }
    }

    pub fn pop(&mut self) -> Option<T> {
        if self.data.is_empty() {
            None
        } else {
            let i = self.data.len() - 1;
            self.data.swap(0, i);
            let result = self.data.remove(i);
            self.sift_down(0);
            return Some(result);
        }
    }

    fn sift_down(&mut self, i: usize) {
        let len = self.data.len();
        let l = left_child_index(i);
        let r = right_child_index(i);
        let child = if l >= len && r >= len {
            return;
        } else if l >= len {
            r
        } else if r >= len {
            l
        } else if self.data.get(l) < self.data.get(r) {
            l
        } else {
            r
        };

        if self.data.get(child) < self.data.get(i) {
            self.data.swap(child, i);
            self.sift_down(child);
        }
    }

    pub fn len(&self) -> usize {
        self.data.len()
    }

    pub fn peek(&self) -> Option<&T> {
        match self.data.len() {
            0 => None,
            _ => self.data.get(0),
        }
    }
}

fn parent_index(i: usize) -> Option<usize> {
    if i == 0 {
        None
    } else {
        Some((i - 1) / 2)
    }
}

fn left_child_index(i: usize) -> usize {
    (i + 1) * 2 - 1
}

fn right_child_index(i: usize) -> usize {
    (i + 1) * 2
}

impl<T: PartialOrd> From<Vec<T>> for Heap<T> {
    fn from(values: Vec<T>) -> Self {
        let mut h: Heap<T> = Heap {
            data: Vec::with_capacity(values.len()),
        };
        for value in values {
            h.insert(value);
        }
        return h;
    }
}

#[derive(PartialEq, PartialOrd, Debug)]
struct Book {
    id: i32,
}

#[cfg(test)]
mod tests {
    use crate::heap::Book;

    use super::Heap;

    #[test]
    fn construct_from_vector() {
        let mut h: Heap<i32> = vec![3, 2, 1].into();
        assert_eq!(h.peek(), Some(&1));
        assert_eq!(h.pop(), Some(1));
        assert_eq!(h.pop(), Some(2));
        assert_eq!(h.pop(), Some(3));
    }

    #[test]
    fn construct_from_vector_of_structs_which_implement_partial_ord() {
        let mut h: Heap<Book> = vec![Book { id: 230 }, Book { id: 2 }, Book { id: 930 }].into();
        assert_eq!(h.peek(), Some(&Book { id: 2 }));
        assert_eq!(h.pop(), Some(Book { id: 2 }));
        assert_eq!(h.pop(), Some(Book { id: 230 }));
        assert_eq!(h.pop(), Some(Book { id: 930 }));
    }

    #[test]
    fn empty_heap() {
        let h: Heap<i32> = Heap::new();
        assert_eq!(h.len(), 0);
    }

    #[test]
    fn empty_heap_with_capacity() {
        let h: Heap<i32> = Heap::with_capacity(1);
        assert_eq!(h.len(), 0);
    }
}
