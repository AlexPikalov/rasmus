use super::types::*;

pub trait ParseBin<T: Sized> {
    fn parse(&mut self, bytes: &[u8]) -> ParseResult<(Vec<Byte>, T)>;
}
