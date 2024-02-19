use std::borrow::Cow;

use derive_more::From;
use diesel::{PgConnection, prelude::*, r2d2::ConnectionManager};
use libp2p::kad::{ProviderRecord, Record};
use libp2p::kad::record::Key;
use libp2p::kad::store::{Error, RecordStore};
use libp2p::PeerId;
use r2d2::Pool;

use crate::provider_models::ProviderRecordSerializable;
use crate::record_models::RecordSerializable;

#[derive(From, Clone, Debug)]
pub struct RedisRecordStore {
    pool: Pool<ConnectionManager<PgConnection>>,
}


impl RecordStore for RedisRecordStore {
    type RecordsIter<'a> = Box<dyn Iterator<Item=Cow<'a, Record>> + 'a>;
    type ProvidedIter<'a> = Box<dyn Iterator<Item=Cow<'a, ProviderRecord>> + 'a>;

    fn get(&self, k: &Key) -> Option<Cow<'_, Record>> {
        use crate::schema::records::dsl::*;

        let mut conn = self.pool.get().unwrap();
        records.filter(id.eq(k.to_vec()))
            .first::<RecordSerializable>(&mut conn)
            .optional()
            .unwrap()
            .map(Into::into)
            .map(Cow::Owned)
    }

    fn put(&mut self, r: Record) -> std::result::Result<(), Error> {
        use crate::schema::records::dsl::*;

        let mut conn = self.pool.get().unwrap();
        diesel::insert_into(records)
            .values(RecordSerializable::from(r))
            .execute(&mut conn)
            .unwrap();

        Ok(())
    }

    fn remove(&mut self, k: &Key) {
        use crate::schema::records::dsl::*;
        let mut conn = self.pool.get().unwrap();

        let _ = diesel::delete(records.filter(id.eq(k.to_vec())))
            .execute(&mut conn);
    }

    fn records(&self) -> Self::RecordsIter<'_> {
        use crate::schema::records::dsl::*;
        let mut conn = self.pool.get().unwrap();

        let results: Vec<RecordSerializable> = records
            .select(RecordSerializable::as_select())
            .load(&mut conn)
            .expect("Error loading provider records")
            ;

        let res: Vec<Record> = results
            .iter()
            .map(|e| (*e).clone().into())
            .collect();

        Box::new(res.into_iter().map(Cow::Owned))
    }

    fn add_provider(&mut self, r: ProviderRecord) -> Result<(), Error> {
        use crate::schema::provider_records::dsl::*;
        let mut conn = self.pool.get().unwrap();

        diesel::insert_into(provider_records)
            .values(ProviderRecordSerializable::from(r))
            .execute(&mut conn)
            .unwrap();

        Ok(())
    }

    fn providers(&self, k: &Key) -> Vec<ProviderRecord> {
        use crate::schema::provider_records::dsl::*;
        let mut conn = self.pool.get().unwrap();

        provider_records.filter(id.eq(k.to_vec()))
            .load::<ProviderRecordSerializable>(&mut conn)
            .map(|v| v.into_iter().map(Into::into).collect::<Vec<_>>())
            .expect("Error loading provider records")
    }

    fn provided(&self) -> Self::ProvidedIter<'_> {
        use crate::schema::provider_records::dsl::*;

        let mut conn = self.pool.get().unwrap();

        let results: Vec<ProviderRecordSerializable> = provider_records
            .select(ProviderRecordSerializable::as_select())
            .load(&mut conn)
            .expect("Error loading provider records")
            ;

        let res: Vec<ProviderRecord> = results
            .iter()
            .map(|e| (*e).clone().into())
            .collect();

        Box::new(res.into_iter().map(Cow::Owned))
    }

    fn remove_provider(&mut self, k: &Key, p: &PeerId) {
        use crate::schema::provider_records::dsl::*;
        let mut conn = self.pool.get().unwrap();

        diesel::delete(provider_records.filter(id.eq(k.to_vec()).and(provider.eq(p.to_bytes()))))
            .execute(&mut conn)
            .expect("Error removing provider record");
    }
}