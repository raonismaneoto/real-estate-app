use postgres_types::ToSql;
use tokio_postgres::{GenericClient, NoTls, Row};

use crate::error::{app_error::DynAppError, default::DefaultAppError};

#[derive(Clone)]
pub struct Storage {
    connection_string: String,
}

impl Storage {
    pub fn new(host: String, database: String, user: String, passwd: String) -> Self {
        Self {
            connection_string: String::from(format!(
                "host={} dbname={} user={} password={}",
                host, database, user, passwd
            )),
        }
    }

    pub async fn exec(
        &self,
        cmd: String,
        cmd_params: &[&(dyn ToSql + Sync)],
    ) -> Result<u64, DynAppError> {
        let connection_result = tokio_postgres::connect(&self.connection_string, NoTls).await;

        match connection_result {
            Ok((client, connection)) => {
                tokio::spawn(async move {
                    if let Err(e) = connection.await {
                        eprintln!("connection error: {}", e);
                    }
                });

                match client.execute(&cmd, &cmd_params).await {
                    Ok(lines) => Ok(lines),
                    Err(error) => Err(Box::new(DefaultAppError {
                        message: Some(error.to_string()),
                        status_code: 500,
                    })),
                }
            }
            Err(err) => Err(Box::new(DefaultAppError {
                message: Some(err.to_string()),
                status_code: 500,
            })),
        }
    }

    pub async fn batch_exec(&self, cmd: String) -> Result<(), DynAppError> {
        let connection_result = tokio_postgres::connect(&self.connection_string, NoTls).await;

        match connection_result {
            Ok((client, connection)) => {
                tokio::spawn(async move {
                    if let Err(e) = connection.await {
                        eprintln!("connection error: {}", e);
                    }
                });

                match client.batch_execute(&cmd).await {
                    Ok(_) => Ok(()),
                    Err(error) => Err(Box::new(DefaultAppError {
                        message: Some(error.to_string()),
                        status_code: 500,
                    })),
                }
            }
            Err(err) => Err(Box::new(DefaultAppError {
                message: Some(err.to_string()),
                status_code: 500,
            })),
        }
    }

    pub async fn query(
        &self,
        cmd: String,
        query_params: &[&(dyn ToSql + Sync)],
    ) -> Result<Vec<Row>, DynAppError> {
        let connection_result = tokio_postgres::connect(&self.connection_string, NoTls).await;

        match connection_result {
            Ok((client, connection)) => {
                tokio::spawn(async move {
                    if let Err(e) = connection.await {
                        eprintln!("connection error: {}", e);
                    }
                });

                match client.query(&cmd, &query_params).await {
                    Ok(rows) => Ok(rows),
                    Err(err) => Err(Box::new(DefaultAppError {
                        message: Some(err.to_string()),
                        status_code: 500,
                    })),
                }
            }
            Err(err) => {
                print!("{}", err);
                Err(Box::new(DefaultAppError {
                    message: Some(err.to_string()),
                    status_code: 500,
                }))
            }
        }
    }
}
