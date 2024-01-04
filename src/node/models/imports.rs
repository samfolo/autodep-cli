pub struct NodeImport {
    raw: String,
    absolute: String,
}

impl NodeImport {
    pub fn raw(&self) -> &str {
        &self.raw
    }

    pub fn absolute(&self) -> &str {
        &self.absolute
    }
}
