// General idea:
//
// let mut emulator = Emulator::new();
//
// emulator.load(program);
// emulator.run();


pub struct Emulator {
   pub cpu: Cpu,
   pub ram: Ram,
   pub cartridge: Cartridge,
}

impl Emulator {
    fn new() -> Self 
    {
       Self 
       {
        cpu: Cpu::new(),
        Ram: Ram::new(),
       } 
    }

    /*
     * Load a ROM, after this function every operations will be on the loaded ROM.
     */
    fn load(filepath: &str){
      // fill cartridge struct  (header + rom raw data)
    }

    fn run(){

    }
}
