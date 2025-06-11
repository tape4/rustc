use crate::semantic::resolver::resolver::Resolver;
use crate::semantic::type_checker::type_checker::TypeChecker;

impl<'a> TypeChecker<'a> {
    pub fn new(resolver: &'a mut Resolver) -> Self {
        TypeChecker {
            resolver,
            current_ret_ty: None,
        }
    }
}
