use std::iter::Peekable;
use std::str::Chars;

pub struct MultiPeek<'a> {
    iter: Peekable<Chars<'a>>, //note to self: the lifetime operator is here to indicate tha the Multipeek struct will not outlive the string slice it is borrowing from
    buffer: Vec<char>,
    peek_distance: usize,
}

impl<'a> MultiPeek<'a> {
    pub fn new(mut s: Peekable<Chars<'a>>, peek_distance: usize) -> Self {
        let mut buffer = Vec::with_capacity(peek_distance);
        for _ in 0..peek_distance {
            if let Some(c) = s.next() {
                buffer.push(c);
            }
        }
        Self {
            iter: s,
            buffer,
            peek_distance,
        }
    }
    pub fn peek_at(&self, n: usize) -> Option<&char> {
        self.buffer.get(n)
    }
    pub fn peek_until(&self, n: usize) -> String {
        let end_of_string = self.buffer.len();
        if n > self.peek_distance {
            panic!("Peek distance exceeded");
        }
        if n > end_of_string {
            return self.buffer[..end_of_string].iter().collect::<String>();
        }
        self.buffer[..n].iter().collect::<String>()
    }
    pub fn next(&mut self) -> Option<char> {
        if self.buffer.is_empty() {
            return None;
        }
        if let Some(next_char) = self.iter.next() {
            self.buffer.push(next_char);
        }
        Some(self.buffer.remove(0))
    }
    pub fn is_empty(&mut self) -> bool {
        self.buffer.is_empty()
    }
}
