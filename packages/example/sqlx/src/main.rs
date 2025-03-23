use sqlx::{Column, Connection, Row, SqliteConnection};

#[tokio::main]
async fn main() {
    crsqlite::init_crdt();
    let mut conn = SqliteConnection::connect("sqlite::memory:").await.unwrap();
    sqlx::raw_sql("create table foo (a primary key not null, b);")
        .execute(&mut conn)
        .await
        .unwrap();
    sqlx::raw_sql("select crsql_as_crr('foo');")
        .execute(&mut conn)
        .await
        .unwrap();
    sqlx::raw_sql("insert into foo (a,b) values (1,2);")
        .execute(&mut conn)
        .await
        .unwrap();
    let ret =     sqlx::raw_sql(r#"select "table", "pk", "cid", "val", "col_version", "db_version", "site_id", "cl", "seq" from crsql_changes;"#).fetch_all(&mut conn).await.unwrap();
    for row in ret {
        for col in row.columns() {
            let name = col.name();
            if name == "pk" || name == "site_id" {
                println!("{}: {:?}", name, row.get::<Vec<u8>, _>(name));
            } else if name == "val"
                || name == "col_version"
                || name == "db_version"
                || name == "cl"
                || name == "seq"
            {
                println!("{}: {:?}", name, row.get::<i64, _>(name));
            } else {
                println!("{}: {:?}", name, row.get::<String, _>(name));
            }
        }
        println!("==========");
    }
    sqlx::raw_sql("select crsql_finalize();")
        .execute(&mut conn)
        .await
        .unwrap();
}
