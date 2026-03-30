use std::env;


#[derive(Debug, Clone)]
pub struct Config {
    pub service_port: u16,
    pub database_url: String,
}

impl Config {
    pub fn from_env() -> Result<Self, String> {
        let service_port: u16 = env::var("SERVICE_PORT")
            .map_err(|_| "SERVICE_PORT is not set".to_string())?
            .parse()
            .map_err(|_| "SERVICE_PORT must be a valid u16".to_string())?;

        let database_url = env::var("DATABASE_URL").map_err(|_| "DATABASE_URL is not set".to_string())?;

        Ok(Self {
            service_port,
            database_url,
        })
    }
}
