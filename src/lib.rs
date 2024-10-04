/// This module contains user creation functions.
pub mod user;

/// Creates a new user with the given name and age.
///
/// # Arguments
///
/// * `name` - A string slice representing the user's name.
/// * `age` - A usize representing the user's age.
///
/// # Example
///
/// ```
/// use my_crate::create_new_user;
///
/// let user = create_new_user("Alice".to_string(), 30);
/// assert_eq!(user.get_name().0, "Alice");
/// assert_eq!(user.get_name().1, 30);
/// ```
pub fn create_new_user(name: String, age: usize) -> user::User {
    let new_user = user::User::new(name, age);
    return new_user;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_user_test() {
        let us = create_new_user("Chirag".to_string(), 2);
        assert_eq!(us.get_name().0, "Chirag".to_string());
        assert_eq!(us.get_name().1, 2);
    }
}
