use tokio_postgres;

#[tokio::main]
async fn main() -> Result<(), tokio_postgres::Error> {
  // Connect to the database.
  let (client, connection) = tokio_postgres::connect("host=localhost user=postgres", tokio_postgres::NoTls).await?;
  
  // The connection object performs the actual communication with the database,
  // so spawn it off to run on its own.
  tokio::spawn(async move {
    if let Err(e) = connection.await {
      eprintln!("connection error: {}", e);
    }
  });
  
  // Now we can execute a simple statement that just returns its parameter.
  let rows = client
    .query("SELECT $1::TEXT", &[&"hello world"])
    .await?;
  
  // And then check that we got back the same string we sent over.
  let value: &str = rows[0].get(0);
  assert_eq!(value, "hello world");
  println!(">>> {}", value);
  
  Ok(())
}
