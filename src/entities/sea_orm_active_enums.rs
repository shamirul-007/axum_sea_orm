use sea_orm::entity::prelude::*;
use serde::{ Deserialize, Serialize };

#[derive(Debug, Clone, PartialEq, Eq, EnumIter, DeriveActiveEnum, Serialize, Deserialize)]
#[sea_orm(rs_type = "String", db_type = "String(None)")]
pub enum Roles {
    #[sea_orm(string_value = "user")]
    User,

    #[sea_orm(string_value = "admin")]
    Admin,
}

impl Roles {
    pub fn as_str(&self) -> &'static str {
        match self {
            Roles::Admin => { "admin" }
            Roles::User => { "user" }
        }
    }
}
