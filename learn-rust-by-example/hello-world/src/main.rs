struct CQueue {
    data: Vec<i32>, //定义一个slice，
    top: usize,     //栈中元素个数
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl CQueue {
    fn new() -> Self {
        //初始化一个栈，实现一个队列
        CQueue {
            data: Vec::with_capacity(10000),
            top: 0,
        }
    }

    //入栈
    fn append_tail(&mut self, value: i32) {
        //判断栈的容量和存量关系
        if self.top >= self.data.capacity() {
            return println!("there is no space to append new value");
        }
        //入栈
        self.data.push(value);
        self.top += 1;
    }

    fn delete_head(&mut self) -> i32 {
        if self.top == 0 {
            return -1;
        } else {
            self.top -= 1;
            self.data.pop();
            if self.top == 0 {
                return 0;
            } else {
                return self.data[0];
            }
        }
    }
}

fn main() {}
