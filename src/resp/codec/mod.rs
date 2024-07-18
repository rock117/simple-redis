use nom::Err::Incomplete;
use nom::IResult;

mod decode;
mod encode;

pub struct RespCodec;

fn is_incomplete<I, O>(result: IResult<I, O>) -> bool {
    match result {
        Ok(_) => false,
        Err(err) => match err {
            Incomplete(_) => true,
            _ => false,
        },
    }
}
