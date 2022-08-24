use actix_web::{HttpServer, App, Responder, HttpResponse, get, post, web};
use std::sync::{Arc, Mutex};
use services::balance::BalanceRepository;
use serde::{Serialize, Deserialize};
use std::fmt::{Display, Formatter, Result};

struct AppState {
    balances_repository: Arc<Mutex<BalanceRepository>>
}

// ********
// Routes *
// ********
#[derive(Serialize)]
struct BalanceResponse {
    free: usize,
    lock: usize
}

#[derive(Serialize, Deserialize, Clone, Copy, Debug)]
enum Coins {
    USD,
    BTC,
    ETH
}

impl Display for Coins {
        fn fmt(&self, f: &mut Formatter) -> Result {
            write!(f, "{:?}", self)
        }
    }

#[get("/{user_id}/{coin}")]
async fn get_balance(data: web::Data<AppState>, path: web::Path<(String, Coins)>) -> impl Responder {
    let (user_id, coin) = path.into_inner();
    let mut locked_repo = data.balances_repository.lock().unwrap();
    let user = locked_repo.get(&user_id, &coin.to_string());
    std::mem::drop(locked_repo);
    match user {
        Some(balance) => {
            let balance = balance.lock().unwrap();
            let response = BalanceResponse {
                free: balance.free,
                lock: balance.lock
            };
            HttpResponse::Ok().json(response)
            
        },
        None => {
            HttpResponse::NotFound().body("User not found")
        }
    }
}

#[derive(Deserialize)]
struct UpdateBalance {
    user_id: String,
    value: usize
}

#[post("/add/{coin}")]
async fn add_balance(data: web::Data<AppState>, form: web::Json<UpdateBalance>, coin: web::Path<Coins>) -> impl Responder {
    let mut locked_repo = data.balances_repository.lock().unwrap();
    let user = locked_repo.get(&form.user_id, &coin.to_string());
    std::mem::drop(locked_repo);
    match user {
        Some(balance) => {
            let mut balance = balance.lock().unwrap();
            balance.add(form.value);
            let response = BalanceResponse {
                free: balance.free,
                lock: balance.lock
            };
            HttpResponse::Ok().json(response)
            
        },
        None => {
            HttpResponse::NotFound().body("User not found")
        }
    }
}

#[post("/force_add/{coin}")]
async fn force_add_balance(data: web::Data<AppState>, form: web::Json<UpdateBalance>, coin: web::Path<Coins>) -> impl Responder {
    let mut locked_repo = data.balances_repository.lock().unwrap();
    let user = locked_repo.get(&form.user_id, &coin.to_string());
    std::mem::drop(locked_repo);
    match user {
        Some(balance) => {
            let mut balance = balance.lock().unwrap();
            balance.force_add(form.value);
            let response = BalanceResponse {
                free: balance.free,
                lock: balance.lock
            };
            HttpResponse::Ok().json(response)
        },
        None => {
            HttpResponse::NotFound().body("User not found")
        }
    }
}

#[post("/remove/{coin}")]
async fn remove_balance(data: web::Data<AppState>, form: web::Json<UpdateBalance>, coin: web::Path<Coins>) -> impl Responder {
    let mut locked_repo = data.balances_repository.lock().unwrap();
    let user = locked_repo.get(&form.user_id, &coin.to_string());
    std::mem::drop(locked_repo);
    match user {
        Some(balance) => {
            let mut balance = balance.lock().unwrap();
            match balance.remove(form.value) {
                Ok(_) => {
                    let response = BalanceResponse {
                        free: balance.free,
                        lock: balance.lock
                    };
                    HttpResponse::Ok().json(response)
                },
                Err(_) => {
                    HttpResponse::NotAcceptable().body("Cannot lock this value because there is no available balance to make this operation")
                }
            }
        },
        None => {
            HttpResponse::NotFound().body("User not found")
        }
    }
}

