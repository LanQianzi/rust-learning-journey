use std::alloc::{self, Layout};
use std::ptr::{self, NonNull};
pub struct Queue<T> {
    data: NonNull<T>,
    len: usize,
    cap: usize,
    head: usize,
    tail: usize,
}

impl<T> Queue<T> {
    pub fn new(cap: usize) -> Self {
        assert!(std::mem::size_of::<T>() != 0, "暂不支持零大小类型");
        let layout = Layout::array::<T>(cap).unwrap();
        let ptr = unsafe { alloc::alloc(layout) as *mut T };
        let data = NonNull::new(ptr).unwrap_or_else(|| alloc::handle_alloc_error(layout));
        Self {
            data,
            len: 0,
            cap,
            head: 0,
            tail: 0,
        }
    }

    pub fn len(&self) -> usize {
        self.len
    }

    pub fn cap(&self) -> usize {
        self.cap
    }

    pub fn is_empty(&self) -> bool {
        self.len == 0
    }

    pub fn front(&self) -> Option<&T> {
        if self.is_empty() {
            None
        } else {
            unsafe { Some(&*self.data.as_ptr().add(self.head)) }
        }
    }

    pub fn front_mut(&mut self) -> Option<&mut T> {
        if self.is_empty() {
            None
        } else {
            unsafe { Some(&mut *self.data.as_ptr().add(self.head)) }
        }
    }

    pub fn push(&mut self, val: T) -> Result<(), String> {
        if self.len == self.cap {
            return Err("队列已经满了".to_string());
        }

        unsafe {
            self.data.as_ptr().add(self.tail).write(val);
        }
        self.tail += 1;
        self.len += 1;

        if self.tail == self.cap {
            self.tail = 0;
        }

        Ok(())
    }

    pub fn pop(&mut self) -> Option<T> {
        if self.len == 0 {
            return None;
        }

        let result;
        unsafe {
            result = self.data.as_ptr().add(self.head).read();
        }
        self.head += 1;
        self.len -= 1;
        if self.head == self.cap {
            self.head = 0
        }
        Some(result)
    }
}

impl<T> Drop for Queue<T> {
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

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_queue() {
        let mut que = Queue::new(3);
        assert_eq!(que.push(1), Ok(()));
        assert_eq!(que.push(2), Ok(()));
        assert_eq!(que.push(3), Ok(()));
        assert_eq!(que.push(4), Err("队列已经满了".to_string()));

        assert_eq!(que.front(), Some(&1));
        assert_eq!(que.pop(), Some(1));
        assert_eq!(que.front(), Some(&2));
        assert_eq!(que.pop(), Some(2));

        assert_eq!(que.push(4), Ok(()));
        assert_eq!(que.push(5), Ok(()));
        assert_eq!(que.pop(), Some(3));
        assert_eq!(que.pop(), Some(4));
        assert_eq!(que.pop(), Some(5));
    }
}
