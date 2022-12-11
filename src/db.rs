use deadpool_postgres::Client;
use tokio_pg_mapper::FromTokioPostgresRow;
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
    let _stmt = include_str!("../sql/add_location.sql");
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

pub async fn get_locations(client: &Client, input: &models::GetLocationParam) -> Result<Vec<models::Location>, errors::MyError>{
    let _stmt = include_str!("../sql/get_locations.sql");
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