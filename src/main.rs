fn main() {
    let exp = "6.1 5.2 4.3 * + 3.4 2.5 / 1.6 * -";

    let answer = rpn(exp);

    debug_assert_eq!(6.1 + 5.2 * 4.3 - 3.4 / 2.5 * 1.6, answer);
}

fn rpn(exp: &str) -> f64 {
    let mut stack = vec![];

    for token in exp.split_ascii_whitespace() {
        if let Ok(n) = token.parse::<f64>() {
            stack.push(n);
        } else {
            match token {
                "+" => apply2(&mut stack, |x, y| x + y),
                "-" => apply2(&mut stack, |x, y| x - y),
                "*" => apply2(&mut stack, |x, y| x * y),
                "/" => apply2(&mut stack, |x, y| x / y),
                _ => panic!("Unknown operator {}", token),
            }
        }
    }

    stack.pop().expect("Stack underflow")
}

fn apply2<F>(stack: &mut Vec<f64>, fun: F)
where
    F: Fn(f64, f64) -> f64,
{
    if let (Some(y), Some(x)) = (stack.pop(), stack.pop()) {
        let z = fun(x, y);
        stack.push(z);
    } else {
        panic!("Stack underflow");
    }
}
