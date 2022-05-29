
use sea_orm::{Database, ConnectionTrait, Schema, Set, EntityTrait, QueryFilter, ColumnTrait, ActiveModelTrait};
use sea_orm_demo::post::prelude::*;


#[tokio::main]
async fn main() -> anyhow::Result<()> {
 
    let db_url = "sqlite:blog.db";
    let db = Database::connect(db_url).await?;
    let builder = db.get_database_backend();
    let schema = Schema::new(builder);
    let create_table_op =  
        db.execute(builder.build(&schema.create_table_from_entity(Post))).await;
    println!("`CREATE TABLE post` {:?}", 
    match create_table_op {
                  Ok(_) => "Operation Successful".to_owned(),
                   Err(e) => format!("Unsuccessful - Error {:?}", e),
                }
            );
    
    let model = PostActiveModel {
        title: Set("sea-orm".to_owned()),
        content: Set("内容2".to_owned()),
        ..Default::default()
    };
    println!("model:{:#?}",model);

    let op = Post::insert(model).exec(&db).await?;
    println!("INSERTED ONE: {:?}", op);

    let post_list = Post::find().all(&db).await?;
    println!("list:{:?}",post_list);


    let post = Post::find_by_id(2).one(&db).await?;
    println!("post:{:?}",post.unwrap());
    
    let post_list =Post::find()
        .filter(PostColumn::Title.contains("标"))
        .all(&db)
        .await?;

    println!("filter list:{:?}",post_list);


    let post = Post::find()
        .filter(PostColumn::Title.contains("sea"))
        .one(&db).await?;

    let mut post_update: PostActiveModel = post.unwrap().into();
    post_update.title = Set("new title + sea orm".to_owned());

    let post_updated = post_update.update(&db).await?;

    println!("post updated :{:?}",post_updated);

    let post = Post::find()
        .filter(PostColumn::Title.contains("sea"))
        .one(&db).await?;

    let mut post_update: PostActiveModel = post.unwrap().into();
    let delete_result = post_update.delete(&db).await?;

    println!("delete count:{:?}",delete_result.rows_affected);

    Ok(())
}
