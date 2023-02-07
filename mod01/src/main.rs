mod authentication;

fn main() {
    let mut user = authentication::User::new("ace", "ace-secret");

    println!("The username is: {}", user.get_username());
    println!("The hash pwd  is: {}", user.get_hashpwd());
    user.set_password("ace-more-secret");
    println!("The new hash pwd  is: {}", user.get_hashpwd());
}