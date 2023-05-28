
use actix_web::{web, Error, HttpResponse};
use deadpool_postgres::{Client, Pool};

use super::models;
use crate::db;
use crate::errors;
use crate::repos::locations;

pub async fn add_user(
    user: web::Json<models::User>,
    db_pool: web::Data<Pool>,
) -> Result<HttpResponse, Error> 
{
    let user_info: models::User = user.into_inner();
    let client: Client = db_pool.get().await.map_err(errors::MyError::PoolError)?;
    let new_user = db::add_user(&client, user_info).await?;
    Ok(HttpResponse::Ok().json(new_user))
}

pub async fn get_users(db_pool: web::Data<Pool>) -> Result<HttpResponse, Error> 
{
    let client: Client = db_pool.get().await.map_err(errors::MyError::PoolError)?;
    let user_vec = db::get_users(&client).await?;
    Ok(HttpResponse::Ok().json(user_vec))
}

pub async fn test(_db_pool: web::Data<Pool>) -> Result<HttpResponse, Error> 
{
    let testrespose = models::Test{field1: "Field1Value".to_string()};
    Ok(HttpResponse::Ok().json(testrespose))
}

pub async fn add_location(db_pool: web::Data<Pool>, input: web::Json<models::AddLocationParam>) -> Result<HttpResponse, Error> {
    let client: Client = db_pool.get().await.map_err(errors::MyError::PoolError)?;
    let response = db::add_location(&client, &input).await;
    match response{
        Ok(()) => return Ok(HttpResponse::Accepted().finish()),
        Err(e) => return Err(actix_web::error::ErrorBadGateway(e)),
    }
}

pub async fn add_timestamp_to_location(db_pool: web::Data<Pool>, input: web::Json<models::AddTimeslotToLocationParam>) -> Result<HttpResponse, Error> {
    let client: Client = db_pool.get().await.map_err(errors::MyError::PoolError)?;
    let response = db::add_timeslot_to_location(&client, &input).await;
    match response{
        Ok(resp) => return Ok(HttpResponse::Ok().json(resp)),
        Err(e) => return Err(actix_web::error::ErrorBadGateway(e)),
    }
}

pub async fn get_locations(db_pool: web::Data<Pool>, input: web::Json<models::GetLocationParam>) -> Result<HttpResponse, Error> 
{
    let client: Client = db_pool.get().await.map_err(errors::MyError::PoolError)?;
    let locations = db::get_locations(&client,&input).await?;
    Ok(HttpResponse::Ok().json(locations))
}

pub async fn get_locations_by_filters(db_pool: web::Data<Pool>, input: web::Json<models::GetFilteredLocationParam>) -> Result<HttpResponse, Error> 
{
    let client: Client = db_pool.get().await.map_err(errors::MyError::PoolError)?;
    let locations = locations::get_filtered_locations(&client,&input).await?;
    Ok(HttpResponse::Ok().json(locations))
}

pub async fn delete_timeslot(db_pool: web::Data<Pool>, input: web::Json<models::DeleteTimeslotRequest>) -> Result<HttpResponse, Error> 
{
    let client: Client = db_pool.get().await.map_err(errors::MyError::PoolError)?;
    let locations = db::delete_timeslot(&client,&input).await;
    match locations{
        Ok(()) => return Ok(HttpResponse::Ok().finish()),
        Err(_) => return Ok(HttpResponse::BadRequest().finish())
    }
}