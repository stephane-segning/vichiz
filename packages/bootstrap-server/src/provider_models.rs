use std::str::FromStr;
use diesel::prelude::*;
use libp2p::{Multiaddr, PeerId};
use libp2p::kad::*;
use libp2p::kad::record::Key;
use serde_derive::{Deserialize, Serialize};

use crate::schema::provider_records;
use crate::utils::convert_sec_to_instant;

#[derive(Serialize, Deserialize, Debug, Clone, Queryable, Insertable, Identifiable, Selectable)]
#[diesel(check_for_backend(diesel::pg::Pg), table_name = provider_records)]
pub(crate) struct ProviderRecordSerializable {
    pub id: Vec<u8>,
    /// The provider of the value for the key.
    pub provider: Vec<u8>,
    /// The expiration time as measured by a local, monotonic clock.
    pub expires: Option<i64>,
    /// The known addresses that the provider may be listening on.
    pub addresses: Vec<Vec<u8>>,
}

impl From<ProviderRecord> for ProviderRecordSerializable {
    fn from(record: ProviderRecord) -> Self {
        let mut addresses = Vec::new();
        for x in record.addresses.iter() {
            let b = x.to_vec();
            addresses.push(b);
        }

        ProviderRecordSerializable {
            id: record.key.to_vec(),
            provider: record.provider.to_bytes(),
            expires: record.expires.map(|e| e.elapsed().as_secs() as i64),
            addresses,
        }
    }
}

impl Into<ProviderRecord> for ProviderRecordSerializable {
    fn into(self) -> ProviderRecord {
        let mut addresses = Vec::new();
        for x in self.addresses.iter() {
            let b = Multiaddr::from_str(String::from_utf8(x.to_vec()).unwrap().as_str()).unwrap();
            addresses.push(b);
        }

        ProviderRecord {
            key: Key::new(&self.id),
            provider: PeerId::from_bytes(&self.provider).unwrap(),
            expires: self.expires.map(|e| convert_sec_to_instant(e)),
            addresses,
        }
    }
}
