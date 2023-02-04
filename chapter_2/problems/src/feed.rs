use std::collections::VecDeque;

/// Replicates the getchar() and ungetchar() functions used in C
pub trait Feed {
    fn get_char(&mut self) -> Option<char>;
    fn unget_char(&mut self) -> Option<char>;
}

pub struct InputFeed {
    not_seen: VecDeque<char>,
    seen: Vec<char>,
}

impl InputFeed {
    pub fn new(input: String) -> Self {
        let not_seen: VecDeque<char> = input.chars().collect();

        InputFeed {
            seen: vec![],
            not_seen,
        }
    }
}

impl Feed for InputFeed {
    fn get_char(&mut self) -> Option<char> {
        if let Some(next_char) = self.not_seen.pop_front() {
            self.seen.push(next_char);
            return Some(next_char);
        };
        None
    }

    fn unget_char(&mut self) -> Option<char> {
        if let Some(prev_char) = self.seen.pop() {
            self.not_seen.push_front(prev_char);
            return Some(prev_char);
        }
        None
    }
}

#[cfg(test)]
mod tests {
    use super::{Feed, InputFeed};

    #[test]
    fn test_get_char() {
        let mut input_feed = InputFeed::new("hi".to_string());

        assert_eq!(input_feed.get_char(), Some('h'));
        assert_eq!(input_feed.get_char(), Some('i'));
        assert_eq!(input_feed.get_char(), None);
    }

    #[test]
    fn test_unget_char() {
        let mut input_feed = InputFeed::new("hi".to_string());

        assert_eq!(input_feed.unget_char(), None);
    }

    #[test]
    fn test_get_char_and_unget_char() {
        let string = "This is a test".to_string();
        let mut results = Vec::new();

        let mut input_feed = InputFeed::new(string.clone());

        for _ in 0..string.len() {
            results.push(input_feed.get_char());
        }

        for _ in 0..string.len() {
            assert_eq!(input_feed.unget_char(), results.pop().unwrap());
        }
    }
}
