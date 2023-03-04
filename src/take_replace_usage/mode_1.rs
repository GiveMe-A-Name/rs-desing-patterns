//! # Description
//! 假定我们有一个&mut MyEnum，它有（至少）两个变体， A { name: String, x: u8 }和B { name: String }。
//! 现在我们想如果x为零，把MyEnum::A改成B，同时保持MyEnum::B不变。
//!
use std::mem;

#[derive(PartialEq, Eq, Debug)]
pub enum MyEnum {
    A { name: String, x: u8 },
    B { name: String },
}

pub fn a_to_b(e: &mut MyEnum) {
    if let MyEnum::A { name, x: 0 } = e {
        *e = MyEnum::B {
            name: mem::take(name),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn use_take_and_replace() {
        let mut a = MyEnum::A {
            name: "hello".into(),
            x: 0,
        };

        a_to_b(&mut a);
        assert_eq!(
            a,
            MyEnum::B {
                name: "hello".into()
            }
        )
    }
}
