#![allow(dead_code)]
use ch_client::prelude::{Options, Pool};
use std::convert::TryInto;
use std::env;
use url::Url;

pub fn db_url() -> String {
    env::var("DATABASE_URL").unwrap_or_else(|_| {
        "tcp://127.0.0.1:9000?execute_timeout=5s&query_timeout=20s&pool_max=4&compression=lz4".into()
    })
}

pub fn get_config() -> Options {
    let database_url = db_url();
    database_url.try_into().unwrap()
}

/// Replace connection parameters in
/// the environment with specified in `url` argument
/// Preserve host,port,database,user info
pub fn get_pool_extend(url: &str) -> Pool {
    let mut config_url = Url::parse(db_url().as_str()).unwrap();
    let url = Url::parse(url).unwrap();

    config_url.set_query(url.query());
    Pool::create(config_url).unwrap()
}

pub fn get_pool() -> Pool {
    // let url = db_url();
    let opts = get_config();
    Pool::create(opts).unwrap()
}
