#[derive(Debug, Clone)]
pub struct Subject {
    pub id: String,
    pub scopes: Vec<String>,
}

impl Subject {
    pub fn has_scope(&self, scope: &str) -> bool {
        self.scopes.iter().any(|s| s == scope)
    }
}
