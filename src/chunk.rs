pub enum OpCode {
    OpReturn,
    OpConstant(usize),
}

pub struct Chunk {
    pub code: Vec<(u32, OpCode)>,
    constants: ValueArray,
}

pub type Value = f64;

pub struct ValueArray {
    values: Vec<Value>,
}

impl Chunk {
    pub fn create_chunk() -> Chunk {
        Chunk {
            code: Vec::new(),
            constants: ValueArray::create_value_array(),
        }
    }

    pub fn write_chunk(&mut self, opc: OpCode, l: u32) {
        self.code.push((l, opc))
    }

    pub fn clear_chunk(&mut self) {
        self.code.clear();
        self.code.shrink_to_fit();
        self.constants.clear_value_array()
    }

    pub fn disassemble_chunk(&mut self, name: String) {
        println!("Chunk {}", name);
        let mut offset: usize = 0;
        while offset < self.code.len() {
            offset = disassemble_instruction(&self, offset)
        }
    }

    pub fn add_constant(&mut self, v: Value) -> usize {
        self.constants.write_value(v);
        self.constants.values.len() - 1
    }

    pub fn read_constant(&self, index: usize) -> f64 {
        self.constants.values[index]
    }
}

impl std::fmt::Debug for OpCode {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl ValueArray {
    fn create_value_array() -> ValueArray {
        ValueArray { values: Vec::new() }
    }
    fn write_value(&mut self, v: Value) {
        self.values.push(v)
    }
    fn clear_value_array(&mut self) {
        self.values.clear();
        self.values.shrink_to_fit()
    }
}

fn disassemble_instruction(chunk: &Chunk, offset: usize) -> usize {
    let (line, opc) = &chunk.code[offset];
    print!("{:4} {:4} ", offset, line);
    match opc {
        OpCode::OpReturn => simple_instruction("OP_RETURN".to_string(), offset),
        OpCode::OpConstant(constant_index) => {
            constant_instruction("OP_CONSTANT".to_string(), chunk, *constant_index, offset)
        }
    }
}

fn value_to_string(v: Value) -> String {
    format!("{}", v)
}

fn simple_instruction(name: String, offset: usize) -> usize {
    println!("{}", name);
    offset + 1
}

fn constant_instruction(
    name: String,
    chunk: &Chunk,
    constant_index: usize,
    offset: usize,
) -> usize {
    let v: Value = chunk.constants.values[constant_index];
    println!("{} {} '{}'", name, constant_index, value_to_string(v));
    offset + 1
}
