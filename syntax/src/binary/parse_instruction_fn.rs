use super::{instructions::InstructionType, types::*};
use nom::IResult as NomResult;

pub fn parse_instruction(bytes: &[Byte]) -> NomResult<&[Byte], InstructionType> {
    unimplemented!()
}
