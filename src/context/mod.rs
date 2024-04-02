#[derive(Debug, Clone)]
pub struct UserContext {
    pub jwt: String,
    pub username: String,

}

impl UserContext {
    pub fn username(&self) -> String {
        self.username.clone()
    }

    pub fn jwt(&self) -> String {
        self.jwt.clone()
    }
}