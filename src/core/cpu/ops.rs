use crate::core::cpu::{Cpu, Register};
use crate::core::mem::AddressError;
use crate::core::mem::Bus;

pub type OpcodeHandler = fn(&mut Cpu, &mut dyn Bus, u8);

pub struct OpcodeDispatcher {
    table: [OpcodeHandler; 256],
}

impl OpcodeDispatcher {
    pub fn new() -> Self {
        let mut table = [illegal_opcode as OpcodeHandler; 256];

        // 0x00 - 0x3F : instructions diverses (decode par opcode)
        table[0x00..0x40].fill(low_opcode_handler);

        // 0x40 - 0x7F : LD r,r
        table[0x40..0x80].fill(ld_matrix_handler);
        table[0x76] = halt_handler;

        // 0x80 - 0xBF : ALU A,r
        table[0x80..0xC0].fill(alu_handler);

        // 0xC0 - 0xFF : instructions de contrôle, stack, immediates...
        table[0xC0..0x100].fill(control_opcode_handler);

        Self { table }
    }

    pub fn execute(&self, cpu: &mut Cpu, bus: &mut dyn Bus, opcode: u8) {
        (self.table[opcode as usize])(cpu, bus, opcode);
    }
}

// ============================================================
// 0x40 - 0x7F : LD r,r
// ============================================================

fn ld_matrix_handler(cpu: &mut Cpu, bus: &mut dyn Bus, opcode: u8) {
    let dst = (opcode >> 3) & 0x07;
    let src = opcode & 0x07;

    let value = read_r8(cpu, bus, src);
    write_r8(cpu, bus, dst, value);
}

// ============================================================
// 0x80 - 0xBF : ALU A,r
// ============================================================

fn alu_handler(cpu: &mut Cpu, bus: &mut dyn Bus, opcode: u8) {
    let operation = (opcode >> 3) & 0x07;
    let operand = opcode & 0x07;

    let value = read_r8(cpu, bus, operand);

    match operation {
        0 => add_a(cpu, value),
        1 => adc_a(cpu, value),
        2 => sub_a(cpu, value),
        3 => sbc_a(cpu, value),
        4 => and_a(cpu, value),
        5 => xor_a(cpu, value),
        6 => or_a(cpu, value),
        7 => cp_a(cpu, value),
        _ => unreachable!(),
    }
}

// ============================================================
// 0x00 - 0x3F : instructions diverses
// ============================================================

