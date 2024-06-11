use crate::storage::mem::MemStorage;
use crate::storage::Storage;
pub trait Context {
    type Storage: Storage;
    fn storage(&self) -> Self::Storage;
}

pub struct AppContext;
impl Context for AppContext {
    type Storage = MemStorage;

    fn storage(&self) -> Self::Storage {
        todo!()
    }
}
