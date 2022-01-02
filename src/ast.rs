/// 書籍を見るとOOPっぽく書きたくなるが、
/// 取りうるデータ構造を列挙してmatch式で分岐させた方がrustっぽいらしい。
/// 書籍のtokenフィールドはenumで代用できるため不要になる。
#[derive(Debug)]
pub enum Statement {
    LET { name: Expression, value: Expression },
}

#[derive(Debug)]
pub enum Expression {
    Identifier(String),
}

pub struct Program {
    pub statements: Vec<Statement>,
}
