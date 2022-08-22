pub extern crate redis;
use redis::Commands;
use serde_json;
use serde::de;

pub struct DB {
    connection: redis::Connection
}

impl DB {
    pub fn new(connection_url: &str) -> DB {
        let con = redis::Client::open(connection_url)
        .expect("Invalid connection URL")
        .get_connection()
        .expect("failed to connect to Redis");

        DB {
            connection: con
        }
    }

    pub fn set<T: serde::Serialize>(&mut self, key: &str, value: &T) {
        let serialized_value = serde_json::to_string(value).unwrap();
        let error_message = format!("Failed to save {} - {}", &key, &serialized_value);
        let _: () = self.connection.set(key, serialized_value).expect(&error_message.to_string());
    }

    pub fn get<T: de::DeserializeOwned>(&mut self, key: &str) -> Option<T> {
        let fn_default = |_: redis::RedisError| return Option::None;
        let fn_closure = |x: String| return Some(serde_json::from_str(&x).unwrap());

        self.connection.get(key).map_or_else(fn_default, fn_closure)
    }

    pub fn del<T: de::DeserializeOwned>(&mut self, key: &str) {
        let error_message = format!("Failed to delete {}", &key);
        let _: () = self.connection.del(key).expect(&error_message);
    }

    pub fn push<T: serde::Serialize>(&mut self, queue_name: &str, value: &T) {
        let serialized_value = serde_json::to_string(value).unwrap();
        let error_message = format!("Failed to push queue {} - {}", &queue_name, &serialized_value);
        let _: () = redis::cmd("LPUSH")
        .arg(queue_name)
        .arg(serialized_value.clone())
        .query(&mut self.connection)
        .expect(&error_message.to_string());
    }

    pub fn pop<T: de::DeserializeOwned>(&mut self, queue_name: &str) -> Option<T> {
        let fn_default = |_: redis::RedisError| return Option::None;
        let fn_closure = |x: String| return Some(serde_json::from_str(&x).unwrap());

        self.connection.lpop(queue_name, Option::None).map_or_else(fn_default, fn_closure)
    }
}