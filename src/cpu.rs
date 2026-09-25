pub struct Cpu {
    pub registers: Registers,
    pub stack: Stack,
}

impl Cpu {
    pub fn new() -> Self 
    {
        Self
        {
            registers: Registers::new(),
            stack: Stack::new(),
            current_rom: Vec::new(),
        }
    }
    /*
     * A step represent these actions:
     * - fetch
     * - decode
     * - execute
     *
     * Exemple idea: while(!interrupt) {cpu.step()}
     *
     */
    pub fn step(&mut self){} 
    
}
