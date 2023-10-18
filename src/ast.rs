#[derive(Debug)]
pub enum NodeType {
    ProgramType,
    VariableDeclarationType,
    AssignmentExpressionType,
    MemberExpressionType,
    CallExpressionType,
    ConditionalStatementType,
    WhileStatementType,
    LoopStatementType,
    ForEachStatementType,
    ForStatementType,
    FunctionDeclarationType,
    ReturnStatementType,
    BreakStatementType,
    ImportStatementType,
    ClassDeclarationType,
    CommentType,
    PropertyType,
    ObjectLiteralType,
    ArrayLiteralType,
    NumericLiteralType,
    IdentifierType,
    BinaryExpressionType,
    UnaryExpressionType,
    LogicalExpressionType,
    StringLiteralType,
    NullLiteralType,
    ExpressionStatementType,
}

pub trait Statement {
    fn kind(&self) -> NodeType;
    fn to_string(&self) -> String;
}

pub struct Program {
    pub body: Vec<Box<dyn Statement>>,
}

impl Statement for Program {
    fn kind(&self) -> NodeType {
        return NodeType::ProgramType;
    }

    fn to_string(&self) -> String {
        return self.body.iter().map(|stmt| stmt.to_string()).collect();
    }
}

impl Program {
    pub fn new(body: Vec<Box<dyn Statement>>) -> Self {
        return Program { body };
    }
}

pub struct VariableDeclaration {
    pub constant: bool,
    pub identifier: String,
    pub value: Box<dyn Expression>,
}

impl Statement for VariableDeclaration {
    fn kind(&self) -> NodeType {
        NodeType::VariableDeclarationType
    }

    fn to_string(&self) -> String {
        let constant_str = if self.constant { "const " } else { "let " };
        return format!(
            "{}{} = {};\n",
            constant_str,
            self.identifier,
            self.value.to_string()
        );
    }
}

impl VariableDeclaration {
    pub fn new(constant: bool, identifier: String, value: Box<dyn Expression>) -> Self {
        return VariableDeclaration {
            constant,
            identifier,
            value,
        };
    }
}

pub struct FunctionDeclaration {
    parameters: Vec<String>,
    name: String,
    body: Vec<Box<dyn Statement>>,
    is_anonymous: bool,
}


impl Statement for FunctionDeclaration {
    fn kind(&self) -> NodeType {
        return NodeType::FunctionDeclarationType
    }

    fn to_string(&self) -> String {
        let mut prefix: String = "".into();
        if self.is_anonymous {
            prefix = "function(".into();
        } else {
            prefix = format!("function {}", self.name);
        }

        let params_str = self.parameters.join(", ");
        let mut body_str: String = String::new();

        for stmt in &self.body {
            body_str += &stmt.to_string();
        }

        let function = format!("{}({}) {{\n\t{}\n}}\n", prefix, params_str, body_str);
        println!("{}", function);
        return function;
    }
}

impl FunctionDeclaration {
    pub fn new(
        parameters: Vec<String>,
        name: String,
        body: Vec<Box<dyn Statement>>,
        is_anonymous: bool,
    ) -> Self {
        return FunctionDeclaration {
            parameters,
            name,
            body,
            is_anonymous,
        };
    }
}

pub struct ReturnStatement {
    value: Box<dyn Expression>,
}

impl Statement for ReturnStatement {
    fn kind(&self) -> NodeType {
        return NodeType::ReturnStatementType;
    }

    fn to_string(&self) -> String {
        return format!("return {}", self.value.to_string());
    }
}

impl ReturnStatement {
    pub fn new(value: Box<dyn Expression>) -> Self {
        return ReturnStatement { value };
    }
}

pub struct BreakStatement;

impl Statement for BreakStatement {
    fn kind(&self) -> NodeType {
        return NodeType::BreakStatementType;
    }

    fn to_string(&self) -> String {
        return String::from("break\n");
    }
}

impl BreakStatement {
    pub fn new() -> Self {
        return BreakStatement;
    }
}

pub struct ImportStatement {
    path: String,
}

impl Statement for ImportStatement {
    fn kind(&self) -> NodeType {
        return NodeType::ImportStatementType;
    }

    fn to_string(&self) -> String {
        return format!("import {}\n", self.path);
    }
}

impl ImportStatement {
    pub fn new(path: String) -> Self {
        return ImportStatement { path };
    }
}

pub struct ClassDeclaration {
    name: String,
    body: Vec<Box<dyn Statement>>,
}

