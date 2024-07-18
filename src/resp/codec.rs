use std::fs::read_to_string;

use bytes::{Buf, BufMut, BytesMut};
use nom::branch::alt;
use nom::bytes::streaming::{is_not, tag, take, take_while, take_while1};
use nom::character::streaming::{crlf, digit1, none_of};
use nom::combinator::map_res;
use nom::sequence::{delimited, pair, tuple};
use nom::Err::{Error, Failure, Incomplete};
use nom::{AsBytes, IResult, InputTake};
use tokio_util::codec::{Decoder, Encoder};
use tracing::{error, info};

use crate::error::{RedisError, RespError};
use crate::resp::bulk_strings;
use crate::resp::Resp::{Arrays, BulkStrings, Nulls, SimpleErrors, SimpleStrings};
use crate::resp::{arrays, Resp, Serializer};

pub struct RespCodec;

impl Encoder<Resp> for RespCodec {
    type Error = RedisError;

    fn encode(&mut self, item: Resp, dst: &mut BytesMut) -> Result<(), Self::Error> {
        let bytes = item.serialize().unwrap(); // TODO
        dst.put_slice(bytes.as_slice());
        Ok(())
    }
}

impl Decoder for RespCodec {
    type Item = Resp;
    type Error = RedisError;

    fn decode(&mut self, src: &mut BytesMut) -> Result<Option<Self::Item>, Self::Error> {
        let data = src.as_bytes();
        info!(
            "=== receive data: {}",
            String::from_utf8(Vec::from(data)).unwrap()
        );
        let result = decode_resp(src.as_bytes());

        match result {
            Ok(v) => Ok(Some(v.1)),
            Err(e) => match e {
                Incomplete(_) => Ok(None),
                Error(_) => Err(RedisError::RespError(RespError::InvalidResp)),
                Failure(_) => Err(RedisError::RespError(RespError::InvalidResp)),
            },
        }
    }
}

fn decode_resp(input: &[u8]) -> IResult<&[u8], Resp> {
    alt((
        decode_arrays,
        decode_simple_errors,
        decode_simple_strings,
        decode_bulk_strings,
        decode_nulls,
    ))(input.as_bytes())
}

fn decode_arrays(input: &[u8]) -> IResult<&[u8], Resp> {
    // *<number-of-elements>\r\n<element-1>...<element-n>
    let (mut remain, (_, digit, _)) = tuple((tag(b"*"), digit1, crlf))(input)?; // parse
    let len = String::from_utf8_lossy(digit)
        .parse::<usize>()
        .unwrap_or_default();
    let mut arrays = Vec::with_capacity(len);
    for i in 0..len {
        let (remain_input, resp) = decode_resp(remain)?;
        remain = remain_input;
        arrays.push(resp);
    }
    Ok((input, Arrays(arrays::Arrays(arrays))))
}

fn decode_simple_errors(input: &[u8]) -> IResult<&[u8], Resp> {
    let (remain, v) = delimited(
        tag(b"-"),
        take_while(|v| v != b'\r' && v != b'\n'),
        tag(b"\r\n"),
    )(input)?;
    let serr = crate::resp::simple_errors::SimpleErrors(String::from_utf8_lossy(v).to_string());
    Ok((remain, SimpleErrors(serr)))
}

fn decode_simple_strings(input: &[u8]) -> IResult<&[u8], Resp> {
    let (remain, v) = delimited(
        tag(b"+"),
        take_while(|v| v != b'\r' && v != b'\n'),
        tag(b"\r\n"),
    )(input)?;
    let sstr = crate::resp::simple_strings::SimpleStrings(String::from_utf8_lossy(v).to_string());
    Ok((remain, SimpleStrings(sstr)))
}

fn decode_bulk_strings(input: &[u8]) -> IResult<&[u8], Resp> {
    let (remain, (_, digit, _)) = tuple((tag(b"$"), digit1, crlf))(input)?; // parse
    let len = String::from_utf8_lossy(digit)
        .parse::<usize>()
        .unwrap_or_default();
    let (remain, (data, _)) = pair(take(len), crlf)(remain)?;
    let bstrs = bulk_strings::BulkStrings(Vec::from(data));
    Ok((remain, BulkStrings(bstrs)))
}

fn decode_nulls(input: &[u8]) -> IResult<&[u8], Resp> {
    let (remain, _) = tag(b"_\r\n")(input)?;
    Ok((remain, Nulls(crate::resp::nulls::Nulls)))
}

fn is_incomplete<I, O>(result: IResult<I, O>) -> bool {
    match result {
        Ok(_) => false,
        Err(err) => match err {
            Incomplete(_) => true,
            _ => false,
        },
    }
}

mod tests {
    use super::*;

    #[test]
    fn test_decode_nulls() {
        let (remain, resp) = decode_nulls(b"_\r\n").unwrap();
        let Nulls(_) = resp else {
            panic!("not SimpleStrings")
        };
        assert_eq!(b"", remain);
    }

    #[test]
    fn test_decode_simple_strings() {
        let sstr = decode_simple_strings(b"+OK\r\n");
        assert_eq!(true, sstr.is_ok());

        let sstr = decode_simple_strings(b"+\r\n");
        assert_eq!(true, sstr.is_ok());

        let sstr = decode_simple_strings(b"+");
        assert_eq!(true, is_incomplete(sstr));

        let (remain, resp) = decode_simple_strings(b"+OK\r\nabc").unwrap();
        let SimpleStrings(_) = resp else {
            panic!("not SimpleStrings")
        };
        assert_eq!(b"abc", remain);
    }

    #[test]
    fn test_decode_simple_errors() {
        let serr = decode_simple_errors(b"-Error\r\n");
        assert_eq!(true, serr.is_ok());

        let serr = decode_simple_errors(b"-Err\ror\r\n");
        assert_eq!(false, serr.is_ok());

        let serr = decode_simple_errors(b"-Err\nor\r\n");
        assert_eq!(false, serr.is_ok());
    }

    #[test]
    fn test_decode_bulks_strings() {
        let bstr = decode_bulk_strings(b"$3\r\nabc\r\n");
        assert_eq!(true, bstr.is_ok());

        let bstr = decode_bulk_strings(b"$3\r\nabc");
        assert_eq!(true, is_incomplete(bstr));
    }

    #[test]
    fn test_decode_arrays() {
        // *<number-of-elements>\r\n<element-1>...<element-n>
        let arrays = decode_arrays(b"*2\r\n+OK\r\n+OK\r\n");
        assert_eq!(true, arrays.is_ok());

        let arrays = decode_arrays(b"*2\r\n+OK\r\n");
        assert_eq!(true, is_incomplete(arrays));

        let arrays = decode_arrays(b"*2\r\n+O\rK\r\n");
        assert_eq!(true, arrays.is_err());
    }
}
