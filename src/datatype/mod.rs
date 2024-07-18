use crate::datatype::hashes::Hashes;
use crate::datatype::lists::Lists;
use crate::datatype::sets::Sets;
use crate::datatype::sorted_sets::SortedSets;
use crate::datatype::strings::Strings;
use crate::error::RedisError;
use crate::resp::{AsResp, RespFrame};

mod hashes;
mod lists;
mod op;
mod sets;
mod sorted_sets;
mod strings;

#[derive(Clone, Debug)]
pub enum RedisCollection {
    Strings(Strings),
    Lists(Lists),
    Hashes(Hashes),
    Sets(Sets),
    SortedSets(SortedSets),
}
impl AsResp for RedisCollection {
    fn as_resp_try(&self) -> Result<RespFrame, RedisError> {
        match self {
            RedisCollection::Strings(string) => string.as_resp_try(),
            RedisCollection::Lists(list) => list.as_resp_try(),
            RedisCollection::Hashes(hashes) => hashes.as_resp_try(),
            RedisCollection::Sets(set) => set.as_resp_try(),
            RedisCollection::SortedSets(sset) => sset.as_resp_try(),
        }
    }
}
