use crate::models;
use crate::errors;
use deadpool_postgres::{Client};
use std::path::Path;

pub async fn get_filtered_locations(client: &Client, input: &models::GetFilteredLocationParam) -> Result<Vec<models::Location>, errors::MyError>{
    let path = Path::new("../sql/location/get_locations.sql");
    //let _stmt = include_str!(path.unwrap());
    let stmt = client.prepare(&path.to_str().unwrap()).await.unwrap();
    
    let query_result = client.query(&stmt,&[&input.boundingbox.west,&input.boundingbox.south,&input.boundingbox.east,&input.boundingbox.north]).await;
    match query_result{
        Ok(res) => {
            println!("{}",res.len());
            let x = res.iter().map(|row| models::Location{
                name: row.get(0),
                longitude: row.get(1),
                latitude: row.get(2)}
            )
            .collect::<Vec<models::Location>>();
            return Ok(x);
            },
        Err(e) => {
            println!("Error getting locations");
            return Err(errors::MyError::PGError(e));
        }
    }
}