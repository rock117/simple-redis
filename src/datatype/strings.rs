use crate::error::RedisError;
use crate::resp;
use crate::resp::RespFrame::BulkStrings;
use crate::resp::{AsResp, RespFrame};

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Strings(Data);

#[derive(Debug, Clone, Eq, PartialEq)]
enum Data {
    String(String),
    Number(isize),
}

impl Strings {
    pub fn incr(&mut self) -> Result<isize, RedisError> {
        match &mut self.0 {
            Data::String(_) => Err(RedisError::Other),
            Data::Number(v) => {
                *v = *v + 1;
                Ok(*v)
            }
        }
    }

    pub fn append(&mut self, data: String) {
        match &mut self.0 {
            Data::String(v) => v.push_str(data.as_str()),
            Data::Number(v) => self.0 = Data::String(format!("{}{}", *v, data)),
        }
    }
}

impl AsResp for Strings {
    fn as_resp_try(&self) -> Result<RespFrame, RedisError> {
        match &self.0 {
            Data::String(str) => Ok(BulkStrings(resp::bulk_strings::BulkStrings(
                str.as_bytes().to_vec(),
            ))),
            Data::Number(n) => Ok(BulkStrings(resp::bulk_strings::BulkStrings(
                n.to_string().as_bytes().to_vec(),
            ))), // TODO use Numbers?
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::datatype::strings::{Data, Strings};

    #[test]
    fn test_incr() {
        let mut strings = Strings(Data::String("rock".into()));
        assert_eq!(true, strings.incr().is_err());

        let mut strings = Strings(Data::Number(123));
        assert_eq!(124, strings.incr().unwrap());
    }

    #[test]
    fn test_append() {
        let mut strings = Strings(Data::String("rock".into()));
        strings.append("123".into());
        assert_eq!(Strings(Data::String("rock123".into())), strings);

        let mut strings = Strings(Data::Number(123));
        strings.append("123".into());
        assert_eq!(Strings(Data::String("123123".into())), strings);
    }
}
