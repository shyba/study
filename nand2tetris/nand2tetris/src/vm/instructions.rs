#[derive(Debug, PartialEq, Eq)]
pub enum Segment {
    Argument,
    Local,
    Static,
    Constant,
    This,
    That,
    Pointer,
    Temp,
}

#[derive(Debug, PartialEq, Eq)]
pub enum Arithmetic {
    Add,
    Sub,
    Neg,
    Eq,
    Gt,
    Lt,
    And,
    Or,
    Not,
}

#[derive(Debug, PartialEq, Eq)]
pub struct VMFunctionCall {
    pub target: String,
    pub from: String,
    pub arguments: u16,
}

#[derive(Debug, PartialEq, Eq)]
pub enum VMInstruction {
    Comment(String),
    Push(Segment, u16),
    Pop(Segment, u16),
    Arithmetic(Arithmetic),
    Label(String),
    GoTo(String),
    IfGoTo(String),
    Function(String, u16),
    Return,
    Call(VMFunctionCall),
}

pub fn try_parse_arithmetic(line: &str) -> Option<VMInstruction> {
    match line.trim().to_lowercase().as_str() {
        "add" => Some(VMInstruction::Arithmetic(Arithmetic::Add)),
        "sub" => Some(VMInstruction::Arithmetic(Arithmetic::Sub)),
        "neg" => Some(VMInstruction::Arithmetic(Arithmetic::Neg)),
        "eq" => Some(VMInstruction::Arithmetic(Arithmetic::Eq)),
        "gt" => Some(VMInstruction::Arithmetic(Arithmetic::Gt)),
        "lt" => Some(VMInstruction::Arithmetic(Arithmetic::Lt)),
        "and" => Some(VMInstruction::Arithmetic(Arithmetic::And)),
        "or" => Some(VMInstruction::Arithmetic(Arithmetic::Or)),
        "not" => Some(VMInstruction::Arithmetic(Arithmetic::Not)),
        _ => None,
    }
}

pub fn parse_push(line: String) -> VMInstruction {
    let words: Vec<&str> = line.split_whitespace().collect();
    if words.len() != 3 {
        panic!(
            "Error parsing push. Expected: push <segment> <value>, got:{}",
            line
        )
    }
    VMInstruction::Push(
        parse_segment(words[1]),
        words[2].parse::<u16>().expect("Error parsing push index"),
    )
}

pub fn parse_pop(line: String) -> VMInstruction {
    let words: Vec<&str> = line.split_whitespace().collect();
    if words.len() != 3 {
        panic!("Error parsing pop. Expected: pop <segment> <value>")
    }
    VMInstruction::Pop(
        parse_segment(words[1]),
        words[2].parse::<u16>().expect("Error parsing pop index"),
    )
}

pub fn parse_segment(segment: &str) -> Segment {
    match segment {
        "argument" => Segment::Argument,
        "pointer" => Segment::Pointer,
        "local" => Segment::Local,
        "static" => Segment::Static,
        "this" => Segment::This,
        "that" => Segment::That,
        "temp" => Segment::Temp, // double check
        "constant" => Segment::Constant,
        unexpected => panic!("Unexpected segment: {}", unexpected),
    }
}

pub struct Parser {
    pub line_number: u16,
    pub current_function: Option<String>,
}

impl Parser {
    pub fn new() -> Self {
        Self {
            line_number: 0,
            current_function: None,
        }
    }
    pub fn parse_line(&mut self, line: &String) -> VMInstruction {
        self.line_number += 1;
        let line = if line.contains("//") && !line.starts_with("//") {
            line.split("//").next().unwrap().trim()
        } else {
            line
        };
        let line = &String::from(line);

        let lower_line = line.trim().to_lowercase();

        if line.is_empty() || line.starts_with("//") {
            VMInstruction::Comment(line.to_string()) // todo: handle comments after instructions
        } else if lower_line.starts_with("push") {
            parse_push(line.to_lowercase())
        } else if try_parse_arithmetic(line).is_some() {
            try_parse_arithmetic(line).unwrap()
        } else if lower_line.starts_with("pop") {
            parse_pop(line.to_lowercase())
        } else if lower_line.starts_with("label") {
            let label = line.split_whitespace().nth(1).unwrap();
            VMInstruction::Label(String::from(label))
        } else if lower_line.starts_with("if-goto") {
            let label = line.split_whitespace().nth(1).unwrap();
            VMInstruction::IfGoTo(String::from(label))
        } else if lower_line.starts_with("goto") {
            let label = line.split_whitespace().nth(1).unwrap();
            VMInstruction::GoTo(String::from(label))
        } else if lower_line.starts_with("function") {
            let mut pieces = line.split_whitespace().skip(1);
            let label = &pieces.next().unwrap();
            let locals = &pieces.next().unwrap();
            self.current_function = Some(label.to_string());
            VMInstruction::Function(label.to_string(), locals.parse().unwrap())
        } else if lower_line.starts_with("call") {
            let mut pieces = line.split_whitespace().skip(1);
            let label = &pieces.next().unwrap();
            let arguments = &pieces.next().unwrap();
            VMInstruction::Call(VMFunctionCall {
                from: self
                    .current_function
                    .as_ref()
                    .expect("call not in a function!")
                    .clone(),
                target: label.to_string(),
                arguments: arguments.parse().unwrap(),
            })
        } else if lower_line.starts_with("return") {
            VMInstruction::Return
        } else {
            panic!("Unexpected instruction: {}", line)
        }
    }
}
