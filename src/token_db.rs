use bincode::{serialize, deserialize, Infinite};

use leveldb::database::Database;
use leveldb::kv::KV;
use leveldb::iterator::Iterable;
use leveldb::options::{Options,WriteOptions,ReadOptions};

use std::path::Path;

use serde_json;

#[derive(Serialize, Deserialize)]
pub struct SSDeposit {
    pub status: String,
    pub address: String, // new ETC address
    pub amount: String, // ETC amount
    pub depositAddress: String,
    pub depositAmount: String,
    pub coinType: String,
    pub orderId: String,
    pub expiration: String,
    pub params: Vec<String>,
    pub id: usize,
}

pub struct TokenDB {
  db: Database<i32>
}

impl TokenDB {
  pub fn new(path: &Path) -> TokenDB {
      let mut options = Options::new();
      options.create_if_missing = true;
      let db = match Database::open(path, options) {
        Ok(db) => { db },
        Err(e) => { panic!("failed to open database: {:?}", e) }
      };
      TokenDB {
        db: db,
      }
  }
  
  pub fn write_deposit(&self, deposit: &SSDeposit) -> () {
      let write_opts = WriteOptions::new();
      // turn into buffer
      let bytes: Vec<u8> = serialize(deposit, Infinite).unwrap();
      match self.db.put(write_opts, deposit.id as i32, &bytes) {
          Ok(_) => { () },
          Err(e) => { panic!("failed to write to database: {:?}", e) }
      };    
  }

  pub fn read_deposit(&self, key: i32) -> Option<Vec<u8>> {
      let read_opts = ReadOptions::new();
      let res = self.db.get(read_opts, key);
      let data = match res {
        Ok(data) => { data },
        Err(e) => { panic!("failed reading data: {:?}", e) }
      };
      data
  } 

  pub fn delete_deposit(&self, key: i32) -> () {
      let write_opts = WriteOptions::new();
      let res = self.db.delete(write_opts, key);
      let data = match res {
        Ok(_) => { () },
        Err(e) => { panic!("failed deleting data: {:?}", e) }
      };
  }   

  pub fn dump(&self) -> Vec<Vec<u8>> {
      let read_opts = ReadOptions::new();
      let mut iter = self.db.value_iter(read_opts);
      let mut data = vec![];
      loop {
          match iter.next() {
              Some(d) => { 
                  data.push(d);
                },
                _ => { break; }
          };
      };
      data
  } 

  pub fn dump_addrs(&self) -> Vec<String> {
      let read_opts = ReadOptions::new();
      let mut iter = self.db.value_iter(read_opts);
      let mut data = vec![];
      loop {
          match iter.next() {
              Some(d) => {
                  data.push(deserialize::<SSDeposit>(&d).unwrap().address);
                },
                _ => { break; }
          };
      };
      data
  }   

}
