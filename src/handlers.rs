use std::convert::Infallible;
use warp::{self, http::StatusCode, Reply};

use crate::db::Db;
use crate::models::Customer;

pub async fn list_customers(db: Db) -> Result<impl warp::Reply, Infallible> {
    let customers = db.lock().await;
    let customers = customers.clone();
    Ok(warp::reply::json(&customers))
}

pub async fn create_customer(new_customer: Customer, db: Db) -> Result<impl Reply, Infallible> {
    let mut customers = db.lock().await;

    if customers
        .iter()
        .find(|c| c.guid == new_customer.guid)
        .is_some()
    {
        return Ok(StatusCode::CONFLICT);
    }

    customers.push(new_customer);
    Ok(StatusCode::CREATED)
}

pub async fn get_customer(guid: String, db: Db) -> Result<Box<dyn warp::Reply>, Infallible> {
    let customers = db.lock().await;

    if let Some(customer) = customers.iter().find(|c| c.guid == guid) {
        return Ok(Box::new(warp::reply::json(&customer)));
    }

    Ok(Box::new(StatusCode::NOT_FOUND))
}

// Not sure how to make the router ignore the first parameter
pub async fn update_customer(
    _: String,
    updated_customer: Customer,
    db: Db,
) -> Result<impl Reply, Infallible> {
    let mut customers = db.lock().await;

    if let Some(customer) = customers
        .iter_mut()
        .find(|c| c.guid == updated_customer.guid)
    {
        // All fields in customer are mut
        customer.first_name = updated_customer.first_name;
        customer.last_name = updated_customer.last_name;
        customer.email = updated_customer.email;
        customer.address = updated_customer.address;
        return Ok(StatusCode::OK);
    }

    Ok(StatusCode::NOT_FOUND)
}

pub async fn delete_customer(guid: String, db: Db) -> Result<impl Reply, Infallible> {
    let mut customers = db.lock().await;
    let customer_count = customers.len();

    customers.retain(|c| c.guid != guid);

    let deleted = customers.len() != customer_count;

    if deleted {
        Ok(StatusCode::NO_CONTENT)
    } else {
        Ok(StatusCode::NOT_FOUND)
    }
}
