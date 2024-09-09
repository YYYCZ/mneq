use std::{collections::HashMap, rc::Rc};

use ahash::RandomState;

use crate::expression::Expression;
use crate::mn::MysteriousNumberProcess;

/// 提供一串神秘数字，发现其中能构造的一组等式，无解时返回 None
pub fn find_mneq(mns: &[u8]) -> Option<String> {
    // 初始化工作
    let mut mnp = MysteriousNumberProcess::new(&mns);
    let l_half_len = mns.len() / 2;
    let r_half_len = l_half_len + mns.len() % 2;

    // 处理半长度以下的所有 d 值情况
    if r_half_len > 2 {
        for d in 0..r_half_len - 2 {
            mnp.process_dist(d);
        }
    }

    // 遍历所有等号可能的位置，判断左右两边的表达式是否相等
    // 从中间处开始分割，然后向两边进行延申
    for t in 0..l_half_len {
        // 左侧位置
        let left_point = l_half_len - 1 - t;
        // 右侧位置
        let right_point = r_half_len - 1 + t;

        // 处理上一层 d 值
        if r_half_len >= 2 {
            mnp.process_dist(r_half_len - 2 + t);
        }

        for k in if left_point == right_point {
            [left_point, 128]
        } else {
            [left_point, right_point]
        } {
            // 128 特殊值不处理
            if k == 128 {
                continue;
            }

            // 处理 d = k, i = 0 以及 d = mns.len() - k - 2, i = k + 1 的情况
            mnp.process_step(k, 0);
            mnp.process_step(mns.len() - k - 2, k + 1);

            let left_expressions = &mnp.get_expressions()[0][k];
            let right_expressions = &mnp.get_expressions()[k + 1][mns.len() - 1];

            // 记录下左侧的表达式的值以及对应的表达式
            let left_values: HashMap<i64, Rc<Box<Expression>>, RandomState> = left_expressions
                .iter()
                .fold(HashMap::default(), |mut acc, (expression, value)| {
                    acc.insert(*value, expression.clone());
                    acc
                });

            // 遍历右侧的表达式，寻找值相等的左侧的表达式
            for (right_expression, right_value) in right_expressions {
                if let Some(left_expression) = left_values.get(right_value) {
                    return Some(format!(
                        "{}={}",
                        left_expression.to_string(),
                        right_expression.to_string()
                    ));
                }
            }
        }
    }

    None
}
