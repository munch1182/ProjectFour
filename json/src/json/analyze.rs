use std::fmt::Display;

pub enum TokenType {
    ObjectStart,
    ObjectEnd,
    ArrayStart,
    ArrayEnd,
    // CommentLine,  // //
    // CommentStart, // /*
    // CommentEnd,   //  */
    Colon, // :
    Comma, // ,
    String,
    True,
    Flase,
    Number,
    Null,
    End,
}

impl Display for TokenType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let str = match self {
            TokenType::ObjectStart => "{ ".to_string(),
            TokenType::ObjectEnd => " }".to_string(),
            TokenType::ArrayStart => "[ ".to_string(),
            TokenType::ArrayEnd => " ]".to_string(),
            TokenType::Number => "num".to_string(),
            TokenType::String => "str".to_string(),
            TokenType::True => "true".to_string(),
            TokenType::Flase => "false".to_string(),
            TokenType::Colon => ": ".to_string(),
            TokenType::Comma => ", ".to_string(),
            TokenType::Null => "null".to_string(),
            // TokenType::CommentLine => "// ".to_string(),
            // TokenType::CommentStart => "/*".to_string(),
            // TokenType::CommentEnd => "*/".to_string(),
            TokenType::End => "eof".to_string(),
        };
        write!(f, "{}", str)
    }
}

pub struct JsonToken {
    token_type: TokenType,
    value: Option<String>,
}

impl std::fmt::Debug for JsonToken {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if let Some(c) = &self.value {
            write!(f, "{}({})", self.token_type, c)
        } else {
            write!(f, "{}", self.token_type)
        }
    }
}

impl JsonToken {
    fn new(token_type: TokenType) -> Self {
        Self {
            token_type,
            value: None,
        }
    }

    fn new_str(token_type: TokenType, value: String) -> Self {
        Self {
            token_type,
            value: Some(value),
        }
    }
}
pub struct StringReader {
    _chars: Vec<char>,
    _index: usize,
    _size: usize,
}

impl StringReader {
    fn new(str: String) -> Self {
        let vec: Vec<char> = str.chars().collect();
        let len = vec.len();
        Self {
            _chars: vec,
            _index: 0,
            _size: len,
        }
    }

    /**
     * 当前的字符
     *
     * 因为取用当前使用peek必定会移动index, 所以当前字符需要index-1
     */
    fn now(&self) -> Option<&char> {
        if self._index < 1 {
            return None;
        }
        return self._chars.get(self._index - 1);
    }

    /**
     * 获取当前字符到目标字符中任一字符之间的所有字符并组成Strng, 包含当前字符但不包含目标字符
     *
     * 会移动index
     *
     * @param ignore_white: 是否将之间的空白字符也加入到String中(如果是字符串中, 则应该加入)
     */
    fn collect_untill(&mut self, until: Vec<char>, ignore_white: bool) -> Option<String> {
        let now = *self.now().unwrap();
        let mut str = String::from(now);
        loop {
            match self.peek() {
                Some(c) => {
                    if until.contains(c) {
                        return Some(str);
                    } else if c.is_whitespace() && ignore_white {
                        continue;
                    } else {
                        str.push(*c)
                    }
                }
                _ => return None,
            }
        }
    }

    fn next_match(&mut self, str: String) -> Result<(), ()> {
        for ele in str.chars() {
            let c = self.peek().unwrap();
            if ele != *c {
                return Err(());
            }
        }
        Ok(())
    }

    fn peek(&mut self) -> Option<&char> {
        if self._index >= self._size {
            return None;
        }
        let c = self._chars.get(self._index);
        self._index += 1;
        return c;
    }

    fn back(&mut self) {
        if self._index > 0 {
            self._index -= 1
        }
    }
}

pub struct Json4Analyze {
    _reader: StringReader,
    pub tokens: Vec<JsonToken>,
}

impl std::fmt::Debug for Json4Analyze {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut str = String::new();
        let mut i = 0;
        for ele in &self.tokens {
            if i > 0 {
                str.push(' ');
            }
            str.push_str(format!("{:?}", ele).as_str());
            i += 1;
        }
        write!(f, "{}", str)
    }
}

impl Json4Analyze {
    pub fn new(json: String) -> Self {
        Self {
            _reader: StringReader::new(json),
            tokens: Vec::new(),
        }
    }

    pub fn analyze(&mut self) {
        while let Some(c) = self._reader.peek() {
            //匹配token开头的字符
            match c {
                '{' => self.add(TokenType::ObjectStart),
                '}' => self.add(TokenType::ObjectEnd),
                '[' => self.add(TokenType::ArrayStart),
                ']' => self.add(TokenType::ArrayEnd),
                ':' => self.add(TokenType::Colon),
                ',' => self.add(TokenType::Comma),
                'n' => self.read_null(),
                '\"' => self.read_str(),
                'f' | 't' => self.read_bool(),
                _ => {
                    let c = *c;
                    if c.is_whitespace() {
                        continue;
                    } else if ('0' <= c && c <= '9') || c == '.' || c == '-' {
                        self.read_num();
                        continue;
                    }
                    panic!()
                }
            }
        }
        self.add(TokenType::End)
    }

    fn read_num(&mut self) {
        let str = self
            ._reader
            .collect_untill(vec![',', ']', '}', ';'], true)
            .unwrap();
        self._reader.back();
        if str.contains('.') {
            let _: f64 = str.parse().unwrap();
            self.tokens.push(JsonToken::new_str(TokenType::Number, str))
        } else {
            let _: i64 = str.parse().unwrap();
            self.tokens.push(JsonToken::new_str(TokenType::Number, str))
        }
    }

    fn read_str(&mut self) {
        let mut str = self._reader.collect_untill(vec!['\"'], false).unwrap();
        str.push('\"');
        self.tokens.push(JsonToken::new_str(TokenType::String, str))
    }

    fn read_null(&mut self) {
        match self._reader.next_match("ull".to_string()) {
            Ok(_) => self.tokens.push(JsonToken::new(TokenType::Null)),
            _ => panic!("以n开头但不是null"),
        }
    }

    fn read_bool(&mut self) {
        let c = self._reader.now().unwrap();
        if *c == 't' {
            match self._reader.next_match("rue".to_string()) {
                Ok(_) => self.tokens.push(JsonToken::new(TokenType::True)),
                _ => panic!("以t开头但不是true"),
            }
        } else if *c == 'f' {
            match self._reader.next_match("alse".to_string()) {
                Ok(_) => self.tokens.push(JsonToken::new(TokenType::Flase)),
                _ => panic!("以f开头但不是false"),
            }
        } else {
            panic!("now移动了位置")
        }
    }

    fn add(&mut self, token_type: TokenType) {
        self.tokens.push(JsonToken::new(token_type))
    }
}

#[test]
fn test_parse() {
    let json = "{
        \"a\" : {
            \"b\": \"  b bb\"
        },
        \"c\" : [
            2.2  , 2.3, 2.4, 2.5
        ],
        \"d\": false,
        \"e\":null
    }"
    .to_string();
    let mut a = Json4Analyze::new(json);
    a.analyze();
    println!("{:?}", a)
}
