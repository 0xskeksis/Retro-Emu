use crate::core::Model;
use super::Bus;

#[test]
fn accesses_and_internal_cycles_share_the_scheduler() {
    let mut bus = Bus::new(Model::Dmg);

    bus.read8(0xFF04);
    bus.write8(0xFF06, 0x42);
    bus.idle_mcycle();

    assert_eq!(bus.elapsed_tcycles(), 12);
    assert_eq!(bus.elapsed_ppu_dots(), 12);
}

#[test]
fn timer_interrupt_reaches_if() {
    let mut bus = Bus::new(Model::Dmg);

    bus.write8(0xFF06, 0x42); // TMA
    bus.write8(0xFF05, 0xFF); // TIMA
    bus.write8(0xFF07, 0b101); // Enable fastest selection

    for _ in 0..5 {
        bus.idle_mcycle();
    }

    assert_ne!(bus.mapped_read(0xFF0F) & 0b100, 0);
    assert_eq!(bus.mapped_read(0xFF05), 0x42);
}
