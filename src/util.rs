use std::collections::HashMap;

pub fn helper(x: i32) -> i32 {
    x * 2
}

pub fn tally(values: &[i32]) -> HashMap<String, i32> {
    let mut out = HashMap::new();
    out.insert("sum".to_string(), values.iter().sum());
    out
}

pub struct Counter {
    pub count: i32,
}

impl Counter {
    pub fn new() -> Self {
        Counter { count: 0 }
    }

    pub fn bump(&mut self, by: i32) -> i32 {
        self.count += by;
        self.count
    }
}
