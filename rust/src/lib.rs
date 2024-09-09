use wasm_bindgen::prelude::*;

use crate::find_mneq::find_mneq;

pub(crate) mod expression;
pub(crate) mod find_mneq;
pub(crate) mod mn;

/// 寻找神秘数字构成的等式
///
/// 参数：
///
/// - mns: 神秘数字序列
///
/// 返回值：
///
/// 存在返回则返回等式字符串，其他情况返回对应错误代码
///
/// 错误代码：
/// - 0: 无解
/// - 1: 序列长度过长（长度大于等于 128）
/// - 2: 字符串中存在非数字字符
#[wasm_bindgen]
#[allow(non_snake_case)]
pub fn findMysteriousNumberEquation(mns: &str) -> Result<String, i32> {
    // 输入数字的长度大于 1 才有解
    // 并且长度不能大于或等于 128
    if mns.len() <= 1 {
        return Err(0);
    } else if mns.len() >= 128 {
        return Err(1);
    }

    // 字符串转化为数字数组
    // 存在非数字字符时返回错误码
    let mns: Vec<u8> = mns
        .chars()
        .map(|c| c.to_digit(10).map(|e| e as u8).ok_or(2))
        .collect::<Result<_, _>>()?;

    find_mneq(&mns).ok_or(0)
}

#[cfg(test)]
mod tests {
    use mn::MysteriousNumberProcess;

    use super::*;

    /// 提供一串数字，获取能构成的所有表达式
    #[test]
    fn test_get_expressions() {
        let mns = [1, 1, 4];
        let mut mnp = MysteriousNumberProcess::new(&mns);

        mnp.process_all();

        let expressions = &mnp.get_expressions()[0][2];
        for (expression, value) in expressions {
            println!("{}={}", expression.to_string(), value);
        }
    }

    /// 寻找神秘数字所能构成的等式
    #[test]
    fn test_find_mneq() {
        let mns = [1, 1, 4, 5, 1, 4, 1, 9, 1, 9, 8, 1, 0];
        if let Some(eq) = find_mneq(&mns) {
            println!("{}", eq);
        } else {
            println!(
                "神秘数字 {} 无法构成等式",
                mns.iter().map(|e| char::from(b'0' + e)).collect::<String>()
            );
        }
    }
}
