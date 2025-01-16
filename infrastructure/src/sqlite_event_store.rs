use chrono;
use serde::{de::DeserializeOwned, Serialize};
use uuid::Uuid;

use metamodel::{
    entity::EntityVersion,
    event::{Event, EventBody},
};
use rusqlite;

pub struct SqliteEventStore {
    conn: rusqlite::Connection,
}

// Possible errors when using append()
#[derive(Debug)]
pub enum AppendError {
    Sql(rusqlite::Error),
    Serialization(serde_json::Error),
}

#[derive(Debug)]
pub enum IterError {
    Sql(rusqlite::Error),
    UuidParse(uuid::Error),
    TimestampParse(chrono::ParseError),
    Deserialization(serde_json::Error),
}

struct Row(String, EntityVersion, String, String);

impl SqliteEventStore {
    pub fn empty() -> Result<Self, rusqlite::Error> {
        let conn = rusqlite::Connection::open_in_memory()?;
        conn.execute(
            "CREATE TABLE IF NOT EXISTS events (
                id INT PRIMARY KEY,
                aggregate_id TEXT,
                aggregate_version INT,
                timestamp TEXT NOT NULL,
                body TEXT NOT NULL
            )",
            (), // empty list of parameters.
        )?;
        Ok(Self { conn })
    }

    pub fn append<E: EventBody + Serialize>(&mut self, event: Event<E>) -> Result<(), AppendError> {
        let event_str =
            serde_json::to_string(&event.body).map_err(|err| AppendError::Serialization(err))?;

        self.conn.execute(
            "INSERT INTO events (aggregate_id, aggregate_version, timestamp, body) VALUES (?1, ?2, ?3, ?4)",
            (serialize_uuid(&event.aggregate_id),
             &event.aggregate_version,
             serialize_timestamp(&event.timestamp),
             event_str,
             ),
        ).map_err(|err| AppendError::Sql(err))?;

        Ok(())
    }

    pub fn iter<E: EventBody + DeserializeOwned>(
        &self,
    ) -> Result<Vec<metamodel::event::Event<E>>, IterError> {
        let mut stmt = self
            .conn
            .prepare("SELECT aggregate_id, aggregate_version, timestamp, body FROM events")
            .map_err(|err| IterError::Sql(err))?;
        let events = stmt
            .query_map([], |row| {
                Ok(Row(row.get(0)?, row.get(1)?, row.get(2)?, row.get(3)?))
            })
            .map_err(|err| IterError::Sql(err))?;

        events
            .map(|r| match r {
                Err(err) => Err(IterError::Sql(err)),
                Ok(row) => {
                    let uuid_str = row.0;
                    let aggregate_id = deserialize_uuid(&String::from(uuid_str))
                        .map_err(|err| IterError::UuidParse(err))?;

                    let aggregate_version = row.1;

                    let ts_str = row.2;
                    let timestamp = deserialize_timestamp(&String::from(ts_str))
                        .map_err(|err| IterError::TimestampParse(err))?;

                    let body_str = row.3;
                    let body: E = serde_json::from_str(&body_str)
                        .map_err(|err| IterError::Deserialization(err))?;

                    Ok(Event::<E> {
                        aggregate_id,
                        aggregate_version,
                        timestamp,
                        body,
                    })
                }
            })
            .collect()
    }
}

fn serialize_uuid(u: &Uuid) -> String {
    u.to_string()
}

fn deserialize_uuid(text: &str) -> Result<Uuid, uuid::Error> {
    Uuid::parse_str(text)
}

fn serialize_timestamp(ts: &chrono::DateTime<chrono::Utc>) -> String {
    return ts.to_string();
}

fn deserialize_timestamp(text: &str) -> Result<chrono::DateTime<chrono::Utc>, chrono::ParseError> {
    text.parse::<chrono::DateTime<chrono::Utc>>()
}

#[cfg(test)]
mod tests {
    use metamodel::event::now;
    use serde::Deserialize;

    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
    struct TestEvent {
        x: i32,
    }

    impl EventBody for TestEvent {}

    #[test]
    fn test_construct_empty() {
        let store = SqliteEventStore::empty().unwrap();
        let events = store.iter::<TestEvent>().unwrap();

        assert_eq!(events.len(), 0);
    }

    #[test]
    fn test_append() {
        let mut store = SqliteEventStore::empty().unwrap();

        let event = now(TestEvent { x: 42 });
        store.append(event).unwrap();
    }

    #[test]
    fn test_iter() {
        let mut store = SqliteEventStore::empty().expect("invalid");
        let event = now(TestEvent { x: 42 });
        store.append(event.clone()).unwrap();

        let events = store.iter::<TestEvent>().unwrap();

        assert_eq!(events.len(), 1);

        let actual = events.get(0).unwrap();
        assert_eq!(actual.aggregate_id, event.aggregate_id);
        assert_eq!(actual.aggregate_version, event.aggregate_version);
        assert_eq!(actual.timestamp, event.timestamp);
        assert_eq!(actual.body, event.body);
    }
}
