use crate::chunk;

pub enum InterpretResult {
    InterpretOk,
    InterpretCompileError,
    InterpretRuntimeError,
}

pub struct VM<'a> {
    chunk: &'a chunk::Chunk,
}

impl<'a> VM<'a> {
    pub fn init_vm(chunk: &'a chunk::Chunk) -> VM<'a> {
        VM {
            chunk: chunk
        }
    }
    
    pub fn set_chunk(&mut self, chunk: &'a chunk::Chunk) {
        self.chunk = chunk
    }

    pub fn interpret(&self) -> InterpretResult {
        self.run()
    }

    fn run(&self) -> InterpretResult {
        let mut ip = 0;
        loop {
            let (_, op) = &self.chunk.code[ip];
            let result = match op {
                chunk::OpCode::OpConstant(constant_index) => {
                    println!("{}", self.chunk.read_constant(*constant_index))
                }
                chunk::OpCode::OpReturn => return InterpretResult::InterpretOk,
            };
            ip += 1;
            result
        }
    }
}
