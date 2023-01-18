use super::types::*;
use nom::IResult as NomResult;

pub trait ParseBin<T: Sized> {
    fn parse(bytes: &[u8]) -> ParseResult<(Vec<Byte>, T)>
    where
        Self: Sized;
}

pub trait ParseWithNom {
    fn parse(bytes: &[u8]) -> NomResult<&[Byte], Self>
    where
        Self: Sized;
}
