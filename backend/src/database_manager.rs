use sqlx::Executor;
use sqlx::Pool;
use sqlx::mysql::*;

pub async fn connect() -> Result<(), sqlx::Error> {
    let conn = MySqlPool::connect("mysql://website-api:root@127.0.0.1:3306/website").await?;
    conn.execute("BEGIN").await?;
    conn.execute(sqlx::query("")).await?;
    Ok(())
}

pub async fn create_stories() -> Result<(), sqlx::Error>{
    sqlx::query("CREATE TABLE `website`.`stories` (
        `id` INT NOT NULL,
        `title` VARCHAR(45) NOT NULL,
        `description` VARCHAR(45) NOT NULL,
        `tags` VARCHAR(90) NULL,
        PRIMARY KEY (`id`),
        UNIQUE INDEX `id_UNIQUE` (`id` ASC) VISIBLE);").execute(&conn).await?;
    Ok(())
}