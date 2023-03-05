//!
//! # Description
//! 在一小部分情况下，库作者可能想在不破坏后向兼容性的情况下，为公共结构体添加公共字段或为枚举添加新的变体。
//! Rust为这个问题提供了两种解决方案：
//!
//! - 在struct，enum和enum变体上使用#[non_exhaustive]。 关于所有可以使用#[non_exhaustive]的地方的详细文档，见文档。
//! - 你可以向结构体添加一个私有字段，以防止它被直接实例化或与之匹配（见备选方案）。

pub mod a {

    #[non_exhaustive]
    pub struct S {
        pub foo: i32,
    }

    pub fn print(s: S) {
        let S { foo: _ } = s;
    }

    #[non_exhaustive]
    pub enum AdmitMoreVariants {
        VariantA,
        VariantB,
        #[non_exhaustive]
        VariantC {
            a: String,
        },
    }
}

pub fn print_matched_variants(s: a::S) {
    // Because S is `#[non_exhaustive]`, it cannot be named here and
    // we must use `..` in the pattern.
    let a::S { foo: _ } = s;

    let some_enum = a::AdmitMoreVariants::VariantA;
    match some_enum {
        a::AdmitMoreVariants::VariantA => println!("it's an A"),
        a::AdmitMoreVariants::VariantB => println!("it's a b"),

        // .. required because this variant is non-exhaustive as well
        a::AdmitMoreVariants::VariantC { a: _a, .. } => println!("it's a c"),
        // The wildcard match is required because more variants may be
        // added in the future
        // _ => println!("it's a new variant"),
    }
}
