use async_trait::async_trait;
use crate::module::admin::schema::AdminLoginSchema;
use validator::Validate;

#[async_trait]
pub trait AdminService {
    async fn login(&self, credentials: &AdminLoginSchema) -> Result<String, String>;
}

pub trait AdminValidation {
    fn validate_login(data: &AdminLoginSchema) -> Result<(), String> {
        // Validate using the derive(Validate) implementation
        if let Err(errors) = data.validate() {
            return Err(errors.to_string());
        }
        Ok(())
    }
}
