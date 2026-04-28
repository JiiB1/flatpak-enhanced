pub struct Aliases {
    pub target: String,
    pub aliases: Vec<String>,
}

impl Aliases {
    pub fn new(target: String, aliases: Vec<String>) -> Self {
        Aliases { target, aliases }
    }
}
