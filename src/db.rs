/// Database functions

use std::collections::HashMap;
use mysql;

struct Record {
    fxa_uid: String,
    device_id: String,
    service: String,
    index: u64,
    ttl: u64,
    store_ref: String,
    store_size: u64,
}

struct Database {
    pool:mysql::Pool
}

impl Database {
    fn new(config: HashMap) -> Database {
        // build the table if not already present.
        let pool = mysql::Pool::new(config.get("dsn")).expect("Could not initialize data pool");
        pool.prep_exec(
            r"Create table if not exists :table_name (
                        fxa_uid text not null,
                        device_id text not null,
                        service text not null,
                        ttl int not null,
                        store_ref text not null,
                        store_size int)",
            params!{"table_name"=>config.get("db_tablename")}
        ).expect("Could not create table");

        Database{
            pool
        }
    }

    fn store(record:Record) -> Result<u64, Error>{
        /// Store Record into the database

        return 0
    }

    fn fetch(fxa_uid:String, device_id:String, service:String, limit:u8, start_at: u64) -> Result<Record, Error> {
        /// Fetch a Record from the database
        return Record{}
    }

}
