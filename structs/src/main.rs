#[allow(dead_code)]
fn main() {
    #[derive(Debug)]
    struct Book {
        title: String,
        author: String,
        pages: u32,
        available: bool,
    }

    #[derive(Debug)]
    struct User {
        active: bool,
        username: String,
        email: String,
        sign_in_count: u64,
    }

    let mut user1 = build_user(String::from("matt@gianni.org"), String::from("mattg"));
    user1.email = String::from("moowuzz@hotmail.com");

    println!("user1 = {:?}", user1);

    fn build_user(email: String, username: String) -> User {
        User {
            active: true,
            email,
            username,
            sign_in_count: 1,
        }
    }
}
