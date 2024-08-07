pub type Value = f64;

#[derive(Clone)]
pub struct ValueArray {
    pub values: Vec<Value>,
}

impl ValueArray {
    pub fn new() -> ValueArray {
        ValueArray { values: Vec::new() }
    }

    pub fn write(&mut self, val: Value) {
        self.values.push(val);
    }
}

pub fn print_value(val: Option<&Value>) {
    match val {
        Some(val) => print!("{}", val),
        None => print!("nil"),
    }
}
