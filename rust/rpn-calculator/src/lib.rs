#[derive(Debug)]
pub enum CalculatorInput {
    Add,
    Subtract,
    Multiply,
    Divide,
    Value(i32),
}

pub fn evaluate(inputs: &[CalculatorInput]) -> Option<i32> {
    use CalculatorInput::*;
    let mut stack: Vec<CalculatorInput> = vec![];
    for input in inputs {
        match input {
            Add | Subtract | Multiply | Divide => {
                let second = match stack.pop() {
                    Some(Value(x)) => x,
                    _ => return None,
                };
                let first = match stack.pop() {
                    Some(Value(x)) => x,
                    _ => return None,
                };
                let val = match input {
                    Add => first + second,
                    Subtract => first - second,
                    Multiply => first * second,
                    Divide => first / second,
                    Value(_) => panic!(), //unreachable
                };
                stack.push(Value(val));
            },
            Value(x) => { stack.push(Value(*x)); }
        }
    }
    if stack.len() > 1 { return None }
    match stack.pop() {
        Some(Value(x)) => Some(x),
        _ => None,
    }
}
