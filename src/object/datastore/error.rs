//! Datastore errors
//! errors

pub enum Error<T> {
    DataStoreSpecific(T),
}
