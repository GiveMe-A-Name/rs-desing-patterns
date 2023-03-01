//! [Rust Design Patterns](https://fomalhauthmj.github.io/patterns/patterns/behavioural/strategy.html)
//!
//! # Description
//! 策略模式的基本思想是，给定一个解决特定问题的算法，我们只在抽象层面上定义算法的骨架，并将具体的算法实现分成不同的部分。
//! 实际上可以理解为活用 trait 和 generic
//!
//! # Motivation
//! 想象一下，我们正在做一个每月都会生成报告的项目。 我们需要以不同的格式（策略）生成报告，例如，以JSON或Plain Text格式。

use std::collections::HashMap;

type Data = HashMap<String, u32>;

pub trait Formatter {
    fn format(&self, data: &Data, buf: &mut String);
}

pub struct Report;

impl Report {
    pub fn generate<T: Formatter>(g: T, s: &mut String) {
        let mut data: Data = HashMap::new();

        data.insert("one".into(), 1);
        data.insert("two".into(), 2);

        g.format(&data, s);
    }
}

struct Text;
impl Formatter for Text {
    fn format(&self, data: &Data, buf: &mut String) {
        for (k, v) in data {
            let entry = format!("{} {}\n", k, v);
            buf.push_str(&entry);
        }
    }
}

struct Json;
impl Formatter for Json {
    fn format(&self, data: &Data, buf: &mut String) {
        buf.push('[');
        for (k, v) in data.into_iter() {
            let entry = format!(r#"{{"{}":"{}"}}"#, k, v);
            buf.push_str(&entry);
            buf.push(',');
        }
        buf.pop(); // remove extra , at the end
        buf.push(']');
    }
}

#[cfg(test)]
mod tests {

    use super::{Json, Report, Text};

    #[test]
    fn use_strategy_pattern() {
        let mut s = String::from("");
        Report::generate(Text, &mut s);
        assert!(s.contains("one 1"));
        assert!(s.contains("two 2"));

        s.clear();
        Report::generate(Json, &mut s);
        assert!(s.contains(r#"{"one":"1"}"#));
        assert!(s.contains(r#"{"two":"2"}"#));
    }
}
