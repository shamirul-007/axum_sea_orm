use crate::services::UserService;

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
}
