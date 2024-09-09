use std::rc::Rc;

/// 表达式
#[derive(Debug, Clone)]
pub(crate) enum Expression {
    /// 数字
    Number(u64),
    /// 加法
    Add(Rc<Box<Expression>>, Rc<Box<Expression>>),
    /// 减法
    Subtract(Rc<Box<Expression>>, Rc<Box<Expression>>),
    /// 乘法
    Multiply(Rc<Box<Expression>>, Rc<Box<Expression>>),
}

impl Expression {
    /// 判断是否为加法或减法
    fn is_add_or_subtract(&self) -> bool {
        match self {
            Expression::Add(..) | Expression::Subtract(..) => true,
            _ => false,
        }
    }
}

impl ToString for Expression {
    fn to_string(&self) -> String {
        match self {
            Expression::Number(num) => num.to_string(),
            Expression::Add(left, right) => {
                format!("{}+{}", left.to_string(), right.to_string())
            }
            Expression::Subtract(left, right) => {
                let left_str = left.to_string();
                let right_str = right.to_string();
                if right.is_add_or_subtract() {
                    format!("{}-({})", left_str, right_str)
                } else {
                    format!("{}-{}", left_str, right_str)
                }
            }
            Expression::Multiply(left, right) => {
                let left_str = left.to_string();
                let right_str = right.to_string();
                let left_bracket = left.is_add_or_subtract();
                let right_bracket = right.is_add_or_subtract();
                if left_bracket && right_bracket {
                    format!("({})×({})", left_str, right_str)
                } else if left_bracket {
                    format!("({})×{}", left_str, right_str)
                } else if right_bracket {
                    format!("{}×({})", left_str, right_str)
                } else {
                    format!("{}×{}", left_str, right_str)
                }
            }
        }
    }
}
