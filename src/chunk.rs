pub enum OpCode {
    OpReturn
}

pub struct Chunk {
    code: Vec<OpCode>
}

impl Chunk {
    pub fn create_chunk() -> Chunk {
        Chunk {
            code: Vec::new()
        }
    }
    pub fn write_chunk(&mut self, opc: OpCode) {
        self.code.push(opc)
    }

    pub fn clear_chunk(&mut self) {
        self.code.clear();
        self.code.shrink_to_fit()
    }

    pub fn disassemble_chunk(&mut self, name: String) {
        println!("Chunk {}", name);
        
        let mut offset: usize = 0;
        while offset < self.code.len() {
            offset = disassemble_instruction(&self.code[offset], offset)
        } 
    }
}

fn disassemble_instruction(opc: &OpCode, offset: usize) -> usize {
    print!("{} ", offset);
    match opc {
        OpCode::OpReturn => {println!("OP_RETURN"); offset + 1}
    }
}
