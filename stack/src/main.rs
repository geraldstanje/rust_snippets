#![feature(box_syntax)]

struct Node<T> {
    data: T,
    next: Option<Box<Node<T>>>,
}

struct Stack<T> {
    head: Option<Box<Node<T>>>,
}

impl<T> Stack<T> {
    fn new() -> Stack<T> {
        Stack { head: None }
    }

    fn push(&mut self, elem: T) {
        self.head = Some(box Node { 
            data: elem, 
            next: self.head.take(), 
        });
    }
    
    fn pop(&mut self) -> Option<T> {
        match self.head.take() {
            None => None,
            Some(mut head) => {
                self.head = head.next.take();
                Some(head.data)
            }
        }
    }
}

fn print(input_num: Option<u32>) {
    let num = match input_num {
        Some(num) => num,
        None      => {
            println!("stack empty");
            return;
        }
    };
    
    println!("{}", num);
}

fn main() {
    let mut stack = Stack::new();
  
    stack.push(1);
    stack.push(2);
    stack.push(3);
    print(stack.pop());
    stack.push(4);
    print(stack.pop());
    print(stack.pop());
    stack.push(5);
    print(stack.pop());
    print(stack.pop());
    print(stack.pop());
}
