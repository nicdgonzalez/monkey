use crate::{environment::Environment, expression::Identifier, statement::Block};

#[derive(Debug, Clone)]
pub struct Function {
    parameters: Vec<Identifier>,
    body: Block,
    env: Environment,
}

impl Function {
    pub fn new(parameters: Vec<Identifier>, body: Block, env: Environment) -> Self {
        Self {
            parameters,
            body,
            env,
        }
    }

    pub fn parameters(&self) -> &[Identifier] {
        &self.parameters
    }

    pub const fn body(&self) -> &Block {
        &self.body
    }

    pub const fn env(&self) -> &Environment {
        &self.env
    }
}

impl PartialEq for Function {
    fn eq(&self, _: &Self) -> bool {
        false
    }
}
