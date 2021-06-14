use mongodb::error::Error;
use mongodb::{options::ClientOptions, Client};

pub async fn connect_to_mongodb() -> Result<Client, Error> {
    // Parse a connection string into an options struct.
    let mut client_options = ClientOptions::parse("mongodb://localhost:27017").await?;

    // Manually set an option
    client_options.app_name = Some("rocket-app".to_string());

    // Get a handle to the deployment.
    let client = Client::with_options(client_options)?;

    for db_name in client.list_database_names(None, None).await.unwrap() {
        println!("{}", db_name);
    }

    Ok(client)
}
