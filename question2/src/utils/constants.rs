use lazy_static::lazy_static;
use std::env;

lazy_static! {
    pub static ref ADDRESS: String = _set_address();
    pub static ref PORT: u16 = _set_port();
    pub static ref DATABASE_URL: String = _set_database_url();
}

fn _set_address() -> String {
    dotenv::dotenv().ok();
    env::var("ADDRESS").unwrap()
}

fn _set_port() -> u16 {
    dotenv::dotenv().ok();
    env::var("PORT").unwrap().parse::<u16>().unwrap()
    
}

fn _set_database_url() -> String {
    dotenv::dotenv().ok();
    env::var("DATABASE_URL").unwrap()
    
}