use postgres::Client;

use openssl::ssl::{SslConnector, SslMethod};

use postgres_openssl::MakeTlsConnector;

use std::error;

fn main() -> Result<(), Box<dyn error::Error>> {
    let builder = SslConnector::builder(SslMethod::tls())?;

    let connector = MakeTlsConnector::new(builder.build());

    let mut client = Client::connect(
        "postgresql://rustdb_owner:Hfgapn6vAO4c@ep-hidden-mouse-a547c3w0.us-east-2.aws.neon.tech/rustdb?sslmode=require",
        connector,
    )?;

    // select * from rustdb.rust_table;
    for row in client.query("SELECT * FROM playing_with_neon", &[])? {
        // println!("Found row: {:?}", row);
        println!("id {}", row.get::<_, i32>(0));
        println!("name {}", row.get::<_, String>(1));
        // println!("value {}", row.get::<_, f32>(2));
    }

    // insert
    client.execute(
        "INSERT INTO playing_with_neon (name , value) VALUES ($1, $2)",
        &[&"test", &(1.0 as f32)],
    )?;

    // // update
    client.execute(
        "UPDATE playing_with_neon SET name = $1 WHERE id = $2",
        &[&"test2", &(1 as i32)],
    )?;

    // delete id 11
    client.execute(
        "DELETE FROM playing_with_neon WHERE id = $1",
        &[&(13 as i32)],
    )?;

    Ok(())
}
