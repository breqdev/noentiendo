mod memory;
mod registers;
mod system;

fn main() {
    use memory::Memory;

    let memory = memory::BlockMemory::new();
    let mut system = Box::new(system::System::new(Box::new(memory)));

    system.write(0xFFFC, 0x00).unwrap();
    system.write(0xFFFD, 0x80).unwrap();

    system.write(0x8000, 0xA9).unwrap();
    system.write(0x8001, 0x01).unwrap();
    system.write(0x8002, 0x8D).unwrap();
    system.write(0x8003, 0x00).unwrap();
    system.write(0x8004, 0x02).unwrap();

    system.reset();

    system.tick();
    system.tick();

    println!("{:02X}", system.read(0x0200).unwrap());
}
