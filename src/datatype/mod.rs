use crate::datatype::hashes::Hashes;
use crate::datatype::lists::Lists;
use crate::datatype::sets::Sets;
use crate::datatype::sorted_sets::SortedSets;
use crate::datatype::strings::Strings;

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
