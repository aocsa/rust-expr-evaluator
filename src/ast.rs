use crate::error::EvalError;

#[derive(Debug, Clone)]
pub enum Expr {
    Number(f64),
    Add(Box<Expr>, Box<Expr>),
    Sub(Box<Expr>, Box<Expr>),
    Mul(Box<Expr>, Box<Expr>),
    Div(Box<Expr>, Box<Expr>),
    Neg(Box<Expr>),
    Pow(Box<Expr>, Box<Expr>),
}

impl Expr {
    pub fn number(n: f64) -> Expr {
        Expr::Number(n)
    }

    pub fn add(a: Expr, b: Expr) -> Expr {
        Expr::Add(Box::new(a), Box::new(b))
    }

    pub fn sub(a: Expr, b: Expr) -> Expr {
        Expr::Sub(Box::new(a), Box::new(b))
    }

    pub fn mul(a: Expr, b: Expr) -> Expr {
        Expr::Mul(Box::new(a), Box::new(b))
    }

    pub fn div(a: Expr, b: Expr) -> Expr {
        Expr::Div(Box::new(a), Box::new(b))
    }

    pub fn neg(a: Expr) -> Expr {
        Expr::Neg(Box::new(a))
    }

    pub fn pow(a: Expr, b: Expr) -> Expr {
        Expr::Pow(Box::new(a), Box::new(b))
    }

    pub fn eval(&self) -> Result<f64, EvalError> {
        match self {
            Expr::Number(n) => Ok(*n),
            Expr::Add(a, b) => {
                let result = a.eval()? + b.eval()?;
                Self::check_result(result)
            }
            Expr::Sub(a, b) => {
                let result = a.eval()? - b.eval()?;
                Self::check_result(result)
            }
            Expr::Mul(a, b) => {
                let result = a.eval()? * b.eval()?;
                Self::check_result(result)
            }
            Expr::Div(a, b) => {
                let divisor = b.eval()?;
                if divisor == 0.0 {
                    return Err(EvalError::DivisionByZero);
                }
                let result = a.eval()? / divisor;
                Self::check_result(result)
            }
            Expr::Neg(a) => {
                let result = -a.eval()?;
                Self::check_result(result)
            }
            Expr::Pow(a, b) => {
                let result = a.eval()?.powf(b.eval()?);
                Self::check_result(result)
            }
        }
    }

    fn check_result(result: f64) -> Result<f64, EvalError> {
        if result.is_infinite() {
            if result.is_sign_positive() {
                Err(EvalError::Overflow)
            } else {
                Err(EvalError::Underflow)
            }
        } else {
            Ok(result)
        }
    }

    pub fn to_string(&self) -> String {
        match self {
            Expr::Number(n) => n.to_string(),
            Expr::Add(a, b) => format!("({}) + ({})", a.to_string(), b.to_string()),
            Expr::Sub(a, b) => format!("({}) - ({})", a.to_string(), b.to_string()),
            Expr::Mul(a, b) => format!("({}) * ({})", a.to_string(), b.to_string()),
            Expr::Div(a, b) => format!("({}) / ({})", a.to_string(), b.to_string()),
            Expr::Neg(a) => format!("-({})", a.to_string()),
            Expr::Pow(a, b) => format!("({}) ^ ({})", a.to_string(), b.to_string()),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ast() {
        // 2 + 3 * 4
        let expr = Expr::add(
            Expr::number(2.0),
            Expr::mul(Expr::number(3.0), Expr::number(4.0)),
        );
        let result = expr.eval().expect("Evaluation failed");
        println!("{} = {}", expr.to_string(), result);
        assert_eq!(result, 14.0);
    }

    #[test]
    fn test_division_by_zero() {
        let expr = Expr::div(Expr::number(1.0), Expr::number(0.0));
        let result = expr.eval();
        assert!(matches!(result, Err(EvalError::DivisionByZero)));
    }

    #[test]
    fn test_overflow() {
        let expr = Expr::mul(Expr::number(f64::MAX), Expr::number(2.0));
        let result = expr.eval();
        assert!(matches!(result, Err(EvalError::Overflow)));
    }

    #[test]
    fn test_underflow() {
        let expr = Expr::mul(Expr::number(f64::MIN), Expr::number(2.0));
        let result = expr.eval();
        assert!(matches!(result, Err(EvalError::Underflow)));
    }
}
