use command::Address;
#[derive(Clone, Debug, PartialEq)]
pub struct Line {
    pub contents: String,
    pub addr:     Address,
}
