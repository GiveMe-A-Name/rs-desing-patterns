//!
//! [Rust Design Patterns - New Type](https://fomalhauthmj.github.io/patterns/patterns/behavioural/newtype.html)
//!
//! # Description
//! 如果在某些情况下，我们希望一个类型的行为类似于另一个类型，或者在编译时强制执行一些行为，而仅仅使用类型别名是不够的
//! 例如，如果我们出于安全考虑（如密码），想为String创建一个自定义的Display实现。
//! 对于这种情况，我们可以使用 NewType 模式来提供类型安全和封装。
//!
//! 使用单个字段的元组结构体为一个类型做不透明包装。 这将创建一个新的类型。
//!
//! # Motivation
//! 主要是了更进一步的封装，抽象。有一个原则可以很形象的描述：高内聚。
//! 它允许你在类型之间精确控制接口。
//! 例如： 新类型可以用来区分单位，例如，包装f64以获得可区分的Miles和Kms。
//! # Example
//! example-1
//! ```ignore
//!    pub struct Foo(Bar<T1, T2>);    
//! ```
//! example-2
//! ```ignore
//!    pub Miles(i32);
//!    pub Kms(i32);
//!    
//!    impl from<Kms> for Miles {
//!         //..
//!    }
//! ```
