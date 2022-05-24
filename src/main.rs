// Adapted from https://docs.rs/mysql_async/0.24.2/mysql_async/index.html#example
extern crate mysql_async;
extern crate tokio;

use mysql_async::prelude::*;
use mysql_async::*;

#[tokio::main]
async fn main() {
    let host = std::env::var("RS_HOST").unwrap();
    let port = match std::env::var("RS_PORT") {
        Ok(port_str) => port_str.parse::<u16>().unwrap(),
        Err(_) => 3306,
    };
    let username = std::env::var("RS_USERNAME").unwrap();
    let auth = match std::env::var("RS_PASSWORD") {
        Ok(password) => format!("{}:{}", username, password),
        Err(_) => username,
    };
    let database = std::env::var("RS_DATABASE").unwrap();
    let url = format!("mysql://{}@{}:{}/{}", &auth, &host, port, &database);
    println!("+++ URL: {}\n", &url);
    let pool = Pool::new(url.as_str());
    let mut conn = pool.get_conn().await.unwrap();

    let query = "SET FOREIGN_KEY_CHECKS=0";
    println!("--- query:\n\t{}\n\n", query);
    let stmt: Result<_> = conn.prep(query).await;
    let _: Result<Vec<Row>> = conn.exec(stmt.unwrap(), ()).await;

    let query = "DROP TABLE IF EXISTS access_tokens";
    println!("--- query:\n\t{}\n\n", query);
    let stmt: Result<_> = conn.prep(query).await;
    let _: Result<Vec<Row>> = conn.exec(stmt.unwrap(), ()).await;

    let query = r"
	CREATE TABLE access_tokens (
		id int(10) unsigned NOT NULL AUTO_INCREMENT,
		token varchar(40) COLLATE utf8mb4_unicode_ci NOT NULL,
		user_id int(10) unsigned NOT NULL,
        last_activity_at datetime NOT NULL,
        created_at datetime NOT NULL,
        type varchar(100) COLLATE utf8mb4_unicode_ci NOT NULL,
        title varchar(150) COLLATE utf8mb4_unicode_ci DEFAULT NULL,
        last_ip_address varchar(45) COLLATE utf8mb4_unicode_ci DEFAULT NULL,
        last_user_agent varchar(255) COLLATE utf8mb4_unicode_ci DEFAULT NULL,
        PRIMARY KEY (id),
        UNIQUE KEY access_tokens_token_unique (token),
        KEY access_tokens_user_id_foreign (user_id),
        KEY access_tokens_type_index (type),
        CONSTRAINT access_tokens_token_user_id_foreign FOREIGN KEY (user_id) REFERENCES users (id) ON DELETE CASCADE) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci";
    println!("--- query:{}\n\n", query);
    let stmt = conn.prep(query).await.expect("CREATE TABLE prep failed");
    let res: Result<Vec<Row>> = conn.exec(stmt, ()).await;
    res.unwrap();
}