#[post("/force_remove/{coin}")]
async fn force_remove_balance(data: web::Data<AppState>, form: web::Json<UpdateBalance>, coin: web::Path<Coins>) -> impl Responder {
    let mut locked_repo = data.balances_repository.lock().unwrap();
    let user = locked_repo.get(&form.user_id, &coin.to_string());
    std::mem::drop(locked_repo);
    match user {
        Some(balance) => {
            let mut balance = balance.lock().unwrap();
            match balance.force_remove(form.value) {
                Ok(_) => {
                    let response = BalanceResponse {
                        free: balance.free,
                        lock: balance.lock
                    };
                    HttpResponse::Ok().json(response)
                },
                Err(_) => {
                    HttpResponse::NotAcceptable().body("Cannot lock this value because there is no available balance to make this operation")
                }
            }
        },
        None => {
            HttpResponse::NotFound().body("User not found")
        }
    }
}

#[post("/lock/{coin}")]
async fn lock_balance(data: web::Data<AppState>, form: web::Json<UpdateBalance>, coin: web::Path<Coins>) -> impl Responder {
    let mut locked_repo = data.balances_repository.lock().unwrap();
    let user = locked_repo.get(&form.user_id, &coin.to_string());
    std::mem::drop(locked_repo);
    match user {
        Some(balance) => {
            let mut balance = balance.lock().unwrap();
            match balance.lock(form.value) {
                Ok(_) => {
                    let response = BalanceResponse {
                        free: balance.free,
                        lock: balance.lock
                    };
                    HttpResponse::Ok().json(response)
                },
                Err(_) => {
                    HttpResponse::NotAcceptable().body("Cannot lock this value because there is no available balance to make this operation")
                }
            }
        },
        None => {
            HttpResponse::NotFound().body("User not found")
        }
    }
}

#[post("/unlock/{coin}")]
async fn unlock_balance(data: web::Data<AppState>, form: web::Json<UpdateBalance>, coin: web::Path<Coins>) -> impl Responder {
    let mut locked_repo = data.balances_repository.lock().unwrap();
    let user = locked_repo.get(&form.user_id, &coin.to_string());
    std::mem::drop(locked_repo);
    match user {
        Some(balance) => {
            let mut balance = balance.lock().unwrap();
            match balance.unlock(form.value) {
                Ok(_) => {
                    let response = BalanceResponse {
                        free: balance.free,
                        lock: balance.lock
                    };
                    HttpResponse::Ok().json(response)
                },
                Err(_) => {
                    HttpResponse::NotAcceptable().body("Cannot unlock this value because there is no available balance to make this operation")
                }
            }
        },
        None => {
            HttpResponse::NotFound().body("User not found")
        }
    }
}

#[post("/add_user/{coin}")]
async fn add_user(data: web::Data<AppState>, form: web::Json<UpdateBalance>, coin: web::Path<Coins>) -> impl Responder {
    let mut locked_repo = data.balances_repository.lock().unwrap();
    match locked_repo.get(&form.user_id, &coin.to_string()) {
        Some(_) => {
            HttpResponse::NotModified().body("User already knowed")
        },
        None => {
            locked_repo.add(&form.user_id, &coin.to_string());
            HttpResponse::Ok().body("User added")
        }
    }
    
}

#[actix_web::main]
pub async fn api_server(balances_repository: Arc<Mutex<BalanceRepository>>) -> std::io::Result<()> {
    HttpServer::new(move || {
        App::new()
        .app_data(web::Data::new(AppState {
            balances_repository: balances_repository.clone(),
        }))
        .service(get_balance)
        .service(add_balance)
        .service(lock_balance)
        .service(add_user)
        .service(force_add_balance)
        .service(remove_balance)
        .service(unlock_balance)
        .service(force_remove_balance)
    })
    .bind(("127.0.0.1", 8081))?
    .run()
    .await
}