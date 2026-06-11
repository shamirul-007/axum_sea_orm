use sea_orm::prelude::Decimal;
use validator::ValidationError;

pub fn validate_decimal(price: &Decimal) -> Result<(), ValidationError> {

    if *price <= Decimal::ZERO {
        return Err(ValidationError::new("invalid_price"));
    }

    Ok(())
}