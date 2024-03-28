#[derive(Debug, Clone)]
pub struct Ctx {
    pub jwt: String,
    pub username: String,

}

impl Ctx {
    pub fn username(&self) -> String {
        self.username.clone()
    }

    pub fn jwt(&self) -> String {
        self.jwt.clone()
    }
}