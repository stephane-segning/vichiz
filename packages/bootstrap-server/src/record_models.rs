use derive_more::From;
use diesel::prelude::*;
use libp2p::kad::*;
use libp2p::kad::record::Key;
use libp2p::PeerId;
use serde_derive::{Deserialize, Serialize};

use crate::schema::records;
use crate::utils::convert_sec_to_instant;

#[derive(From, Serialize, Deserialize, Debug, Clone, Queryable, Insertable, Identifiable, Selectable)]
#[diesel(check_for_backend(diesel::pg::Pg), table_name = records)]
pub(crate) struct RecordSerializable {
    /// Key of the record.
    pub id: Vec<u8>,
    /// Value of the record.
    pub value: String,
    /// The (original) publisher of the record.
    pub publisher: Option<String>,
    /// The expiration time as measured by a local, monotonic clock.
    pub expires: Option<i64>,
}

impl From<Record> for RecordSerializable {
    fn from(record: Record) -> Self {
        RecordSerializable {
            id: record.key.to_vec(),
            value: String::from_utf8(record.value).unwrap(),
            publisher: record.publisher.map(|p| String::from_utf8(p.to_bytes()).unwrap()),
            expires: record.expires.map(|e| e.elapsed().as_secs_f64() as i64).or_else(|| Some(0.0 as i64)),
        }
    }
}

impl Into<Record> for RecordSerializable {
    fn into(self) -> Record {
        Record {
            key: Key::new(&self.id),
            value: self.value.into_bytes(),
            publisher: self.publisher.map(|p| PeerId::from_bytes(&p.into_bytes()).unwrap()),
            expires: self.expires
                .map(|e| convert_sec_to_instant(e)),
        }
    }
}
