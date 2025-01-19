use serde::{Deserialize, Serialize};
use tokio::sync::RwLock;

use crate::{
    error::ForumMailError,
    mail::{MailFieldsOneTimePassword, MailTemplate},
    ONE_TIME_PASSWORDS,
};

use super::user::User;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct OneTimePassword {
    pub user_id: i64,
    pub one_time_password: String,
}

impl OneTimePassword {
    pub fn init() -> RwLock<Vec<OneTimePassword>> {
        RwLock::new(vec![])
    }

    pub async fn new(user: &User, user_email: &String) -> Result<(), ForumMailError> {
        let one_time_password = "123".to_owned();
        let new_self = Self {
            user_id: user.user_id,
            one_time_password,
        };

        let mail_template =
            MailTemplate::OneTimePassword(MailFieldsOneTimePassword::new(&user.name, &new_self));

        mail_template.send_mail(user_email).await?;

        let mut one_time_passwords = ONE_TIME_PASSWORDS.write().await;
        one_time_passwords.push(new_self);
        one_time_passwords.sort_by_key(|one_time_password| one_time_password.user_id);
        drop(one_time_passwords);

        Ok(())
    }

    pub async fn verify(one_time_password: &OneTimePassword) -> bool {
        let one_time_password_search = ONE_TIME_PASSWORDS
            .read()
            .await
            .binary_search_by(|one_time_password_| one_time_password_.cmp(one_time_password));
        match one_time_password_search {
            Ok(one_time_password_index) => {
                let mut one_time_passwords = ONE_TIME_PASSWORDS.write().await;
                one_time_passwords.swap_remove(one_time_password_index);
                one_time_passwords.sort_by_key(|one_time_password| one_time_password.user_id);
                drop(one_time_passwords);

                true
            }
            Err(_) => false,
        }
    }
}
