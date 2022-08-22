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
        self.db.set(&self.user_id, &PrimitiveBalance {
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
    pub fn add(&mut self, user_id: &String) {
        let mut db = db::DB::new(&self.database_url);
        let primitive = PrimitiveBalance {
            free: 0,
            lock: 0,
        };
        // register this user in database
        db.set::<PrimitiveBalance>(user_id, &primitive);
        self.balances.insert(
            String::from(user_id), 
            Arc::new(Mutex::new(Balance {
                    user_id: String::from(user_id),
                    free: primitive.free,
                    lock: primitive.lock,
                    db
                }))
        );
    }
    pub fn get(&mut self, user_id: &String) -> Option<Arc<Mutex<Balance>>> {
        if self.balances.contains_key(user_id) {
            return Some(self.balances.get(user_id).unwrap().clone())
        } else {
            let mut db = db::DB::new(&self.database_url);
            match db.get::<PrimitiveBalance>(user_id) {
                Some(user) => {
                    self.balances.insert(
                        String::from(user_id), 
                        Arc::new(Mutex::new(Balance {
                                user_id: String::from(user_id),
                                free: user.free,
                                lock: user.lock,
                                db
                            }))
                    );
                    return Some(self.balances.get(user_id).unwrap().clone())
                },
                _ => { None }
            }
        }
    }
}