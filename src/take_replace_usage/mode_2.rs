//!
//! # Motivation
//! mem::take可以换掉这个值，用它的默认值代替，并返回之前的值。
//! 对于String，默认值是一个空的String，不需要分配内存。
//! mem::replace非常相似，但允许我们指定用什么来替换值。 mem::take等价于mem::replace(name, String::new()).

use std::mem;

#[derive(Debug, PartialEq, Eq)]
enum MultiVariateEnum {
    A { name: String },
    B { name: String },
    C,
    D,
}

fn swizzle(e: &mut MultiVariateEnum) {
    use MultiVariateEnum::*;

    *e = match e {
        A { name } => B {
            // Replaces dest with the `default` value of T, returning the previous dest value.
            name: mem::take(name),
        },
        B { name } => A {
            // Moves src into the referenced dest, returning the previous dest value.
            name: mem::replace(name, String::new()),
        },
        C => D,
        D => C,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        let mut a = MultiVariateEnum::A {
            name: "hello".into(),
        };

        swizzle(&mut a);
        assert_eq!(
            a,
            MultiVariateEnum::B {
                name: "hello".into()
            }
        );

        let mut c = MultiVariateEnum::C;
        swizzle(&mut c);
        assert_eq!(c, MultiVariateEnum::D);
    }
}
