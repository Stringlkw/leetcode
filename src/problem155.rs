pub struct MinStack {
    stack: Vec<Pair>,
}

struct Pair {
    val: i32,
    min_val: i32,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl MinStack {
    pub fn new() -> Self {
        MinStack { stack: Vec::new() }
    }

    pub fn push(&mut self, val: i32) {
        if self.stack.is_empty() {
            self.stack.push(Pair { val, min_val: val });
        } else {
            let min = std::cmp::min(val, self.stack.last().unwrap().min_val);
            self.stack.push(Pair { val, min_val: min });
        }
    }

    pub fn pop(&mut self) {
        self.stack.pop();
    }

    pub fn top(&self) -> i32 {
        self.stack.last().unwrap().val
    }

    pub fn get_min(&self) -> i32 {
        self.stack.last().unwrap().min_val
    }
}
