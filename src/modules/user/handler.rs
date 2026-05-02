use crate::modules::user::service::UserService;

#[derive(Clone)]
pub struct UserController {
    pub service: UserService,
}

impl UserController {
    pub fn new(service: UserService) -> Self {
        Self { service }
    }

    pub async fn get_users(&self) -> serde_json::Value {
        let users = self.service.get_users().await.unwrap();

        serde_json::json!(users)
    }

    pub async fn get_user(&self, id: i32) -> Result<serde_json::Value, String> {
        let user = self.service.get_user(id).await;

        match user {
            Ok(Some(user)) => {
                Ok(
                    serde_json::json!({
                    "success": true,
                    "data": user
                })
                )
            }

            Ok(None) => { Err("User not found".to_string()) }

            Err(_) => { Err("Internal server error".to_string()) }
        }
    }
}
