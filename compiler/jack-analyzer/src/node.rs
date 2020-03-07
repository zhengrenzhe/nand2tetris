use crate::constant::Token;

#[derive(Debug, Clone)]
pub enum NodeType {
    Class,
    ClassVarDec,
    SubRoutineDec,
    ParameterList,
    SubRoutineBody,
    VarDec,
    Statements,
    LetStatement,
    IfStatement,
    WhileStatement,
    DoStatement,
    ReturnStatement,
    Expression,
    ExpressionList,
    Term,
    Atom,
}

#[derive(Debug, Clone)]
pub struct Node {
    pub name: NodeType,
    pub payload: Token,
    pub children: Vec<Node>,
}

impl Node {
    pub fn new(name: NodeType, payload: Token) -> Node {
        Node {
            name,
            payload,
            children: vec![],
        }
    }

    pub fn append_child(&mut self, child: Node) {
        self.children.push(child);
    }
}
