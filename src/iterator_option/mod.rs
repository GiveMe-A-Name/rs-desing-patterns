//! # Description
//! Option可以被看作是一个包含零或一个元素的容器。
//! 特别是，它实现了 `IntoIterator` trait，因此可以用于需要这种类型的通用代码。

#[cfg(test)]
mod tests {
    #[test]
    fn test1() {
        let turing = Some("Turing");
        let mut logicians = vec!["Curry", "Kleene", "Markov"];
        logicians.extend(turing);

        // if let Some(turing_inner) = turing {
        //     logicians.push(turing_inner);
        // }
        // or

        assert_eq!(
            logicians
                .into_iter()
                .chain(turing.into_iter())
                .collect::<Vec<_>>(),
            vec!["Curry", "Kleene", "Markov", "Turing", "Turing"]
        )
    }
}
