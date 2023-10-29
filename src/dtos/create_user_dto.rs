use serde::Deserialize;

#[derive(Deserialize)]
pub struct CreateUserDto {
    username: String,
}

impl CreateUserDto {
    pub fn username(&self) -> &String {
        &self.username
    }
}
