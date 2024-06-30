use std::fmt::Display;

use derive_builder::Builder;

use crate::command::Command;
use crate::context::Context;
use crate::error::RedisError;
use crate::resp::Resp;
use crate::resp::Resp::SimpleStrings;
use crate::resp::{simple_strings, AsResp};
use crate::storage::mem::MemStorage;
use crate::storage::Storage;
use anon_enum::{Enum2, Enum5};
use itertools::Either;

/// redis set command
///
/// https://redis.io/docs/latest/commands/set/
#[derive(Builder, Default)]
pub struct Set {
    key: String,
    value: Vec<u8>,
    get: bool,
    set_key_opt: Option<SetKeyOpt>,
    expired_opt: Option<ExpiredOpt>,
}
#[derive(Copy, Clone, Debug)]
enum SetKeyOpt {
    NX,
    XX,
}

#[derive(Copy, Clone, Debug)]
enum ExpiredOpt {
    EX(usize),
    PX(usize),
    EXAT(usize),
    PXAT(usize),
    KEEPTTL,
}

impl Command for Set {
    fn execute(&self, context: &dyn Context<Storage = MemStorage>) -> Result<Resp, RedisError> {
        let storage = context.storage();
        let data = storage.get(&self.key).clone();
        // match data { TODO
        //     None => {
        //        // let strings = Strings(String::from_utf8(self.value.clone()).unwrap());
        //     },
        //     Some(coll) => coll.as_resp_try(),
        // }
        Ok(SimpleStrings(simple_strings::SimpleStrings("OK".into())))
    }
}

impl Display for Set {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut opts = String::new();
        if let Some(SetKeyOpt::XX) = self.set_key_opt {
            opts.push_str(" XX")
        }
        if let Some(SetKeyOpt::NX) = self.set_key_opt {
            opts.push_str(" NX")
        }
        if self.get {
            opts.push_str(" GET");
        }
        if let Some(ExpiredOpt::EX(ex)) = self.expired_opt {
            opts.push_str(&format!(" EX {}", ex));
        }
        if let Some(ExpiredOpt::PX(px)) = self.expired_opt {
            opts.push_str(&format!(" PX {}", px));
        }
        if let Some(ExpiredOpt::EXAT(exat)) = self.expired_opt {
            opts.push_str(&format!(" EXAT {}", exat));
        }
        if let Some(ExpiredOpt::PXAT(pxat)) = self.expired_opt {
            opts.push_str(&format!(" PXAT {}", pxat));
        }
        if let Some(ExpiredOpt::KEEPTTL) = self.expired_opt {
            opts.push_str(" KEEPTTL");
        }
        write!(
            f,
            "SET {} {}{}",
            self.key,
            String::from_utf8_lossy(&self.value),
            opts
        )
    }
}

#[cfg(test)]
mod tests {
    use crate::command::set::*;

    #[test]
    fn test_to_string() {
        let mut builder = SetBuilder::default();
        let set = builder
            .key("id".into())
            .value("100".into())
            .get(false)
            .set_key_opt(Some(SetKeyOpt::NX))
            .expired_opt(Some(ExpiredOpt::EX(3000)))
            .build()
            .unwrap();
        assert_eq!("SET id 100 NX EX 3000", set.to_string());
    }
}
