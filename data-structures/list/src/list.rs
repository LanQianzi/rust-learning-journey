use std::marker::PhantomData;
use std::ptr::NonNull;

pub struct List<T> {
    head: Link<T>,
    tail: Link<T>,
    len: usize,
}

struct Node<T> {
    elem: T,
    next: Link<T>,
}

type Link<T> = Option<NonNull<Node<T>>>;

impl<T> Node<T> {
    fn new(elem: T) -> Self {
        Self { elem, next: None }
    }
}

impl<T> List<T> {
    pub fn new() -> Self {
        Self {
            head: None,
            tail: None,
            len: 0,
        }
    }

    pub fn push_back(&mut self, elem: T) {
        let new = unsafe { NonNull::new_unchecked(Box::into_raw(Box::new(Node::new(elem)))) };
        if let Some(tail) = self.tail {
            unsafe {
                (*tail.as_ptr()).next = Some(new);
            }
        }
        self.tail = Some(new);
        if self.head.is_none() {
            self.head = self.tail;
        }
        self.len += 1;
    }

    pub fn pop(&mut self) -> Option<T> {
        self.head.map(|node| {
            let box_ptr = unsafe { Box::from_raw(node.as_ptr()) };
            self.head = box_ptr.next;
            let res = box_ptr.elem;
            if self.head.is_none() {
                debug_assert!(self.len == 1);
                self.tail = None;
            }
            self.len -= 1;
            res
        })
    }

    pub fn front(&self) -> Option<&T> {
        self.head.map(|node| unsafe { &(*node.as_ptr()).elem })
    }

    pub fn front_mut(&mut self) -> Option<&mut T> {
        self.head.map(|node| unsafe { &mut (*node.as_ptr()).elem })
    }

    pub fn back(&self) -> Option<&T> {
        self.tail.map(|node| unsafe { &(*node.as_ptr()).elem })
    }

    pub fn back_mut(&mut self) -> Option<&mut T> {
        self.tail.map(|node| unsafe { &mut (*node.as_ptr()).elem })
    }
}

impl<T> Drop for List<T> {
    fn drop(&mut self) {
        while let Some(_) = self.pop() {}
    }
}

pub struct Iter<'a, T> {
    head: Link<T>,
    len: usize,
    _boo: PhantomData<&'a T>,
}

impl<T> List<T> {
    pub fn iter(&self) -> Iter<T> {
        Iter {
            head: self.head,
            len: self.len,
            _boo: PhantomData,
        }
    }
}

impl<'a, T> Iterator for Iter<'a, T> {
    type Item = &'a T;
    fn next(&mut self) -> Option<Self::Item> {
        if self.len > 0 {
            self.head.map(|node| {
                self.len -= 1;
                self.head = unsafe { (*node.as_ptr()).next };
                unsafe { &(*node.as_ptr()).elem }
            })
        } else {
            None
        }
    }
}

pub struct IterMut<'a, T> {
    ptr: Link<T>,
    len: usize,
    _boo: PhantomData<&'a mut T>,
}

impl<T> List<T> {
    pub fn iter_mut(&mut self) -> IterMut<'_, T> {
        IterMut {
            ptr: self.head,
            len: self.len,
            _boo: PhantomData,
        }
    }
}

impl<'a, T> Iterator for IterMut<'a, T> {
    type Item = &'a mut T;
    fn next(&mut self) -> Option<Self::Item> {
        if self.len > 0 {
            self.ptr.map(|node| {
                self.len -= 1;
                self.ptr = unsafe { (*node.as_ptr()).next };
                unsafe { &mut (*node.as_ptr()).elem }
            })
        } else {
            None
        }
    }
}

pub struct IntoIter<T> {
    list: List<T>,
}

impl<T> IntoIterator for List<T> {
    type Item = T;
    type IntoIter = IntoIter<T>;
    fn into_iter(self) -> Self::IntoIter {
        IntoIter { list: self }
    }
}

impl<T> Iterator for IntoIter<T> {
    type Item = T;
    fn next(&mut self) -> Option<Self::Item> {
        self.list.pop()
    }
}

impl<'a, T> IntoIterator for &'a List<T> {
    type Item = &'a T;
    type IntoIter = Iter<'a, T>;
    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

impl<'a, T> IntoIterator for &'a mut List<T> {
    type Item = &'a mut T;
    type IntoIter = IterMut<'a, T>;
    fn into_iter(self) -> Self::IntoIter {
        self.iter_mut()
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn list_test() {
        let mut list = List::new();
        list.push_back(1.1);
        list.push_back(2.2);
        list.push_back(3.3);
        assert_eq!(list.front(), Some(&1.1));
        list.front_mut().map(|fr| *fr += 10.0);
        assert_eq!(list.front(), Some(&11.1));
        assert_eq!(list.back(), Some(&3.3));
        list.back_mut().map(|bk| *bk += 10.0);
        assert_eq!(list.back(), Some(&13.3));

        assert_eq!(list.pop(), Some(11.1));
        assert_eq!(list.pop(), Some(2.2));
        assert_eq!(list.pop(), Some(13.3));
    }

    #[test]
    fn iter_test() {
        let vc = vec![1, 2, 3, 4, 5];
        let mut list = List::new();
        for v in &vc {
            list.push_back(*v);
        }

        let mut idx = 0usize;
        for elem in &list {
            assert_eq!(elem, &vc[idx]);
            idx += 1;
        }

        for elem in &mut list {
            *elem += 10
        }

        let mut idx = 0usize;
        for elem in &list {
            assert_eq!(elem, &(vc[idx] + 10));
            idx += 1;
        }

        let mut idx = 0usize;
        for elem in list {
            assert_eq!(elem, vc[idx] + 10);
            idx += 1;
        }
    }
}
