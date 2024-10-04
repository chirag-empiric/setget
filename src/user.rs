/// The `User` struct represents a user with a name and age.
///
/// # Example
///
/// ```
/// use my_crate::user::User;
///
/// let user = User::new("Bob".to_string(), 25);
/// user.print();
/// assert_eq!(user.get_name().0, "Bob");
/// ```
#[derive(Clone, Debug)]
pub struct User {
    name: String,
    age: usize,
}

impl User {
    /// Creates a new `User` instance.
    ///
    /// # Arguments
    ///
    /// * `name` - A string slice representing the user's name.
    /// * `age` - A usize representing the user's age.
    ///
    /// # Example
    ///
    /// ```
    /// let user = User::new("Alice".to_string(), 30);
    /// ```
    pub fn new(name: String, age: usize) -> Self {
        Self { name, age }
    }

    /// Prints the user's name and age.
    ///
    /// # Example
    ///
    /// ```
    /// let user = User::new("Alice".to_string(), 30);
    /// user.print();
    /// ```
    pub fn print(&self) {
        println!("Name is: {} \n Age is: {}", self.name, self.age);
    }

    /// Returns the user's name and age as a tuple.
    ///
    /// # Returns
    ///
    /// A tuple containing the user's name and age.
    ///
    /// # Example
    ///
    /// ```
    /// let user = User::new("Alice".to_string(), 30);
    /// let (name, age) = user.get_name();
    /// assert_eq!(name, "Alice");
    /// assert_eq!(age, 30);
    /// ```
    pub fn get_name(&self) -> (String, usize) {
        return (self.name.to_string(), self.age);
    }
}