impl Statement for ClassDeclaration {
    fn kind(&self) -> NodeType {
        return NodeType::ClassDeclarationType;
    }

    fn to_string(&self) -> String {
        let mut body_str: String = String::new();

        for stmt in &self.body {
            body_str += &stmt.to_string();
        }

        return format!("class {} {{\n{}}}\n", self.name, body_str);
    }
}

impl ClassDeclaration {
    pub fn new(name: String, body: Vec<Box<dyn Statement>>) -> Self {
        return ClassDeclaration { name, body };
    }
}

pub struct Comment {
    text: String,
}

impl Comment {
    pub fn new(text: String) -> Self {
        return Comment { text };
    }
}

impl Statement for Comment {
    fn kind(&self) -> NodeType {
        return NodeType::CommentType;
    }

    fn to_string(&self) -> String {
        return format!("/* {} */\n", self.text);
    }
}

pub struct ConditionalStatement {
    condition: Box<dyn Expression>,
    body: Vec<Box<dyn Statement>>,
    alternate: Vec<Box<dyn Statement>>,
}

impl Statement for ConditionalStatement {
    fn kind(&self) -> NodeType {
        return NodeType::ConditionalStatementType;
    }

    fn to_string(&self) -> String {
        let mut body_str: String = String::new();

        for stmt in &self.body {
            body_str += &stmt.to_string();
        }

        let mut alternate_str: String = String::new();

        for stmt in &self.alternate {
            alternate_str += &stmt.to_string();
        }

        return format!(
            "if ({}) {{\n{}}} else {{\n{}}}\n",
            self.condition.to_string(),
            body_str,
            alternate_str
        );
    }
}

impl ConditionalStatement {
    pub fn new(
        condition: Box<dyn Expression>,
        body: Vec<Box<dyn Statement>>,
        alternate: Vec<Box<dyn Statement>>,
    ) -> Self {
        return ConditionalStatement {
            condition,
            body,
            alternate,
        };
    }
}

pub struct WhileStatement {
    condition: Box<dyn Expression>,
    body: Vec<Box<dyn Statement>>,
}

impl Statement for WhileStatement {
    fn kind(&self) -> NodeType {
        return NodeType::WhileStatementType;
    }

    fn to_string(&self) -> String {
        let mut body_str: String = String::new();

        for stmt in &self.body {
            body_str += &stmt.to_string();
        }

        return format!(
            "while ({}) {{\n{}}}\n",
            self.condition.to_string(),
            body_str
        );
    }
}

impl WhileStatement {
    pub fn new(condition: Box<dyn Expression>, body: Vec<Box<dyn Statement>>) -> Self {
        return WhileStatement { condition, body };
    }
}

pub struct LoopStatement {
    body: Vec<Box<dyn Statement>>,
}

impl Statement for LoopStatement {
    fn kind(&self) -> NodeType {
        return NodeType::LoopStatementType;
    }

    fn to_string(&self) -> String {
        let mut body_str: String = String::new();

        for stmt in &self.body {
            body_str += &stmt.to_string();
        }

        return format!("loop {{\n{}}}\n", body_str);
    }
}

impl LoopStatement {
    pub fn new(body: Vec<Box<dyn Statement>>) -> Self {
        return LoopStatement { body };
    }
}

pub struct ForEachStatement {
    variable: String,
    collection: Box<dyn Expression>,
    body: Vec<Box<dyn Statement>>,
}

impl Statement for ForEachStatement {
    fn kind(&self) -> NodeType {
        return NodeType::ForEachStatementType;
    }

    fn to_string(&self) -> String {
        let mut body_str: String = String::new();

        for stmt in &self.body {
            body_str += &stmt.to_string();
        }

        return format!(
            "for ({} in {}) {{\n{}}}\n",
            self.variable,
            self.collection.to_string(),
            body_str
        );
    }
}

impl ForEachStatement {
    pub fn new(
        variable: String,
        collection: Box<dyn Expression>,
        body: Vec<Box<dyn Statement>>,
    ) -> Self {
        return ForEachStatement {
            variable,
            collection,
            body,
        };
    }
}

pub struct ForStatement {
    init: Box<dyn Statement>,
    condition: Box<dyn Expression>,
    update: Box<dyn Expression>,
    body: Vec<Box<dyn Statement>>,
}

impl Statement for ForStatement {
    fn kind(&self) -> NodeType {
        return NodeType::ForStatementType;
    }

