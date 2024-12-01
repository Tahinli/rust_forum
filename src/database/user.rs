
use crate::feature::user::User;

use super::{SurrealCountReturn, SurrealUserReturn, DB};


pub async fn create_user(user:User) -> SurrealUserReturn {
    DB.create("user").content(user).await
}

pub async fn read_user(email: &String) -> SurrealUserReturn{
    DB.select(("user", email)).await
}

pub async fn update_user(target_user_email: &String, user: User) -> SurrealUserReturn{
    DB.update(("user", target_user_email)).content(user).await
}
pub async fn delete_user(email: &String) -> SurrealUserReturn {
    DB.delete(("user", email)).await
}
pub async fn count_users() -> SurrealCountReturn{
    DB.query("SELECT count() FROM user GROUP BY count;").await?.take("count")
}
pub async fn count_male_users() -> SurrealCountReturn{
    DB.query("SELECT count() FROM user WHERE gender = true GROUP BY count;").await?.take("count")
}
pub async fn count_female_users() -> SurrealCountReturn{
    DB.query("SELECT count() FROM user WHERE gender = false GROUP BY count;").await?.take("count")
}