fn low_opcode_handler(cpu: &mut Cpu, bus: &mut dyn Bus, opcode: u8) {
    match opcode {
        0x00 => nop(cpu, bus, opcode),
        0x01 => ld_rr_d16(cpu, bus, opcode),
        0x02 => ld_indirect_rr_a(cpu, bus, opcode),
        0x03 => inc_rr(cpu, bus, opcode),
        0x04 => inc_r8(cpu, bus, opcode),
        0x05 => dec_r8(cpu, bus, opcode),
        0x06 => ld_r8_d8(cpu, bus, opcode),
        0x07 => rotate_a(cpu, bus, opcode),

        0x08 => ld_sp_indirect_d16(cpu, bus),
        0x09 => add_hl_rr(cpu, bus, opcode),
        0x0A => ld_a_indirect_rr(cpu, bus, opcode),
        0x0B => dec_rr(cpu, bus, opcode),
        0x0C => inc_r8(cpu, bus, opcode),
        0x0D => dec_r8(cpu, bus, opcode),
        0x0E => ld_r8_d8(cpu, bus, opcode),
        0x0F => rotate_a(cpu, bus, opcode),

        0x10 => stop_handler(cpu, bus),
        0x11 => ld_rr_d16(cpu, bus, opcode),
        0x12 => ld_indirect_rr_a(cpu, bus, opcode),
        0x13 => inc_rr(cpu, bus, opcode),
        0x14 => inc_r8(cpu, bus, opcode),
        0x15 => dec_r8(cpu, bus, opcode),
        0x16 => ld_r8_d8(cpu, bus, opcode),
        0x17 => rotate_a(cpu, bus, opcode),

        0x18 => jr(cpu, bus),
        0x19 => add_hl_rr(cpu, bus, opcode),
        0x1A => ld_a_indirect_rr(cpu, bus, opcode),
        0x1B => dec_rr(cpu, bus, opcode),
        0x1C => inc_r8(cpu, bus, opcode),
        0x1D => dec_r8(cpu, bus, opcode),
        0x1E => ld_r8_d8(cpu, bus, opcode),
        0x1F => rotate_a(cpu, bus, opcode),

        0x20 => jr_cond(cpu, bus, opcode),
        0x21 => ld_rr_d16(cpu, bus, opcode),
        0x22 => ld_hl_inc_a(cpu, bus),
        0x23 => inc_rr(cpu, bus, opcode),
        0x24 => inc_r8(cpu, bus, opcode),
        0x25 => dec_r8(cpu, bus, opcode),
        0x26 => ld_r8_d8(cpu, bus, opcode),
        0x27 => daa(cpu, bus),

        0x28 => jr_cond(cpu, bus, opcode),
        0x29 => add_hl_rr(cpu, bus, opcode),
        0x2A => ld_a_hl_inc(cpu, bus),
        0x2B => dec_rr(cpu, bus, opcode),
        0x2C => inc_r8(cpu, bus, opcode),
        0x2D => dec_r8(cpu, bus, opcode),
        0x2E => ld_r8_d8(cpu, bus, opcode),
        0x2F => cpl(cpu, bus),

        0x30 => jr_cond(cpu, bus, opcode),
        0x31 => ld_rr_d16(cpu, bus, opcode),
        0x32 => ld_hl_dec_a(cpu, bus),
        0x33 => inc_rr(cpu, bus, opcode),
        0x34 => inc_r8(cpu, bus, opcode),
        0x35 => dec_r8(cpu, bus, opcode),
        0x36 => ld_r8_d8(cpu, bus, opcode),
        0x37 => scf(cpu, bus),

        0x38 => jr_cond(cpu, bus, opcode),
        0x39 => add_hl_rr(cpu, bus, opcode),
        0x3A => ld_a_hl_dec(cpu, bus),
        0x3B => dec_rr(cpu, bus, opcode),
        0x3C => inc_r8(cpu, bus, opcode),
        0x3D => dec_r8(cpu, bus, opcode),
        0x3E => ld_r8_d8(cpu, bus, opcode),
        0x3F => ccf(cpu, bus),

        _ => unreachable!(),
    }
}

// ============================================================
// 0xC0 - 0xFF : control, stack, immediates, etc.
// ============================================================

fn control_opcode_handler(cpu: &mut Cpu, bus: &mut dyn Bus, opcode: u8) {
    match opcode {
        0xC0 => ret_cond(cpu, bus, opcode),
        0xC1 => pop_rr(cpu, bus, opcode),
        0xC2 => jp_cond(cpu, bus, opcode),
        0xC3 => jp(cpu, bus),
        0xC4 => call_cond(cpu, bus, opcode),
        0xC5 => push_rr(cpu, bus, opcode),
        0xC6 => alu_immediate(cpu, bus, opcode),
        0xC7 => rst(cpu, bus, opcode),

        0xC8 => ret_cond(cpu, bus, opcode),
        0xC9 => ret(cpu, bus),
        0xCA => jp_cond(cpu, bus, opcode),
        0xCB => prefix_cb(cpu, bus),
        0xCC => call_cond(cpu, bus, opcode),
        0xCD => call(cpu, bus),
        0xCE => alu_immediate(cpu, bus, opcode),
        0xCF => rst(cpu, bus, opcode),

        0xD0 => ret_cond(cpu, bus, opcode),
        0xD1 => pop_rr(cpu, bus, opcode),
        0xD2 => jp_cond(cpu, bus, opcode),
        0xD3 => illegal_opcode(cpu, bus, opcode),
        0xD4 => call_cond(cpu, bus, opcode),
        0xD5 => push_rr(cpu, bus, opcode),
        0xD6 => alu_immediate(cpu, bus, opcode),
        0xD7 => rst(cpu, bus, opcode),

        0xD8 => ret_cond(cpu, bus, opcode),
        0xD9 => reti(cpu, bus),
        0xDA => jp_cond(cpu, bus, opcode),
        0xDB => illegal_opcode(cpu, bus, opcode),
        0xDC => call_cond(cpu, bus, opcode),
        0xDD => illegal_opcode(cpu, bus, opcode),
        0xDE => alu_immediate(cpu, bus, opcode),
        0xDF => rst(cpu, bus, opcode),

        0xE0 => ldh(cpu, bus, opcode),
        0xE1 => pop_rr(cpu, bus, opcode),
        0xE2 => ld_c_a(cpu, bus),
        0xE3 => illegal_opcode(cpu, bus, opcode),
        0xE4 => illegal_opcode(cpu, bus, opcode),
        0xE5 => push_rr(cpu, bus, opcode),
        0xE6 => alu_immediate(cpu, bus, opcode),
        0xE7 => rst(cpu, bus, opcode),

        0xE8 => add_sp_r8(cpu, bus),
        0xE9 => jp_hl(cpu, bus),
        0xEA => ld_a16_a(cpu, bus),
        0xEB => illegal_opcode(cpu, bus, opcode),
        0xEC => illegal_opcode(cpu, bus, opcode),
        0xED => illegal_opcode(cpu, bus, opcode),
        0xEE => alu_immediate(cpu, bus, opcode),
        0xEF => rst(cpu, bus, opcode),

        0xF0 => ldh(cpu, bus, opcode),
        0xF1 => pop_rr(cpu, bus, opcode),
        0xF2 => ld_a_c(cpu, bus),
        0xF3 => di(cpu, bus),
        0xF4 => illegal_opcode(cpu, bus, opcode),
        0xF5 => push_rr(cpu, bus, opcode),
        0xF6 => alu_immediate(cpu, bus, opcode),
        0xF7 => rst(cpu, bus, opcode),

        0xF8 => ld_hl_sp_r8(cpu, bus),
        0xF9 => ld_sp_hl(cpu, bus),
        0xFA => ld_a_a16(cpu, bus),
        0xFB => ei(cpu, bus),
        0xFC => illegal_opcode(cpu, bus, opcode),
        0xFD => illegal_opcode(cpu, bus, opcode),
        0xFE => alu_immediate(cpu, bus, opcode),
        0xFF => rst(cpu, bus, opcode),

        _ => unreachable!(),
    }
}

