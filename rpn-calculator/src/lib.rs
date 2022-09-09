use CalculatorInput::*;

#[derive(Debug)]
pub enum CalculatorInput {
    Add,
    Subtract,
    Multiply,
    Divide,
    Value(i32),
}

pub fn evaluate(inputs: &[CalculatorInput]) -> Option<i32> {
    let mut stack: Vec<i32> = Vec::new();

    for input in inputs {
        match input {
            Add => {
                let (a, b) = get_last_two(&mut stack)?;
                stack.push(b + a)
            }
            Subtract => {
                let (a, b) = get_last_two(&mut stack)?;
                stack.push(b - a)
            }
            Multiply => {
                let (a, b) = get_last_two(&mut stack)?;
                stack.push(b * a)
            }
            Divide => {
                let (a, b) = get_last_two(&mut stack)?;
                stack.push(b / a)
            }
            Value(n) => {
                stack.push(*n);
            }
        }
    }

    if stack.len() == 1 { stack.pop() } else { None }
}

fn get_last_two(stack: &mut Vec<i32>) -> Option<(i32, i32)> {
    let a = stack.pop();
    let b = stack.pop();

    if a.is_none() || b.is_none() {
        return None;
    }

    Some((a.unwrap(), b.unwrap()))
}
