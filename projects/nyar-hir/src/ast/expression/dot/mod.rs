use super::*;

/// ```vk
/// [].a
/// [].a()
/// [].a::[]()
/// [].a::[]() {
///    continuation
/// }
/// ```
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct DotCall {
    pub base: ASTNode,
    pub symbol: Symbol,
    pub generic: Option<ASTNode>,
    pub arguments: Option<ASTNode>,
    pub continuation: Option<ASTNode>,
}

impl ChainBuilder {
    pub fn push_dot_call(&mut self, s: Symbol) {
        match self.base.kind {
            _ => unimplemented!("{:?}", self.base.kind),
        }
    }
}
