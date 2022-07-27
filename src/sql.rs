use serde::{Deserialize, Serialize};

use mysql::*;
use mysql::{Pool, OptsBuilder};
use mysql::prelude::Queryable;

mod db;

#[derive(Debug, Serialize, Deserialize)]
pub struct News {
    id: i32,
    pub title: String,
    pub href: String,
    date: String,
}

fn get_opts_builder() -> OptsBuilder {
    let url = OptsBuilder::new()
        .user(Some(db::DB_USER))
        .ip_or_hostname(Some(db::DB_IP))
        .pass(Some(db::DB_PASSWORD))
        .db_name(Some(db::DB_NAME))
        .tcp_port(db::DB_PORT);
    
    url
}

fn set_conn() -> PooledConn {
    let url = get_opts_builder();

    let pool = Pool::new(url).unwrap();
    let conn = pool.get_conn().unwrap();

    conn
}

pub fn get_news_from_sql() -> Vec<News> {
    let mut conn = set_conn();
        
    let news = conn.query_map(
        "SELECT * FROM news", |(id, title, href, date)|
        News {
            id: id,
            title: title,
            href: href,
            date: date,
        })
        .expect("Failed");
    
    news
}

pub fn insert_to_sql(title: &str, href: &str) {
    let mut conn = set_conn();

    conn.exec_drop(
        "INSERT INTO news (title, href) VALUES (:title, :href)",
        params! {
            "title" => title,
            "href" => href,
        },
    ).unwrap();
}

pub fn check_in_sql(title: &str) -> i32 {
    let mut conn = set_conn();
    
    let stmt;

    if title.contains("\"") {
        stmt = "SELECT EXISTS (SELECT title FROM news WHERE title=\'".to_owned() + title + &"\' LIMIT 1) AS SUCCESS".to_owned();
    } else {
        stmt = "SELECT EXISTS (SELECT title FROM news WHERE title=\"".to_owned() + title + &"\" LIMIT 1) AS SUCCESS".to_owned();
    }

    let check = conn.query_map(
        stmt, |check|
        (check)
    ).unwrap();

    check[0]
}

pub fn delete_all() {
    let mut conn = set_conn();

    conn.query_drop(r"DELETE FROM news").unwrap();
}

pub fn reset_auto_increment() {
    let mut conn = set_conn();

    conn.query_drop(r"ALTER TABLE news AUTO_INCREMENT = 1").unwrap();
}