// ============================================================
// Helpers : 8 bits registers (B,C,D,E,H,L,(HL),A)
// ============================================================

fn read_r8(cpu: &Cpu, bus: &mut dyn Bus, index: u8) -> u8 {
    if index == 6 {
        bus.read(cpu.hl);
    }
    match index {
        0 => cpu.b(),
        1 => cpu.c(),
        2 => cpu.d(),
        3 => cpu.e(),
        4 => cpu.h(),
        5 => cpu.l(),
        7 => cpu.a(),
        _ => unreachable!(),
    }
}

fn write_r8(cpu: &mut Cpu, bus: &mut dyn Bus, index: u8, value: u8) {
    if index == 6 {
        bus.write(cpu.hl, value);
    }

    match index {
        0 => cpu.bc.set_hi(value), // B
        1 => cpu.bc.set_lo(value), // C
        2 => cpu.de.set_hi(value), // D
        3 => cpu.de.set_lo(value), // E
        4 => cpu.hl.set_hi(value), // H
        5 => cpu.hl.set_lo(value), // L
        7 => cpu.af.set_hi(value), // A
        _ => unreachable!("Invalid 8-bit register index: {index}"),
    }
}

// ============================================================
// TODO: Handlers
// ============================================================

fn illegal_opcode(_cpu: &mut Cpu, _bus: &mut dyn Bus, _opcode: u8) {}
fn halt_handler(_cpu: &mut Cpu, _bus: &mut dyn Bus, _opcode: u8) {}
fn nop(_cpu: &mut Cpu, _bus: &mut dyn Bus, _opcode: u8) {}
fn stop_handler(_cpu: &mut Cpu, _bus: &mut dyn Bus) {}
fn prefix_cb(_cpu: &mut Cpu, _bus: &mut dyn Bus) {}

fn add_a(_cpu: &mut Cpu, _value: u8) {}
fn adc_a(_cpu: &mut Cpu, _value: u8) {}
fn sub_a(_cpu: &mut Cpu, _value: u8) {}
fn sbc_a(_cpu: &mut Cpu, _value: u8) {}
fn and_a(_cpu: &mut Cpu, _value: u8) {}
fn xor_a(_cpu: &mut Cpu, _value: u8) {}
fn or_a(_cpu: &mut Cpu, _value: u8) {}
fn cp_a(_cpu: &mut Cpu, _value: u8) {}

// ============================================================
// handlers left
// ============================================================

