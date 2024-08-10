fn main() {
    Ops::Add.describe();
    match Ops::Add.execute(10, 5) {
        Ok(value) => println!("Result: {}", value),
        Err(CalcError::DivisionByZero) => println!("Error: Division by zero is not allowed"),
        Err(CalcError::InvalidOperation) => println!("Error: Invalid operation"),
    }
    Ops::Subtract.describe();
    match Ops::Subtract.execute(10, 5) {
        Ok(value) => println!("Result: {}", value),
        Err(CalcError::DivisionByZero) => println!("Error: Division by zero is not allowed"),
        Err(CalcError::InvalidOperation) => println!("Error: Invalid operation"),
    }
    Ops::Multiply.describe();
    match Ops::Multiply.execute(10, 5) {
        Ok(value) => println!("Result: {}", value),
        Err(CalcError::DivisionByZero) => println!("Error: Division by zero is not allowed"),
        Err(CalcError::InvalidOperation) => println!("Error: Invalid operation"),
    }
    Ops::Divide.describe();
    match Ops::Divide.execute(10, 5) {
        Ok(value) => println!("Result: {}", value),
        Err(CalcError::DivisionByZero) => println!("Error: Division by zero is not allowed"),
        Err(CalcError::InvalidOperation) => println!("Error: Invalid operation"),
    }
    println!("Show error is handled");
    match Ops::Divide.execute(5, 0) {
        Ok(value) => println!("Result: {}", value),
        Err(CalcError::DivisionByZero) => println!("Error: Division by zero is not allowed"),
        Err(CalcError::InvalidOperation) => println!("Error: Invalid operation"),
    }
}

enum Ops {
    Add,
    Subtract,
    Multiply,
    Divide,
}
// custom errors def here instead of passing e to be printed out or Err(())=> etc
enum CalcError {
    DivisionByZero,
    InvalidOperation,
}

trait Operation {
    fn execute(&self, x: i32, y: i32) -> Result<i32, CalcError>;
    fn describe(&self);
}

impl Operation for Ops {
    fn execute(&self, x: i32, y: i32) -> Result<i32, CalcError> {
        match self {
            Self::Add => Ok(x + y),
            Self::Subtract => Ok(x - y),
            Self::Multiply => Ok(x * y),
            Self::Divide => {
                if y == 0 {
                    Err(CalcError::DivisionByZero)
                } else {
                    Ok(x / y)
                }
            }
            _ => Err(CalcError::InvalidOperation),
        }
    }
    fn describe(&self) {
        match self {
            Self::Add => println!("Adds two numbers together"),
            Self::Subtract => println!("subtracts one number from the other"),
            Self::Multiply => println!("times one number by the other"),
            Self::Divide => println!("divides one number by another"),
        }
    }
}
