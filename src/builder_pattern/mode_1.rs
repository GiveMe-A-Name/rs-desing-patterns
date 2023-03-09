#[derive(Debug, PartialEq)]
pub struct Foo {
    // Lots of complicated fields.
    pub bar: String,
}

impl Foo {
    pub fn builder() -> FooBuilder {
        // 这里通过 default 的方式让 FooBuilder 的字段都默认
        // 其实在后面 FooBuilder 创建 Foo 的时候也赋予默认值
        FooBuilder::default()
    }
}

#[derive(Debug, Default, PartialEq)]
pub struct FooBuilder {
    // Probably lost of optional fields
    pub(crate) bar: String,
}

impl FooBuilder {
    pub fn name(mut self, bar: String) -> FooBuilder {
        // Set the name on the builder itself, and return the builder by value.
        self.bar = bar;
        self
    }

    pub fn build(self) -> Foo {
        let Self { bar } = self;
        Foo { bar }
    }
}

#[cfg(test)]
pub(crate) mod tests {
    use super::*;
    #[test]
    fn test() {
        let foo1 = Foo {
            bar: "hello1".into(),
        };
        let foo2 = Foo::builder().name("hello1".into()).build();

        assert_eq!(foo1, foo2);
    }
}
