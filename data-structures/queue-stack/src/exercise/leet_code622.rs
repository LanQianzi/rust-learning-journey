/// 622. 设计循环队列

/// 设计你的循环队列实现。 循环队列是一种线性数据结构，其操作表现基于 FIFO（先进先出）原则并且队尾被连接在队首之后以形成一个循环。
/// 它也被称为“环形缓冲器”。
/// 循环队列的一个好处是我们可以利用这个队列之前用过的空间。
/// 在一个普通队列里，一旦一个队列满了，我们就不能插入下一个元素，即使在队列前面仍有空间。
/// 但是使用循环队列，我们能使用这些空间去存储新的值。

/// 你的实现应该支持如下操作：
/// Self::new(k): 构造函数，设置队列长度为 k 。
/// front: 从队首获取元素。
/// rear: 获取队尾元素。
/// en_queue(value): 向循环队列插入一个元素。如果成功插入则返回真。
/// de_queue(): 从循环队列中删除一个元素。如果成功删除则返回真。
/// is_empty(): 检查循环队列是否为空。
/// is_full(): 检查循环队列是否已满。

pub struct MyCircularQueue {
    data: Vec<i32>,
    len: usize,
    cap: usize,
    head: usize,
    tail: usize,
}

impl MyCircularQueue {
    pub fn new(k: i32) -> Self {
        Self {
            data: vec![0; k as usize],
            len: 0,
            cap: k as usize,
            head: 0,
            tail: 0,
        }
    }

    pub fn en_queue(&mut self, value: i32) -> bool {
        if self.len == self.cap {
            return false;
        }

        self.data[self.tail] = value;
        self.len += 1;
        self.tail = (self.tail + 1) % self.cap;
        true
    }

    pub fn de_queue(&mut self) -> bool {
        if self.len == 0 {
            return false;
        }

        self.len -= 1;
        self.head = (self.head + 1) % self.cap;
        true
    }

    pub fn front(&self) -> i32 {
        if self.len == 0 {
            -1
        } else {
            self.data[self.head]
        }
    }

    pub fn rear(&self) -> i32 {
        if self.len == 0 {
            return -1;
        } else {
            self.data[(self.tail - 1 + self.cap) % self.cap]
        }
    }

    pub fn is_empty(&self) -> bool {
        self.len == 0
    }

    pub fn is_full(&self) -> bool {
        self.len == self.cap
    }
}
