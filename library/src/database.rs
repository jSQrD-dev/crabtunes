use async_sqlite::{Connection, Error};

pub struct Library {
    db: Connection,
}