    fn to_string(&self) -> String {
        let mut body_str: String = String::new();

        for stmt in &self.body {
            body_str += &stmt.to_string();
        }

        return format!(
            "for ({}, {}; {}) {{\n{}}}\n",
            self.init.to_string(),
            self.condition.to_string(),
            self.update.to_string(),
            body_str
        );
    }
}

impl ForStatement {
    pub fn new(
        init: Box<dyn Statement>,
        condition: Box<dyn Expression>,
        update: Box<dyn Expression>,
        body: Vec<Box<dyn Statement>>,
    ) -> Self {
        return ForStatement {
            init,
            condition,
            update,
            body,
        };
    }
}



pub trait Expression: Statement {}

pub struct AssignmentExpression {
    pub assignee: Box<dyn Expression>,
    pub value: Box<dyn Expression>,
}

impl Statement for AssignmentExpression {
    fn kind(&self) -> NodeType {
        return NodeType::AssignmentExpressionType;
    }

    fn to_string(&self) -> String {
        return format!(
            "{} = {}",
            self.assignee.to_string(),
            self.value.to_string()
        );
    }
}

impl Expression for AssignmentExpression {}

impl AssignmentExpression {
    pub fn new(assignee: Box<dyn Expression>, value: Box<dyn Expression>) -> Self {
        return AssignmentExpression { assignee, value };
    }
}

pub struct BinaryExpression {
    left: Box<dyn Expression>,
    right: Box<dyn Expression>,
    operator: String,
}

impl Statement for BinaryExpression {
    fn kind(&self) -> NodeType {
        return NodeType::BinaryExpressionType;
    }

    fn to_string(&self) -> String {
        return format!(
            "{} {} {}",
            self.left.to_string(),
            self.operator,
            self.right.to_string()
        );
    }
}

impl Expression for BinaryExpression {}

impl BinaryExpression {
    pub fn new(left: Box<dyn Expression>, right: Box<dyn Expression>, operator: String) -> Self {
        return BinaryExpression {
            left,
            right,
            operator,
        };
    }
}

pub struct UnaryExpression {
    value: Box<dyn Expression>,
    operator: String,
}

impl Statement for UnaryExpression {
    fn kind(&self) -> NodeType {
        return NodeType::UnaryExpressionType;
    }

    fn to_string(&self) -> String {
        return format!("{}{}", self.operator, self.value.to_string());
    }
}

impl Expression for UnaryExpression {}

impl UnaryExpression {
    pub fn new(value: Box<dyn Expression>, operator: String) -> Self {
        return UnaryExpression { value, operator };
    }
}

pub struct LogicalExpression {
    pub left: Box<dyn Expression>,
    pub right: Box<dyn Expression>,
    pub operator: String,
}

impl Statement for LogicalExpression {
    fn kind(&self) -> NodeType {
        return NodeType::LogicalExpressionType;
    }

    fn to_string(&self) -> String {
        if self.operator == "not" {
            return format!("{}{}", self.operator, self.right.to_string());
        } else {
            return format!(
                "{} {} {}",
                self.left.to_string(),
                self.operator,
                self.right.to_string()
            );
        }
    }
}

impl Expression for LogicalExpression {}

impl LogicalExpression {
    pub fn new(left: Box<dyn Expression>, right: Box<dyn Expression>, operator: String) -> Self {
        return LogicalExpression {
            left,
            right,
            operator,
        };
    }
}

pub struct Identifier {
    sybmol: String,
}

impl Statement for Identifier {
    fn kind(&self) -> NodeType {
        return NodeType::IdentifierType;
    }

    fn to_string(&self) -> String {
        return self.sybmol.clone();
    }
}

impl Expression for Identifier {}

impl Identifier {
    pub fn new(symbol: String) -> Self {
        return Identifier { sybmol: symbol };
    }
}

pub struct NumericLiteral {
    value: f64,
}

impl Statement for NumericLiteral {
    fn kind(&self) -> NodeType {
        return NodeType::NumericLiteralType;
    }

    fn to_string(&self) -> String {
        return self.value.to_string();
    }
}

impl Expression for NumericLiteral {}

impl NumericLiteral {
    pub fn new(value: f64) -> Self {
        return NumericLiteral { value };
    }
}

pub struct StringLiteral {
    value: String,
}

impl Statement for StringLiteral {
    fn kind(&self) -> NodeType {
        return NodeType::StringLiteralType;
    }

