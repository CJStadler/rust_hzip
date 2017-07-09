use std::cmp::Ordering;

#[derive(Clone, Debug, Eq)]
pub struct ByteNode {
    pub byte: Option<u8>,
    pub frequency: u32,
    pub left: Option<Box<ByteNode>>,
    pub right: Option<Box<ByteNode>>
}

impl ByteNode {
    pub fn is_leaf(&self) -> bool {
        self.left.is_none() && self.right.is_none()
    }

    pub fn left(&self) -> ByteNode {
        *self.left.clone().unwrap()
    }

    pub fn right(&self) -> ByteNode {
        *self.right.clone().unwrap()
    }
}

impl Ord for ByteNode {
    fn cmp(&self, other: &ByteNode) -> Ordering {
        self.frequency.cmp(&other.frequency)
    }
}

impl PartialOrd for ByteNode {
    fn partial_cmp(&self, other: &ByteNode) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for ByteNode {
    fn eq(&self, other: &ByteNode) -> bool {
        self.frequency == other.frequency
    }
}
