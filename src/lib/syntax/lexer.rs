pub struct Tokens {
    token: String
}

pub struct Lexer {
    pub tokens: Vec<Tokens>,
    // tokens: [], 将Tokens注入到Vec中
    line_number: u64,
    column_number: u64,
    buffer: String, // 输入数据流
}
