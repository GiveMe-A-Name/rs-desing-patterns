//! [Rust Design Patterns](https://fomalhauthmj.github.io/patterns/patterns/behavioural/strategy.html)
//!
//! # Description
//! 策略模式的基本思想是，给定一个解决特定问题的算法，我们只在抽象层面上定义算法的骨架，并将具体的算法实现分成不同的部分。
//! 实际上可以理解为活用 trait 和 generic
//!
//! # Motivation
//! 想象一下，我们正在做一个每月都会生成报告的项目。 我们需要以不同的格式（策略）生成报告，例如，以JSON或Plain Text格式。
//! 这有点想是灵活使用泛型的结果
//!
//! # Good
//! 主要优势是关注点分离。
//! Report对Json和Text的具体实现一无所知，而输出实现则不关心数据如何被预处理、存储和获取。

pub mod mode_1;
pub mod mode_2;

