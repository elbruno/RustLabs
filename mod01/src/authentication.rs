pub struct User {
    username: String,
    password_hash: u64,
}

impl User {
    pub fn new(username: &str, password: &str) -> User {
        User {
            username: username.to_string(),
            password_hash: Self::hash_password(&password),
        }
    }

    fn hash_password(input: &str) -> u64 {
        input.len() as u64
    }

    pub fn get_username(&self) -> &String {
        &self.username
    }

    pub fn get_hashpwd(&self) -> u64 {
        self.password_hash
    }

    pub fn set_password(&mut self, new_password: &str) {
        self.password_hash = Self::hash_password(&new_password.to_owned())
    }
}

// create test for authentication
#[cfg(test)]
mod tests {
    use super::authentication;

    #[test]
    fn test_hash_password() {
        let user = authentication::User::new("test", "test");
        assert_eq!(user.password_hash, 4);
    }
}
