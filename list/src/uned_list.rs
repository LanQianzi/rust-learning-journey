use std::clone::Clone;
use std::default::Default;
use std::fmt::Debug;
use std::hash::Hash;
use std::iter::Extend;
use std::marker::PhantomData;
use std::ptr::NonNull;

pub struct EDList<T> {
    front: Link<T>,
    back: Link<T>,
    len: usize,
    _boo: PhantomData<T>,
}

type Link<T> = Option<NonNull<Node<T>>>;

struct Node<T> {
    next: Link<T>,
    prev: Link<T>,
    elem: T,
}

pub struct Iter<'a, T> {
    front: Link<T>,
    back: Link<T>,
    len: usize,
    _boo: PhantomData<&'a T>,
}

pub struct IterMut<'a, T> {
    front: Link<T>,
    back: Link<T>,
    len: usize,
    _boo: PhantomData<&'a mut T>,
}

pub struct IntoIter<T> {
    list: EDList<T>,
}

impl<T> Node<T> {
    pub fn new(elem: T) -> NonNull<Node<T>> {
        unsafe {
            NonNull::new_unchecked(Box::into_raw(Box::new(Node {
                next: None,
                prev: None,
                elem,
            })))
        }
    }
}

impl<T> EDList<T> {
    pub fn new() -> Self {
        Self {
            front: None,
            back: None,
            len: 0,
            _boo: PhantomData,
        }
    }

    pub fn len(&self) -> usize {
        self.len
    }

    pub fn push_front(&mut self, elem: T) {
        let new = Node::new(elem);
        if let Some(old) = self.front {
            unsafe {
                (*new.as_ptr()).next = Some(old);
                (*old.as_ptr()).prev = Some(new);
            }
        } else {
            self.back = Some(new);
        }
        self.front = Some(new);
        self.len += 1;
    }

    pub fn push_back(&mut self, elem: T) {
        let nb = Node::new(elem);
        if let Some(ob) = self.back {
            unsafe {
                (*ob.as_ptr()).next = Some(nb);
                (*nb.as_ptr()).prev = Some(ob);
            }
        } else {
            self.front = Some(nb);
        }
        self.back = Some(nb);
    }

    pub fn pop_front(&mut self) -> Option<T> {
        self.front.take().map(|front| {
            let box_front = unsafe { Box::from_raw(front.as_ptr()) };
            self.front = box_front.next;
            let res = box_front.elem;
            if let Some(now) = self.front {
                unsafe { (*now.as_ptr()).prev = None };
            } else {
                self.back = None
            }
            self.len -= 1;
            res
        })
    }

    pub fn pop_back(&mut self) -> Option<T> {
        self.back.take().map(|back| {
            let box_back = unsafe { Box::from_raw(back.as_ptr()) };
            let res = box_back.elem;
            self.back = box_back.prev;
            if let Some(now) = self.back {
                unsafe { (*now.as_ptr()).next = None }
            } else {
                self.front = None
            }

            self.len -= 1;
            res
        })
    }
}

impl<T> Drop for EDList<T> {
    fn drop(&mut self) {
        while let Some(_) = self.pop_front() {}
    }
}

impl<T> EDList<T> {
    pub fn front(&self) -> Option<&T> {
        self.front.map(|node| unsafe { &(*node.as_ptr()).elem })
    }

    pub fn front_mut(&mut self) -> Option<&mut T> {
        self.front.map(|node| unsafe { &mut (*node.as_ptr()).elem })
    }

    pub fn back(&self) -> Option<&T> {
        self.back.map(|node| unsafe { &(*node.as_ptr()).elem })
    }

    pub fn back_mut(&mut self) -> Option<&mut T> {
        self.back.map(|node| unsafe { &mut (*node.as_ptr()).elem })
    }
}

//Other features
impl<T> EDList<T> {
    pub fn is_empty(&self) -> bool {
        self.len == 0
    }

    pub fn clear(&mut self) {
        while let Some(_) = self.pop_back() {}
    }
}

impl<T> EDList<T> {
    pub fn iter(&self) -> Iter<T> {
        Iter {
            front: self.front,
            back: self.back,
            len: self.len,
            _boo: PhantomData,
        }
    }
}

impl<'a, T> IntoIterator for &'a EDList<T> {
    type IntoIter = Iter<'a, T>;
    type Item = &'a T;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

impl<'a, T> Iterator for Iter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        if self.len <= 0 {
            return None;
        }
        self.front.map(|node| unsafe {
            self.len -= 1;
            self.front = (*node.as_ptr()).next;
            &(*node.as_ptr()).elem
        })
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        (self.len, Some(self.len))
    }
}

impl<'a, T> DoubleEndedIterator for Iter<'a, T> {
    fn next_back(&mut self) -> Option<Self::Item> {
        if self.len <= 0 {
            return None;
        }

        self.back.map(|node| unsafe {
            self.len -= 1;
            self.back = (*node.as_ptr()).prev;
            &(*node.as_ptr()).elem
        })
    }
}

impl<'a, T> ExactSizeIterator for Iter<'a, T> {
    fn len(&self) -> usize {
        self.len
    }
}

impl<T> EDList<T> {
    pub fn mut_iter(&mut self) -> IterMut<T> {
        IterMut {
            front: self.front,
            back: self.back,
            len: self.len,
            _boo: PhantomData,
        }
    }
}

