// General idea:
//
// let mut emulator = Emulator::new();
//
// emulator.load(program);
// emulator.run();


pub struct Emulator {
   pub cpu: Cpu,
   pub ram: Ram,
}

impl Emulator {
    pub fn step(&mut self){} 
}
