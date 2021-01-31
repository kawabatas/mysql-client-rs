use dotenv::dotenv;
use mysql::prelude::*;
use mysql::*;
use std::env;
use std::fs::File;
use std::io::prelude::*;

fn main() {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let pool = Pool::new(database_url).unwrap();
    let mut conn = pool.get_conn().unwrap();

    let mut file = File::open("sql/exec.sql").expect("file not found");
    let mut data = String::new();
    file.read_to_string(&mut data)
        .expect("something went wrong reading the file");
    let queries: Vec<&str> = data.trim().split("\n").collect();

    for q in queries {
        println!("{}", q);
        conn.query_iter(q).unwrap();
    }
}
