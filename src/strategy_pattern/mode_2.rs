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
