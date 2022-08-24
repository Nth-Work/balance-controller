use std::collections::BTreeMap;
use std::sync::{Arc, Mutex};
use drivers::db;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
struct PrimitiveBalance {
    free: usize,
    lock: usize,
}

pub struct Balance {
    pub free: usize,
    pub lock: usize,
    pub coin: String,
    user_id: String,
    db: db::DB
}

impl Balance {
    pub fn add(&mut self, val: usize) {
        self.lock += val;
        self.commit();
    }

    pub fn unlock(&mut self, val: usize) -> Result<(), ()> {
        if val > self.lock {
            return Err(())
        }

        self.free += val;
        self.lock -= val;
        self.commit();
        Ok(())
    }

    pub fn lock(&mut self, val: usize) -> Result<(), ()> {
        if val > self.free {
            return Err(())
        }

        self.free -= val;
        self.lock += val;
        self.commit();
        Ok(())
    }


    pub fn remove(&mut self, val: usize) -> Result<(), ()> {
        if val > self.lock {
            return Err(())
        }
        self.lock -= val;
        self.commit();
        Ok(())
    }

    pub fn force_remove(&mut self, val: usize) -> Result<(), ()> {
        if val > self.free {
            return Err(())
        }
        self.free -= val;
        self.commit();
        Ok(())
    }

    pub fn force_add(&mut self, val: usize) {
        self.free += val;
        self.commit();
    }

    fn commit(&mut self) {
        self.db.set(&format!("{}:{}", &self.user_id, &self.coin), &PrimitiveBalance {
            free: self.free,
            lock: self.lock
        });
    }
}

pub struct BalanceRepository {
    pub balances: BTreeMap<String, Arc<Mutex<Balance>>>,
    database_url: String
}

impl BalanceRepository {
    pub fn new(database_url: &str) -> BalanceRepository {
        let balances = BalanceRepository {
            balances: BTreeMap::new(),
            database_url: String::from(database_url)
        };
        balances
    }
    pub fn add(&mut self, user_id: &String, coin: &String) {
        let mut db = db::DB::new(&self.database_url);
        let key = format!("{}:{}", user_id, coin);
        let primitive = PrimitiveBalance {
            free: 0,
            lock: 0,
        };
        // register this user in database
        db.set::<PrimitiveBalance>(&key, &primitive);
        self.balances.insert(
            String::from(&key), 
            Arc::new(Mutex::new(Balance {
                    user_id: String::from(user_id),
                    coin: String::from(coin),
                    free: primitive.free,
                    lock: primitive.lock,
                    db
                }))
        );
    }
    pub fn get(&mut self, user_id: &String, coin: &String) -> Option<Arc<Mutex<Balance>>> {
        let key = format!("{}:{}", user_id, coin);
        if self.balances.contains_key(&key) {
            return Some(self.balances.get(&key).unwrap().clone())
        } else {
            let mut db = db::DB::new(&self.database_url);
            match db.get::<PrimitiveBalance>(&key) {
                Some(user) => {
                    self.balances.insert(
                        String::from(&key), 
                        Arc::new(Mutex::new(Balance {
                                user_id: String::from(user_id),
                                coin: String::from(coin),
                                free: user.free,
                                lock: user.lock,
                                db
                            }))
                    );
                    return Some(self.balances.get(&key).unwrap().clone())
                },
                _ => { None }
            }
        }
    }
}