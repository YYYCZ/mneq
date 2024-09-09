//! 处理神秘数字的主要文件

use std::{collections::HashSet, rc::Rc};

use ahash::RandomState;

use crate::expression::Expression;

/// 神秘数字处理中间结构体
pub(crate) struct MysteriousNumberProcess<'a> {
    /// 神秘数字原始序列
    mns: &'a [u8],
    /// 记录第 i 位数字到第 j 位数字组成的数字
    numbers: Vec<Vec<u64>>,
    /// 记录第 i 位数字到第 j 位数字对应的所有可构成及其对应值
    expressions: Vec<Vec<Vec<(Rc<Box<Expression>>, i64)>>>,
    /// 是否基于值判断表达式是否相等
    pub value_base: bool,
}

impl<'a> MysteriousNumberProcess<'a> {
    /// 提供原始序列
    pub fn new(mns: &'a [u8]) -> Self {
        MysteriousNumberProcess {
            mns,
            numbers: vec![vec![0u64; mns.len()]; mns.len()],
            expressions: vec![vec![Vec::new(); mns.len()]; mns.len()],
            value_base: true,
        }
    }

    /// 进行一步处理
    ///
    /// 其中，d 为 j 到 i 的距离，i 为原始序列的开始位置
    pub fn process_step(&mut self, d: usize, i: usize) {
        let j = i + d;

        // 不重复进行记录
        if !self.expressions[i][j].is_empty() {
            return;
        }

        // 记录数字
        let number = if d == 0 {
            self.mns[i] as u64
        } else {
            self.numbers[i][j - 1] * 10 + (self.mns[j] as u64)
        };
        self.numbers[i][j] = number;

        let mut expression = std::mem::take(&mut self.expressions[i][j]);

        // 不记录重复值
        let mut value_set: HashSet<i64, RandomState> = HashSet::default();

        // 若首位数字不为 0
        // 或者数字为个位数
        // 则数字构成表达式
        if self.mns[i] != 0 || d == 0 {
            expression.push((Rc::new(Box::new(Expression::Number(number))), number as i64));
            if self.value_base {
                value_set.insert(number as i64);
            }
        }

        // 左右两侧表达式通过加、减、乘构成表达式
        for k in i..j {
            for (left_expression, left_value) in &self.expressions[i][k] {
                for (right_expression, right_value) in &self.expressions[k + 1][j] {
                    if !self.value_base || !value_set.contains(&(left_value + right_value)) {
                        expression.push((
                            Rc::new(Box::new(Expression::Add(
                                left_expression.clone(),
                                right_expression.clone(),
                            ))),
                            left_value + right_value,
                        ));
                        if self.value_base {
                            value_set.insert(left_value + right_value);
                        }
                    }

                    if !self.value_base || !value_set.contains(&(left_value - right_value)) {
                        expression.push((
                            Rc::new(Box::new(Expression::Subtract(
                                left_expression.clone(),
                                right_expression.clone(),
                            ))),
                            left_value - right_value,
                        ));
                        if self.value_base {
                            value_set.insert(left_value - right_value);
                        }
                    }

                    if !self.value_base || !value_set.contains(&(left_value * right_value)) {
                        expression.push((
                            Rc::new(Box::new(Expression::Multiply(
                                left_expression.clone(),
                                right_expression.clone(),
                            ))),
                            left_value * right_value,
                        ));
                        if self.value_base {
                            value_set.insert(left_value * right_value);
                        }
                    }
                }
            }
        }

        let _ = std::mem::replace(&mut self.expressions[i][j], expression);
    }

    /// 对一种距离为 d 的步骤进行全部处理
    pub fn process_dist(&mut self, d: usize) {
        for i in 0..self.mns.len() - d {
            self.process_step(d, i);
        }
    }

    /// 全部进行处理
    #[allow(dead_code)]
    pub fn process_all(&mut self) {
        for d in 0..self.mns.len() {
            self.process_dist(d)
        }
    }

    /// 获取结果表达式
    pub fn get_expressions(&self) -> &Vec<Vec<Vec<(Rc<Box<Expression>>, i64)>>> {
        &self.expressions
    }
}
