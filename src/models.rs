use serde::{Deserialize, Serialize};
use tokio_pg_mapper_derive::PostgresMapper;
use time::OffsetDateTime;

#[derive(Serialize, Deserialize, PostgresMapper, Debug, Default)]
#[pg_mapper(table = "users")] // singular 'user' is a keyword..
pub struct User {
    pub email: String,
    pub first_name: String,
    pub last_name: String,
    pub username: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Test {
    pub field1: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Location{
    pub name: String,
    pub longitude: f64,
    pub latitude: f64,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Timeslot{
    pub id: uuid::Uuid,
    pub location_id: uuid::Uuid,
    #[serde(with = "time::serde::rfc3339")]
    pub start_time: OffsetDateTime,
    #[serde(with = "time::serde::rfc3339")]
    pub end_time: OffsetDateTime,
}


#[derive(Serialize, Deserialize, Debug)]
pub struct AddLocationParam{
    pub position: Latlong,
    pub name: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Latlong{
    pub longitude: f64,
    pub latitude: f64,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct DeleteTimeslotRequest{
    pub timeslot_id: uuid::Uuid,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct GetLocationParam{
    pub boundingbox: BoundingBox,
}


#[derive(Serialize, Deserialize, Debug)]
pub struct GetFilteredLocationParam{
    pub boundingbox: BoundingBox,
    pub name: Option<String>,
}


#[derive(Serialize, Deserialize, Debug)]
pub struct  AddTimeslotToLocationParam{
    pub location_id: String,
    #[serde(with = "time::serde::rfc3339")]
    pub start_time: OffsetDateTime,
    #[serde(with = "time::serde::rfc3339")]
    pub end_time: OffsetDateTime,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct BoundingBox{
    pub north: f64,
    pub south: f64,
    pub east: f64,
    pub west: f64,
}

impl BoundingBox{
    pub fn query(&self) -> (String, String, String, String){
        return (self.west.to_string(), self.south.to_string(), self.east.to_string(), self.north.to_string())
    }
}