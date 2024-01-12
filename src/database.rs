use anyhow::{Result, Context};
use libsql_client::{Client, Statement, Config};


pub struct ClientDatabase {
    db: Client
}

impl ClientDatabase {

    pub async fn new() -> Result<Self> {
        tracing::info!("Instantiating a new database client");
        
        if std::env::var("LIBSQL_CLIENT_URL").is_err() {
            println!("Error with lib client url.");
        }

        if std::env::var("LIBSQL_CLIENT_TOKEN").is_err() {
            println!("Error with lib token.");
        }

        let config = Config {
            url: url::Url::parse(env!("LIBSQL_CLIENT_URL")).unwrap(),
            auth_token: Some(String::from(env!("LIBSQL_CLIENT_TOKEN")))
        };

        let db = Client::from_config(config).await.unwrap();

        db.batch([
                 "CREATE TABLE IF NOT EXISTS mail (date text, sender text, recipients text, data text)",
                 "CREATE INDEX IF NOT EXISTS mail_date ON mail(date)",
                 "CREATE INDEX IF NOT EXISTS mail_recipients ON mail(recipients)",
        ]).await?;
        
        Ok(Self {db})
    }

    pub async fn test_db() {

        let config = Config {
            url: url::Url::parse(env!("LIBSQL_CLIENT_URL")).unwrap(),
            auth_token: Some(String::from(env!("LIBSQL_CLIENT_TOKEN")))
        };

        let db = Client::from_config(config).await.unwrap();

        let response = match db.execute("SELECT * FROM test").await {
            Ok(response) => response,
            Err(e) => {
                println!("Inside Error {}", e);
                return;
            }
        };
        println!("Db test row: {:?}", response);
    }


}
