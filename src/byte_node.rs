#[derive(Debug)]
pub struct ByteNode {
    pub byte: u8,
    pub frequency: u32,
    pub left: Option<Box<ByteNode>>,
    pub right: Option<Box<ByteNode>>
}
