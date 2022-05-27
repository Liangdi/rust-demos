use std::env;

use sqlx::{sqlite::SqlitePool, Row};

#[derive(Debug)]
#[derive(sqlx::FromRow)]
struct Post {
    id: i64,
    title: String,
    content: String,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    env::set_var("DATABASE_URL", "sqlite:blog.db");
    let db_url = "sqlite:blog.db";
    let pool = SqlitePool::connect(db_url).await?;
    let mut conn = pool.acquire().await?;

    println!("Hello, world!");

    sqlx::query("delete  from blog")
        .execute(&mut conn).await?;

    let title = "标题";
    let contnet = "内容";
    // 使用 query! 和 query_as! 两个宏,需要设定环境变量 DATABASE_URL
    // sqlx 也支持使用 .env 设定,所以这里是写在 .env 文件里面
    sqlx::query!(
        r#"
            insert into blog(title,content) values (?1,?2)
        "#,
        title,
        contnet
    ).execute(&mut conn).await?;

    let title = "标题2";
    let contnet = "内容2";
    sqlx::query!(
        r#"
            insert into blog(title,content) values (?1,?2)
        "#,
        title,
        contnet
    ).execute(&mut conn).await?;
    
    let id = 1;
    let post:Post = sqlx::query_as!(Post,
        r#"select id,title,content from blog where id = ?"#,
        id
    ).fetch_one(&mut conn)
    .await?;

    println!("result:{:#?}",post);


    let mut row = sqlx::query("select id,title,content from blog where id = ?")
        .bind(id)
        .fetch_one(&mut conn).await?;
    let title:String = row.try_get("title").unwrap();
    println!("title:{:?}",title);

    let cond = "标题%";

    // 使用自动绑定 struct  在 struct 中使用 #[derive(sqlx::FromRow)]
    let mut result = sqlx::query_as::<_, Post>("select * from blog where title like $1")
        .bind(cond)
        .fetch_all(&mut conn).await?;
    println!("post list:");
    for post in result {
        println!("post:{:#?}",post);
    }

    Ok(())
}
