use deadpool_postgres::Client;
use time::OffsetDateTime;
use tokio_pg_mapper::FromTokioPostgresRow;
use uuid::Uuid;
use std::str::FromStr;
use crate::*;

pub async fn add_user(client: &Client, user_info: models::User) -> Result<models::User, errors::MyError> {
    let _stmt = include_str!("../sql/add_user.sql");
    let _stmt = _stmt.replace("$table_fields", &models::User::sql_table_fields());
    let stmt = client.prepare(&_stmt).await.unwrap();

    client
        .query(
            &stmt,
            &[
                &user_info.email,
                &user_info.first_name,
                &user_info.last_name,
                &user_info.username,
            ],
        )
        .await?
        .iter()
        .map(|row| models::User::from_row_ref(row).unwrap())
        .collect::<Vec<models::User>>()
        .pop()
        .ok_or(errors::MyError::NotFound) // more applicable for SELECTs
}

pub async fn get_users(client: &Client) -> Result<Vec<models::User>, errors::MyError>{
    let stmt = client.prepare(include_str!("../sql/get_users.sql")).await.unwrap();

    let query_result = client
        .query(
            &stmt,&[]
        )
        .await?
        .iter()
        .map(|row| models::User::from_row_ref(row).unwrap())
        .collect::<Vec<models::User>>();
        
    return Ok(query_result);
}

pub async fn add_location(client: &Client, input: &models::AddLocationParam) -> Result<(), errors::MyError>{
    let _stmt = include_str!("../sql/location/add_location.sql");
    let argument1 = format!("GeomFromEWKT('SRID=4326;POINT({} {})')",input.position.longitude,input.position.latitude);
    let _stmt = _stmt.replace("$1", &argument1);
    let _stmt = _stmt.replace("$2", "$1");
    let stmt = client.prepare(&_stmt).await.unwrap();
    
    let query_result = client.query(&stmt,&[&input.name]).await;
    match query_result{
        Ok(_) => return Ok(()),
        Err(e) => {
            println!("Error adding location");
            return Err(errors::MyError::PGError(e));
        }
    }
}

pub async fn add_timeslot_to_location(client: &Client, input: &models::AddTimeslotToLocationParam) -> Result<String, errors::MyError>{
    let _stmt = include_str!("../sql/timeslot/add_timeslot_to_location.sql");
    let stmt = client.prepare(&_stmt).await.unwrap();
    let query_result = client.query(&stmt,&[&Uuid::from_str(&input.location_id).unwrap(),&input.start_time,&input.end_time]).await;
    match query_result{
        Ok(row) =>{
            let x: uuid::Uuid;
            x = row[0].get(0);
            return Ok(x.to_string())
        },
        Err(e) => {
            println!("Error adding timestamp to location");
            return Err(errors::MyError::PGError(e));
        }
    }
}

pub async fn delete_timeslot(client: &Client, input: &models::DeleteTimeslotRequest) -> Result<(), errors::MyError>{
    let _stmt = include_str!("../sql/timeslot/delete_timeslot.sql");
    let stmt = client.prepare(&_stmt).await.unwrap();
    let query_result = client.query(&stmt,&[&input.timeslot_id]).await;
    match query_result{
        Ok(rows) =>{
            println!("{}",rows.len()); //query_result == 0 even if + rows were affected
            println!("Successfully deleted timeslot");
            return Ok(())
        },
        Err(e) => {
            println!("Error adding timestamp to location");
            return Err(errors::MyError::PGError(e));
        }
    }
}

pub async fn get_locations(client: &Client, input: &models::GetLocationParam) -> Result<Vec<models::Location>, errors::MyError>{
    let _stmt = include_str!("../sql/location/get_locations.sql");
    let stmt = client.prepare(&_stmt).await.unwrap();
    
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


