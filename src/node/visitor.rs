use swc_ecma_ast::*;
use swc_ecma_visit::Visit;

pub struct ModuleSpecifierVisitor {
    imports: Vec<String>,
}

impl ModuleSpecifierVisitor {
    pub fn new() -> Self {
        Self {
            imports: Vec::new(),
        }
    }

    fn add_import(&mut self, src: &str) {
        self.imports.push(src.to_string());
    }

    pub fn collect_from(&mut self, module: &Module) -> &Self {
        self.visit_module(&module);
        self
    }

    pub fn imports(&self) -> &Vec<String> {
        &self.imports
    }
}

impl Visit for ModuleSpecifierVisitor {
    fn visit_import_decl(&mut self, import: &ImportDecl) {
        self.add_import(&import.src.value);
    }

    fn visit_call_expr(&mut self, call: &CallExpr) {
        match &call.callee {
            Callee::Expr(expr) => match &**expr {
                Expr::Ident(ident) if ident.sym == *"require" || ident.sym == *"import" => {
                    if let Some(Expr::Lit(Lit::Str(str_lit))) =
                        call.args.get(0).map(|arg| &*arg.expr)
                    {
                        self.add_import(&str_lit.value);
                    }
                }
                _ => {}
            },
            _ => return,
        }
    }
}
