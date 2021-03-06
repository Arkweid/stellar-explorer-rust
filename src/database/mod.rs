#![allow(dead_code)]

pub use self::models::peer::Peer;

mod models;
mod repository;

pub(crate) use self::repository::db_conn;
pub(crate) use crate::config::CONFIG;
pub(crate) use crate::schema;
pub(crate) use diesel::r2d2::{ConnectionManager, Pool, PooledConnection};
pub(crate) use diesel::sqlite::SqliteConnection;
pub(crate) use dotenv::dotenv;
pub(crate) use lazy_static::lazy_static;
pub(crate) use log::info;
pub(crate) use std::env;

pub(crate) fn init() {
    Peer::load_initial_peers();
}