impl<'a, T> IntoIterator for &'a mut EDList<T> {
    type IntoIter = IterMut<'a, T>;
    type Item = &'a mut T;

    fn into_iter(self) -> Self::IntoIter {
        self.mut_iter()
    }
}

impl<'a, T> Iterator for IterMut<'a, T> {
    type Item = &'a mut T;

    fn next(&mut self) -> Option<Self::Item> {
        if self.len <= 0 {
            return None;
        }

        self.front.take().map(|node| unsafe {
            self.front = (*node.as_ptr()).next;
            self.len += 1;
            &mut (*node.as_ptr()).elem
        })
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        (self.len, Some(self.len))
    }
}

impl<'a, T> DoubleEndedIterator for IterMut<'a, T> {
    fn next_back(&mut self) -> Option<Self::Item> {
        if self.len <= 0 {
            return None;
        }
        self.back.take().map(|node| unsafe {
            self.back = (*node.as_ptr()).prev;
            self.len -= 1;
            &mut (*node.as_ptr()).elem
        })
    }
}

impl<T> EDList<T> {
    pub fn into_iter(self) -> IntoIter<T> {
        IntoIter { list: self }
    }
}

impl<T> IntoIterator for EDList<T> {
    type IntoIter = IntoIter<T>;
    type Item = T;

    fn into_iter(self) -> Self::IntoIter {
        self.into_iter()
    }
}

impl<T> Iterator for IntoIter<T> {
    type Item = T;
    fn next(&mut self) -> Option<Self::Item> {
        self.list.pop_front()
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let len = self.list.len();
        (len, Some(len))
    }
}

impl<T> DoubleEndedIterator for IntoIter<T> {
    fn next_back(&mut self) -> Option<Self::Item> {
        self.list.pop_back()
    }
}

impl<T> ExactSizeIterator for IntoIter<T> {
    fn len(&self) -> usize {
        self.list.len()
    }
}

impl<T> Default for EDList<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T: Clone> Clone for EDList<T> {
    fn clone(&self) -> Self {
        let mut new = Self::new();
        for iter in self {
            new.push_back(iter.clone());
        }
        new
    }
}

impl<T> Extend<T> for EDList<T> {
    fn extend<I: IntoIterator<Item = T>>(&mut self, iter: I) {
        for i in iter {
            self.push_back(i);
        }
    }
}

impl<T> FromIterator<T> for EDList<T> {
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
        let mut list = Self::new();
        list.extend(iter);
        list
    }
}

impl<T: Debug> Debug for EDList<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_list().entries(self).finish()
    }
}

impl<T: PartialEq> PartialEq for EDList<T> {
    fn eq(&self, other: &Self) -> bool {
        self.len() == other.len() && self.iter().eq(other)
    }

    fn ne(&self, other: &Self) -> bool {
        self.len() != other.len() && self.iter().ne(other)
    }
}

impl<T: PartialOrd> PartialOrd for EDList<T> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.iter().partial_cmp(other)
    }
}

impl<T: Ord> Ord for EDList<T> {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.iter().cmp(other)
    }
}

impl<T: Hash> Hash for EDList<T> {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.len().hash(state);
        for item in self {
            item.hash(state);
        }
    }
}

impl<T: Eq> Eq for EDList<T> {}
#[cfg(test)]
mod test {
    use super::EDList;
    #[test]
    fn test_basic_front() {
        let mut list = EDList::new();

        // Try to break an empty list
        assert_eq!(list.len(), 0);
        assert_eq!(list.pop_front(), None);
        assert_eq!(list.len(), 0);

        // Try to break a one item list
        list.push_front(10);
        assert_eq!(list.len(), 1);
        assert_eq!(list.pop_front(), Some(10));
        assert_eq!(list.len(), 0);
        assert_eq!(list.pop_front(), None);
        assert_eq!(list.len(), 0);

        // Mess around
        list.push_front(10);
        assert_eq!(list.len(), 1);
        list.push_front(20);
        assert_eq!(list.len(), 2);
        list.push_front(30);
        assert_eq!(list.len(), 3);
        assert_eq!(list.pop_front(), Some(30));
        assert_eq!(list.len(), 2);
        list.push_front(40);
        assert_eq!(list.len(), 3);
        assert_eq!(list.pop_front(), Some(40));
        assert_eq!(list.len(), 2);
        assert_eq!(list.pop_front(), Some(20));
        assert_eq!(list.len(), 1);
        assert_eq!(list.pop_front(), Some(10));
        assert_eq!(list.len(), 0);
        assert_eq!(list.pop_front(), None);
        assert_eq!(list.len(), 0);
        assert_eq!(list.pop_front(), None);
        assert_eq!(list.len(), 0);
    }

    #[test]
    fn test_iter() {
        let mut list = EDList::new();
        list.push_front(1.1);
        list.push_front(2.1);
        list.push_front(3.1);

        for i in &list {
            println!("{i}")
        }
        println!("=======================");

        for mi in &mut list {
            *mi += 1.0
        }

        for i in &list {
            println!("{i}")
        }
        println!("=======================");

        for into in list {
            println!("into: {into}")
        }
        // let iter = list.iter();
    }
}
