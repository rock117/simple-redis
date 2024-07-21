mod decode;
mod encode;

use nom::Err::Incomplete;
use nom::IResult;

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


