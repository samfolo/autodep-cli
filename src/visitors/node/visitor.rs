use swc_ecma_ast::ImportDecl;
use swc_ecma_visit::Visit;

pub struct NodeModuleVisitor {
    declared_imports: Vec<String>,
    absolute_imports: Vec<String>,
}

impl NodeModuleVisitor {
    pub fn new() -> Self {
        NodeModuleVisitor {
            declared_imports: Vec::new(),
            absolute_imports: Vec::new(),
        }
    }

    pub fn declared_imports(&self) -> &Vec<String> {
        &self.declared_imports
    }

    pub fn absolute_imports(&self) -> &Vec<String> {
        &self.absolute_imports
    }
}

impl Visit for NodeModuleVisitor {
    fn visit_import_decl(&mut self, import_decl: &ImportDecl) {
        let src = import_decl.src.value.to_string();

        self.declared_imports.push(src.clone());
        let absolute_path = resolve_to_absolute_path(&src.clone());
        self.absolute_imports.push(absolute_path);
    }
}

// TODO: resolve absolute paths here
fn resolve_to_absolute_path(relative_path: &str) -> String {
    relative_path.to_owned() // Placeholder
}
