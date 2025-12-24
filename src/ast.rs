
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
    pub fn add(a:Expr, b:Expr) -> Expr {
        Expr::Add(Box::new(a), Box::new(b))
    }
    pub fn sub(a:Expr, b:Expr) -> Expr {
        Expr::Sub(Box::new(a), Box::new(b))
    }
    pub fn mul(a:Expr, b:Expr) -> Expr {
        Expr::Mul(Box::new(a), Box::new(b))
    }
    pub fn div(a:Expr, b:Expr) -> Expr {
        Expr::Div(Box::new(a), Box::new(b))
    }
    pub fn neg(a:Expr) -> Expr {
        Expr::Neg(Box::new(a))
    }
    
    pub fn pow(a:Expr, b:Expr) -> Expr {
        Expr::Pow(Box::new(a), Box::new(b))
    }
    
    pub fn eval(&self) -> f64 {
        match self {
            Expr::Number(n) => *n,
            Expr::Add(a, b) => a.eval() + b.eval(),
            Expr::Sub(a, b) => a.eval() - b.eval(),
            Expr::Mul(a, b) => a.eval() * b.eval(),
            Expr::Div(a, b) => a.eval() / b.eval(),
            Expr::Neg(a) => -a.eval(),
            Expr::Pow(a, b) => a.eval().powf(b.eval()),
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
        // x + 3 * 4
        let expr = Expr::add(Expr::number(2.0), Expr::mul(Expr::number(3.0), Expr::number(4.0)));
        println!("{} = {}", expr.to_string(), expr.eval());
    }
}
