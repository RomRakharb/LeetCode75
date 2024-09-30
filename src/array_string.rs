pub mod merge_strings_alternately {
    fn merge_alternately(word1: String, word2: String) -> String {
        let mut str = String::new();

        let mut chars1 = word1.chars();
        let mut chars2 = word2.chars();

        loop {
            match (chars1.next(), chars2.next()) {
                (Some(c1), Some(c2)) => {
                    str.push(c1);
                    str.push(c2);
                }
                (Some(c1), None) => {
                    str.push(c1);
                    str.push_str(chars1.as_str());
                    break;
                }
                (None, Some(c2)) => {
                    str.push(c2);
                    str.push_str(chars2.as_str());
                    break;
                }
                (None, None) => break,
            }
        }

        str
    }

    pub fn test() {
        assert_eq!(
            merge_alternately("abc".to_string(), "pqr".to_string()),
            "apbqcr".to_string()
        );
        assert_eq!(
            merge_alternately("ab".to_string(), "pqrs".to_string()),
            "apbqrs".to_string()
        );
        assert_eq!(
            merge_alternately("abcd".to_string(), "pq".to_string()),
            "apbqcd".to_string()
        );
    }
}
