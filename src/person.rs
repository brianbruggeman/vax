/// Identifies why someone qualifies for the shot
pub enum Qualification {
    Age,
    ChronicCondition,
    Other,
}

/// Constraints for an individual
pub struct Constraint {}

/// Person
pub struct Person {
    /// First name
    // Reqired by HEB.
    pub first_name: String,
    /// Surname
    // Reqired by HEB.
    pub last_name: String,
    /// Email address
    // Reqired by HEB.
    pub email: String,
    /// Phone number
    // Reqired by HEB.
    pub phone: Vec<String>,
    /// Date of Birth
    // Reqired by HEB.
    pub date_of_birth: String,
    /// Insurance flag; true implies an individual has insurance
    // Reqired by HEB.
    pub insurance: bool,
    /// How a person qualifies for the vaccine
    // Reqired by HEB.
    pub qualification: Qualification,
    /// Constraints required by an individual for making an appointment
    pub constraints: Vec<Constraint>,
}