    fn to_string(&self) -> String {
        return format!("\"{}\"", self.value);
    }
}

impl Expression for StringLiteral {}

impl StringLiteral {
    pub fn new(value: String) -> Self {
        return StringLiteral { value };
    }
}

pub struct NullLiteral {}

impl Statement for NullLiteral {
    fn kind(&self) -> NodeType {
        return NodeType::NullLiteralType;
    }

    fn to_string(&self) -> String {
        return String::from("null");
    }
}

impl Expression for NullLiteral {}

impl NullLiteral {
    pub fn new() -> Self {
        return NullLiteral {};
    }
}

pub struct Property {
    key: String,
    value: Box<dyn Expression>,
}

impl Statement for Property {
    fn kind(&self) -> NodeType {
        return NodeType::PropertyType;
    }

    fn to_string(&self) -> String {
        return format!("{}: {}", self.key, self.value.to_string());
    }
}

impl Expression for Property {}

impl Property {
    pub fn new(key: String, value: Box<dyn Expression>) -> Self {
        return Property { key, value };
    }
}

pub struct ObjectLiteral {
    properties: Vec<Property>,
}

impl Statement for ObjectLiteral {
    fn kind(&self) -> NodeType {
        return NodeType::ObjectLiteralType;
    }

    fn to_string(&self) -> String {
        let mut properties_str: String = String::new();

        for prop in &self.properties {
            properties_str += &prop.to_string();
        }

        return format!("{{{}}}", properties_str);
    }
}

impl Expression for ObjectLiteral {}

impl ObjectLiteral {
    pub fn new(properties: Vec<Property>) -> Self {
        return ObjectLiteral { properties };
    }
}

pub struct ArrayLiteral {
    elements: Vec<Box<dyn Expression>>,
}

impl Statement for ArrayLiteral {
    fn kind(&self) -> NodeType {
        return NodeType::ArrayLiteralType;
    }

    fn to_string(&self) -> String {
        let mut elements_str: String = String::new();

        for elem in &self.elements {
            elements_str += &elem.to_string();
        }

        return format!("[{}]", elements_str);
    }
}

impl Expression for ArrayLiteral {}

impl ArrayLiteral {
    pub fn new(elements: Vec<Box<dyn Expression>>) -> Self {
        return ArrayLiteral { elements };
    }
}

pub struct CallExpression {
    callee: Box<dyn Expression>,
    arguments: Vec<Box<dyn Expression>>,
}

impl Statement for CallExpression {
    fn kind(&self) -> NodeType {
        return NodeType::CallExpressionType;
    }

    fn to_string(&self) -> String {
        let mut arguments_str: String = String::new();

        for (i, arg) in self.arguments.iter().enumerate() {
            if i == 0 {
                arguments_str += &arg.to_string();
            } else {
                arguments_str += &format!(", {}", arg.to_string());
            }
        }

        return format!(
            "{}({})",
            self.callee.to_string(),
            arguments_str
        );
    }
}

impl Expression for CallExpression {}

impl CallExpression {
    pub fn new(callee: Box<dyn Expression>, arguments: Vec<Box<dyn Expression>>) -> Self {
        return CallExpression { callee, arguments };
    }
}

pub struct MemberExpression {
    object: Box<dyn Expression>,
    property: Box<dyn Expression>,
    computed: bool,
}

impl Statement for MemberExpression {
    fn kind(&self) -> NodeType {
        return NodeType::MemberExpressionType;
    }

    fn to_string(&self) -> String {
        if self.computed {
            return format!(
                "{}[{}]",
                self.object.to_string(),
                self.property.to_string()
            );
        } else {
            return format!(
                "{}.{}",
                self.object.to_string(),
                self.property.to_string()
            );
        }
    }
}

impl Expression for MemberExpression {}

impl MemberExpression {
    pub fn new(object: Box<dyn Expression>, property: Box<dyn Expression>, computed: bool) -> Self {
        return MemberExpression {
            object,
            property,
            computed,
        };
    }
}

pub struct ExpressionStatement {
    expression: Box<dyn Expression>,
}

impl Statement for ExpressionStatement {
    fn kind(&self) -> NodeType {
        return NodeType::ExpressionStatementType;
    }

    fn to_string(&self) -> String {
        return self.expression.to_string();
    }
}

impl Expression for ExpressionStatement {}

impl ExpressionStatement {
    pub fn new(expression: Box<dyn Expression>) -> Self {
        return ExpressionStatement { expression };
    }
}