fn ld_rr_d16(_cpu: &mut Cpu, _bus: &mut dyn Bus, _opcode: u8) {}
fn ld_indirect_rr_a(_cpu: &mut Cpu, _bus: &mut dyn Bus, _opcode: u8) {}
fn inc_rr(_cpu: &mut Cpu, _bus: &mut dyn Bus, _opcode: u8) {}
fn dec_rr(_cpu: &mut Cpu, _bus: &mut dyn Bus, _opcode: u8) {}
fn inc_r8(_cpu: &mut Cpu, _bus: &mut dyn Bus, _opcode: u8) {}
fn dec_r8(_cpu: &mut Cpu, _bus: &mut dyn Bus, _opcode: u8) {}
fn ld_r8_d8(_cpu: &mut Cpu, _bus: &mut dyn Bus, _opcode: u8) {}
fn rotate_a(_cpu: &mut Cpu, _bus: &mut dyn Bus, _opcode: u8) {}
fn ld_sp_indirect_d16(_cpu: &mut Cpu, _bus: &mut dyn Bus) {}
fn add_hl_rr(_cpu: &mut Cpu, _bus: &mut dyn Bus, _opcode: u8) {}
fn ld_a_indirect_rr(_cpu: &mut Cpu, _bus: &mut dyn Bus, _opcode: u8) {}
fn jr(_cpu: &mut Cpu, _bus: &mut dyn Bus) {}
fn jr_cond(_cpu: &mut Cpu, _bus: &mut dyn Bus, _opcode: u8) {}
fn ld_hl_inc_a(_cpu: &mut Cpu, _bus: &mut dyn Bus) {}
fn ld_a_hl_inc(_cpu: &mut Cpu, _bus: &mut dyn Bus) {}
fn ld_hl_dec_a(_cpu: &mut Cpu, _bus: &mut dyn Bus) {}
fn ld_a_hl_dec(_cpu: &mut Cpu, _bus: &mut dyn Bus) {}
fn daa(_cpu: &mut Cpu, _bus: &mut dyn Bus) {}
fn cpl(_cpu: &mut Cpu, _bus: &mut dyn Bus) {}
fn scf(_cpu: &mut Cpu, _bus: &mut dyn Bus) {}
fn ccf(_cpu: &mut Cpu, _bus: &mut dyn Bus) {}
fn ret_cond(_cpu: &mut Cpu, _bus: &mut dyn Bus, _opcode: u8) {}
fn pop_rr(_cpu: &mut Cpu, _bus: &mut dyn Bus, _opcode: u8) {}
fn jp_cond(_cpu: &mut Cpu, _bus: &mut dyn Bus, _opcode: u8) {}
fn jp(_cpu: &mut Cpu, _bus: &mut dyn Bus) {}
fn call_cond(_cpu: &mut Cpu, _bus: &mut dyn Bus, _opcode: u8) {}
fn push_rr(_cpu: &mut Cpu, _bus: &mut dyn Bus, _opcode: u8) {}
fn alu_immediate(_cpu: &mut Cpu, _bus: &mut dyn Bus, _opcode: u8) {}
fn rst(_cpu: &mut Cpu, _bus: &mut dyn Bus, _opcode: u8) {}
fn ret(_cpu: &mut Cpu, _bus: &mut dyn Bus) {}
fn call(_cpu: &mut Cpu, _bus: &mut dyn Bus) {}
fn reti(_cpu: &mut Cpu, _bus: &mut dyn Bus) {}
fn ldh(_cpu: &mut Cpu, _bus: &mut dyn Bus, _opcode: u8) {}
fn ld_c_a(_cpu: &mut Cpu, _bus: &mut dyn Bus) {}
fn add_sp_r8(_cpu: &mut Cpu, _bus: &mut dyn Bus) {}
fn jp_hl(_cpu: &mut Cpu, _bus: &mut dyn Bus) {}
fn ld_a16_a(_cpu: &mut Cpu, _bus: &mut dyn Bus) {}
fn ld_a_c(_cpu: &mut Cpu, _bus: &mut dyn Bus) {}
fn di(_cpu: &mut Cpu, _bus: &mut dyn Bus) {}
fn ld_hl_sp_r8(_cpu: &mut Cpu, _bus: &mut dyn Bus) {}
fn ld_sp_hl(_cpu: &mut Cpu, _bus: &mut dyn Bus) {}
fn ld_a_a16(_cpu: &mut Cpu, _bus: &mut dyn Bus) {}
fn ei(_cpu: &mut Cpu, _bus: &mut dyn Bus) {}
