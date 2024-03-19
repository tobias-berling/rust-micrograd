use std::cell::RefCell;
use std::fmt;
use std::ops::Add;
use std::ops::Deref;
use std::ops::Mul;
use std::rc::Rc;

enum Op {
    None,
    Add,
    Mul,
}

struct ValueData {
    data: f64,
    _children: Vec<Value>,
    label: String,
    op: Op,
}

struct Value {
    data: Rc<RefCell<ValueData>>,
}

impl Value {
    fn new(data: f64) -> Value {
        Value {
            data: Rc::new(RefCell::new(ValueData {
                data,
                label: String::new(),
                _children: Vec::new(),
                op: Op::None,
            })),
        }
    }
    fn new_with_label(data: f64, label: &str) -> Value {
        let v = Value::new(data);
        v.set_label(label);
        return v;
    }

    fn set_label(&self, label: &str) {
        self.data.borrow_mut().label = label.to_string();
    }
}

impl Add for Value {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        let v1 = self.data.deref().borrow().data;
        let v2 = other.data.deref().borrow().data;
        let v = Value::new(v1 + v2);
        {
            let mut v_data = v.data.borrow_mut();
            v_data._children.push(self);
            v_data._children.push(other);
            v_data.op = Op::Add
        }

        return v;
    }
}

impl Mul for Value {
    type Output = Self;

    fn mul(self, other: Self) -> Self {
        let v1 = self.data.deref().borrow().data;
        let v2 = other.data.deref().borrow().data;
        let v = Value::new(v1 * v2);
        {
            let mut v_data = v.data.borrow_mut();
            v_data._children.push(self);
            v_data._children.push(other);
            v_data.op = Op::Mul
        }
        return v;
    }
}

impl fmt::Display for Op {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Op::None => write!(f, "Op(=)"),
            Op::Add => write!(f, "Op(+)"),
            Op::Mul => write!(f, "Op(*)"),
        }
    }

}

impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let v_data = self.data.deref().borrow();
        write!(
            f,
            "Value(data={} label={} op={})",
            v_data.data, v_data.label, v_data.op
        )
    }
}

fn print_graph(v: &Value, level: i32) {
    for _ in 0..level {
        print!("  ")
    }
    println!("{}", v);
    for c in &v.data.deref().borrow()._children {
        print_graph(c, level + 1);
    }
}

fn main() {
    println!("Hello, world!");
    let a = Value::new_with_label(4.0, "a");
    let b = Value::new_with_label(2.0, "b");
    let x = Value::new_with_label(10.0, "x");
    let c = a * x + b;
    c.set_label("c");

    println!("a = {}", c);

    print_graph(&c, 0)
}
