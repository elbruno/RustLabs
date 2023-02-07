mod authentication {
    pub struct User {
        username: String,
        password_hash: u64,
    }

    impl User {
        pub fn new(username: &str, password: &str) -> User {
            User {
                username: username.to_string(),
                password_hash: hash_password(password),
            }
        }    
    }    
    fn hash_password(input: &str) -> u64 {
        input.len() as u64
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