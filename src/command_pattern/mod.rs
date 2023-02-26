//!
//! [Rust Design Patterns - Command Patterns](https://fomalhauthmj.github.io/patterns/patterns/behavioural/command.html)
//!
//! # Description
//! 命令模式的基本思想是将行动分离成它自己的对象，并将它们作为参数传递。
//!
//! # Motivation
//! 我们把一套事物封装成一个对象，我们希望这些行动或命令之后以某种顺序被执行或调用。
//!
//! # Example
//! 定义两个数据库操作create table和add field。
//! 每一个操作都是一个可撤销的命令，例如，drop table和remove field。
//! 当用户调用数据库迁移操作时，那么每条命令都按照定义的顺序执行，当用户调用回滚操作时，那么整个命令集将以相反的顺序调用。
//!
pub mod mode_1;
pub mod mode_2;
