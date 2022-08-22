use services::balance;
use std::sync::{Arc, Mutex};
mod api;

fn main() {

    let balances_repo = balance::BalanceRepository::new("redis://127.0.0.1");

    api::api_server(Arc::new(Mutex::new(balances_repo))).unwrap();
}
