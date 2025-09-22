use std::alloc::{self, Layout};
use std::marker::PhantomData;
use std::ptr::{self, NonNull};

pub struct Table<T> {
    cap: usize,
    len: usize,
    data: NonNull<T>,
}

pub struct Iter<'a, T> {
    ptr: *const T,
    end: *const T,
    _marker: PhantomData<&'a T>,
}

pub struct IterMut<'a, T> {
    ptr: *mut T,
    end: *mut T,
    _marker: PhantomData<&'a mut T>,
}

pub struct IntoIter<T> {
    ptr: *mut T,
    end: *mut T,
}

impl<T> Table<T> {
    pub fn new() -> Self {
        assert!(std::mem::size_of::<T>() != 0, "暂不支持零大小类型");
        Table {
            cap: 0,
            len: 0,
            data: NonNull::dangling(),
        }
    }

    pub fn len(&self) -> usize {
        self.len
    }

    pub fn is_empty(&self) -> bool {
        self.len == 0
    }

    pub fn capacity(&self) -> usize {
        self.cap
    }

    pub fn get(&self, index: usize) -> Option<&T> {
        if index < self.len {
            unsafe { Some(&*self.data.as_ptr().add(index)) }
        } else {
            None
        }
    }

    pub fn get_mut(&mut self, index: usize) -> Option<&mut T> {
        if index < self.len {
            unsafe { Some(&mut *self.data.as_ptr().add(index)) }
        } else {
            None
        }
    }

    fn grow(&mut self) {
        let new_cap = if self.cap == 0 { 4 } else { self.cap * 2 };
        let new_layout = Layout::array::<T>(new_cap).unwrap();
        let new_ptr = if self.cap == 0 {
            unsafe { alloc::alloc(new_layout) as *mut T }
        } else {
            let old_layout = Layout::array::<T>(self.cap).unwrap();
            unsafe {
                alloc::realloc(self.data.as_ptr() as *mut u8, old_layout, new_layout.size())
                    as *mut T
            }
        };
        self.data = NonNull::new(new_ptr).unwrap_or_else(|| alloc::handle_alloc_error(new_layout));
        self.cap = new_cap;
    }

    pub fn push_back(&mut self, v: T) {
        if self.len == self.cap {
            self.grow();
        }

        unsafe {
            self.data.as_ptr().add(self.len).write(v);
        }
        self.len += 1
    }

    pub fn pop_back(&mut self) -> Option<T> {
        if self.len == 0 {
            return None;
        }
        self.len -= 1;
        unsafe { Some(self.data.as_ptr().add(self.len).read()) }
    }

    pub fn insert(&mut self, idx: usize, v: T) {
        assert!(idx <= self.len, "下标越界");
        unsafe {
            let p = self.data.as_ptr().add(idx);
            ptr::copy(p, p.add(1), self.len - idx);
            p.write(v);
        }
        self.len += 1
    }

    pub fn remove(&mut self, idx: usize) -> T {
        assert!(idx < self.len, "下标越界");
        unsafe {
            let p = self.data.as_ptr().add(idx);
            let val = p.read();
            ptr::copy(p.add(1), p, self.len - idx - 1);
            self.len -= 1;
            val
        }
    }
}

/// Drop 手动释放堆内存
impl<T> Drop for Table<T> {
    fn drop(&mut self) {
        if self.cap != 0 {
            unsafe {
                ptr::drop_in_place(ptr::slice_from_raw_parts_mut(self.data.as_ptr(), self.len));
                let layout = Layout::array::<T>(self.cap).unwrap();
                alloc::dealloc(self.data.as_ptr() as *mut u8, layout);
            }
        }
    }
}

impl<T> Default for Table<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T> Table<T> {
    pub fn iter(&self) -> Iter<'_, T> {
        unsafe {
            Iter {
                ptr: self.data.as_ptr(),
                end: self.data.as_ptr().add(self.len),
                _marker: PhantomData,
            }
        }
    }
}

impl<'a, T> IntoIterator for &'a Table<T> {
    type Item = &'a T;
    type IntoIter = Iter<'a, T>;
    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

impl<'a, T> Iterator for Iter<'a, T> {
    type Item = &'a T;
    fn next(&mut self) -> Option<Self::Item> {
        if self.ptr == self.end {
            None
        } else {
            unsafe {
                let ret = &*self.ptr;
                self.ptr = self.ptr.add(1);
                Some(ret)
            }
        }
    }
}

impl<T> Table<T> {
    pub fn iter_mut(&mut self) -> IterMut<'_, T> {
        IterMut {
            ptr: self.data.as_ptr(),
            end: unsafe { self.data.as_ptr().add(self.len) },
            _marker: PhantomData,
        }
    }
}

impl<'a, T> IntoIterator for &'a mut Table<T> {
    type IntoIter = IterMut<'a, T>;
    type Item = &'a mut T;
    fn into_iter(self) -> Self::IntoIter {
        self.iter_mut()
    }
}

impl<'a, T> Iterator for IterMut<'a, T> {
    type Item = &'a mut T;
    fn next(&mut self) -> Option<Self::Item> {
        if self.ptr == self.end {
            None
        } else {
            unsafe {
                let ret = &mut *self.ptr;
                self.ptr = self.ptr.add(1);
                Some(ret)
            }
        }
    }
}

impl<T> IntoIterator for Table<T> {
    type Item = T;
    type IntoIter = IntoIter<T>;
    fn into_iter(self) -> Self::IntoIter {
        let ptr = self.data.as_ptr();
        let len = self.len;
        std::mem::forget(self); // 自己管内存，防止二次 drop
        IntoIter {
            ptr,
            end: unsafe { ptr.add(len) },
        }
    }
}

impl<T> Iterator for IntoIter<T> {
    type Item = T;
    fn next(&mut self) -> Option<Self::Item> {
        if self.ptr == self.end {
            None
        } else {
            unsafe {
                let ret = self.ptr.read();
                self.ptr = self.ptr.add(1);
                Some(ret)
            }
        }
    }
}

impl<T> Drop for IntoIter<T> {
    fn drop(&mut self) {
        unsafe {
            let len = self.end.offset_from(self.ptr) as usize;
            ptr::drop_in_place(ptr::slice_from_raw_parts_mut(self.ptr, len));
            let layout = Layout::array::<T>(len).unwrap();
            alloc::dealloc(self.ptr as *mut u8, layout);
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_table() {
        let mut table = Table::new();
        table.push_back(1);
        table.push_back(2);
        table.push_back(3);
        table.insert(1, -2);
        assert_eq!(table.get(1), Some(&-2));
        assert_eq!(table.get(2), Some(&2));
        assert_eq!(table.remove(1), -2);
        assert_eq!(table.get(1), Some(&2));
    }

    #[test]
    fn test_iter() {
        let mut table = Table::new();
        table.push_back(1);
        table.push_back(2);
        table.push_back(3);
        table.insert(1, -2);

        print!("V: ");
        for v in &table {
            print!("{v} ");
        }
        println!();

        for v in &mut table {
            *v = *v + 1;
        }

        print!("V_add: ");
        for v in table {
            print!("{v} ");
        }
        println!();

        // for v in table {
        //     print!("{v} ");
        // }
    }
}
