use std::marker::PhantomData;
use crate::core::cpu::{Cpu, Register};
use crate::core::mem::{Bus, Result};

pub type OpcodeHandler<B> = fn(&mut Cpu, &mut B, u8) -> Result<()>;

pub struct Opcodes<B: Bus>(PhantomData<B>);

impl<B: Bus> Opcodes<B> {
    pub const TABLE: [OpcodeHandler<B>; 256] = Self::build_table();
    pub const CB_TABLE: [OpcodeHandler<B>; 256] = Self::build_cb_table();

    #[inline(always)]
    pub fn execute(cpu: &mut Cpu, bus: &mut B, opcode: u8) -> Result<()> {
        let table: &[OpcodeHandler<B>; 256] = &Self::TABLE;
        table[opcode as usize](cpu, bus, opcode)
    }

const fn build_cb_table() -> [OpcodeHandler<B>; 256] {
        let mut table = [illegal::<B> as OpcodeHandler<B>; 256];

        // ---------------- 0x00 - 0x3F : rotates / shifts ----------------
        table[0x00] = rlc; table[0x01] = rlc; table[0x02] = rlc; table[0x03] = rlc; table[0x04] = rlc; table[0x05] = rlc; table[0x06] = rlc; table[0x07] = rlc;
        table[0x08] = rrc; table[0x09] = rrc; table[0x0A] = rrc; table[0x0B] = rrc; table[0x0C] = rrc; table[0x0D] = rrc; table[0x0E] = rrc; table[0x0F] = rrc;
        table[0x10] = rl;  table[0x11] = rl;  table[0x12] = rl;  table[0x13] = rl;  table[0x14] = rl;  table[0x15] = rl;  table[0x16] = rl;  table[0x17] = rl;
        table[0x18] = rr;  table[0x19] = rr;  table[0x1A] = rr;  table[0x1B] = rr;  table[0x1C] = rr;  table[0x1D] = rr;  table[0x1E] = rr;  table[0x1F] = rr;
        table[0x20] = sla; table[0x21] = sla; table[0x22] = sla; table[0x23] = sla; table[0x24] = sla; table[0x25] = sla; table[0x26] = sla; table[0x27] = sla;
        table[0x28] = sra; table[0x29] = sra; table[0x2A] = sra; table[0x2B] = sra; table[0x2C] = sra; table[0x2D] = sra; table[0x2E] = sra; table[0x2F] = sra;
        table[0x30] = swap; table[0x31] = swap; table[0x32] = swap; table[0x33] = swap; table[0x34] = swap; table[0x35] = swap; table[0x36] = swap; table[0x37] = swap;
        table[0x38] = srl; table[0x39] = srl; table[0x3A] = srl; table[0x3B] = srl; table[0x3C] = srl; table[0x3D] = srl; table[0x3E] = srl; table[0x3F] = srl;

        // ---------------- 0x40 - 0x7F : BIT b,r ----------------
        table[0x40] = bit_r; table[0x41] = bit_r; table[0x42] = bit_r; table[0x43] = bit_r; table[0x44] = bit_r; table[0x45] = bit_r; table[0x46] = bit_r; table[0x47] = bit_r;
        table[0x48] = bit_r; table[0x49] = bit_r; table[0x4A] = bit_r; table[0x4B] = bit_r; table[0x4C] = bit_r; table[0x4D] = bit_r; table[0x4E] = bit_r; table[0x4F] = bit_r;
        table[0x50] = bit_r; table[0x51] = bit_r; table[0x52] = bit_r; table[0x53] = bit_r; table[0x54] = bit_r; table[0x55] = bit_r; table[0x56] = bit_r; table[0x57] = bit_r;
        table[0x58] = bit_r; table[0x59] = bit_r; table[0x5A] = bit_r; table[0x5B] = bit_r; table[0x5C] = bit_r; table[0x5D] = bit_r; table[0x5E] = bit_r; table[0x5F] = bit_r;
        table[0x60] = bit_r; table[0x61] = bit_r; table[0x62] = bit_r; table[0x63] = bit_r; table[0x64] = bit_r; table[0x65] = bit_r; table[0x66] = bit_r; table[0x67] = bit_r;
        table[0x68] = bit_r; table[0x69] = bit_r; table[0x6A] = bit_r; table[0x6B] = bit_r; table[0x6C] = bit_r; table[0x6D] = bit_r; table[0x6E] = bit_r; table[0x6F] = bit_r;
        table[0x70] = bit_r; table[0x71] = bit_r; table[0x72] = bit_r; table[0x73] = bit_r; table[0x74] = bit_r; table[0x75] = bit_r; table[0x76] = bit_r; table[0x77] = bit_r;
        table[0x78] = bit_r; table[0x79] = bit_r; table[0x7A] = bit_r; table[0x7B] = bit_r; table[0x7C] = bit_r; table[0x7D] = bit_r; table[0x7E] = bit_r; table[0x7F] = bit_r;

        // ---------------- 0x80 - 0xBF : RES b,r ----------------
        table[0x80] = res_r; table[0x81] = res_r; table[0x82] = res_r; table[0x83] = res_r; table[0x84] = res_r; table[0x85] = res_r; table[0x86] = res_r; table[0x87] = res_r;
        table[0x88] = res_r; table[0x89] = res_r; table[0x8A] = res_r; table[0x8B] = res_r; table[0x8C] = res_r; table[0x8D] = res_r; table[0x8E] = res_r; table[0x8F] = res_r;
        table[0x90] = res_r; table[0x91] = res_r; table[0x92] = res_r; table[0x93] = res_r; table[0x94] = res_r; table[0x95] = res_r; table[0x96] = res_r; table[0x97] = res_r;
        table[0x98] = res_r; table[0x99] = res_r; table[0x9A] = res_r; table[0x9B] = res_r; table[0x9C] = res_r; table[0x9D] = res_r; table[0x9E] = res_r; table[0x9F] = res_r;
        table[0xA0] = res_r; table[0xA1] = res_r; table[0xA2] = res_r; table[0xA3] = res_r; table[0xA4] = res_r; table[0xA5] = res_r; table[0xA6] = res_r; table[0xA7] = res_r;
        table[0xA8] = res_r; table[0xA9] = res_r; table[0xAA] = res_r; table[0xAB] = res_r; table[0xAC] = res_r; table[0xAD] = res_r; table[0xAE] = res_r; table[0xAF] = res_r;
        table[0xB0] = res_r; table[0xB1] = res_r; table[0xB2] = res_r; table[0xB3] = res_r; table[0xB4] = res_r; table[0xB5] = res_r; table[0xB6] = res_r; table[0xB7] = res_r;
        table[0xB8] = res_r; table[0xB9] = res_r; table[0xBA] = res_r; table[0xBB] = res_r; table[0xBC] = res_r; table[0xBD] = res_r; table[0xBE] = res_r; table[0xBF] = res_r;

        // ---------------- 0xC0 - 0xFF : SET b,r ----------------
        table[0xC0] = set_r; table[0xC1] = set_r; table[0xC2] = set_r; table[0xC3] = set_r; table[0xC4] = set_r; table[0xC5] = set_r; table[0xC6] = set_r; table[0xC7] = set_r;
        table[0xC8] = set_r; table[0xC9] = set_r; table[0xCA] = set_r; table[0xCB] = set_r; table[0xCC] = set_r; table[0xCD] = set_r; table[0xCE] = set_r; table[0xCF] = set_r;
        table[0xD0] = set_r; table[0xD1] = set_r; table[0xD2] = set_r; table[0xD3] = set_r; table[0xD4] = set_r; table[0xD5] = set_r; table[0xD6] = set_r; table[0xD7] = set_r;
        table[0xD8] = set_r; table[0xD9] = set_r; table[0xDA] = set_r; table[0xDB] = set_r; table[0xDC] = set_r; table[0xDD] = set_r; table[0xDE] = set_r; table[0xDF] = set_r;
        table[0xE0] = set_r; table[0xE1] = set_r; table[0xE2] = set_r; table[0xE3] = set_r; table[0xE4] = set_r; table[0xE5] = set_r; table[0xE6] = set_r; table[0xE7] = set_r;
        table[0xE8] = set_r; table[0xE9] = set_r; table[0xEA] = set_r; table[0xEB] = set_r; table[0xEC] = set_r; table[0xED] = set_r; table[0xEE] = set_r; table[0xEF] = set_r;
        table[0xF0] = set_r; table[0xF1] = set_r; table[0xF2] = set_r; table[0xF3] = set_r; table[0xF4] = set_r; table[0xF5] = set_r; table[0xF6] = set_r; table[0xF7] = set_r;
        table[0xF8] = set_r; table[0xF9] = set_r; table[0xFA] = set_r; table[0xFB] = set_r; table[0xFC] = set_r; table[0xFD] = set_r; table[0xFE] = set_r; table[0xFF] = set_r;

        table
}

const fn build_table() -> [OpcodeHandler<B>; 256] {
    let mut table = [illegal::<B> as OpcodeHandler<B>; 256];

    // ---------------- 0x00 - 0x3F ----------------
    table[0x00] = nop;
    table[0x01] = ld_rr_d16;
    table[0x02] = ld_indirect_rr_a;
    table[0x03] = inc_rr;
    table[0x04] = inc_r8;
    table[0x05] = dec_r8;
    table[0x06] = ld_r8_d8;
    table[0x07] = rlca;

    table[0x08] = ld_a16_sp;
    table[0x09] = add_hl_rr;
    table[0x0A] = ld_a_indirect_rr;
    table[0x0B] = dec_rr;
    table[0x0C] = inc_r8;
    table[0x0D] = dec_r8;
    table[0x0E] = ld_r8_d8;
    table[0x0F] = rrca;

    table[0x10] = stop;
    table[0x11] = ld_rr_d16;
    table[0x12] = ld_indirect_rr_a;
    table[0x13] = inc_rr;
    table[0x14] = inc_r8;
    table[0x15] = dec_r8;
    table[0x16] = ld_r8_d8;
    table[0x17] = rla;

    table[0x18] = jr;
    table[0x19] = add_hl_rr;
    table[0x1A] = ld_a_indirect_rr;
    table[0x1B] = dec_rr;
    table[0x1C] = inc_r8;
    table[0x1D] = dec_r8;
    table[0x1E] = ld_r8_d8;
    table[0x1F] = rra;

    table[0x20] = jr_cond;
    table[0x21] = ld_rr_d16;
    table[0x22] = ld_hl_inc_a;
    table[0x23] = inc_rr;
    table[0x24] = inc_r8;
    table[0x25] = dec_r8;
    table[0x26] = ld_r8_d8;
    table[0x27] = daa;

    table[0x28] = jr_cond;
    table[0x29] = add_hl_rr;
    table[0x2A] = ld_a_hl_inc;
    table[0x2B] = dec_rr;
    table[0x2C] = inc_r8;
    table[0x2D] = dec_r8;
    table[0x2E] = ld_r8_d8;
    table[0x2F] = cpl;

    table[0x30] = jr_cond;
    table[0x31] = ld_rr_d16;
    table[0x32] = ld_hl_dec_a;
    table[0x33] = inc_rr;
    table[0x34] = inc_r8;
    table[0x35] = dec_r8;
    table[0x36] = ld_r8_d8;
    table[0x37] = scf;

    table[0x38] = jr_cond;
    table[0x39] = add_hl_rr;
    table[0x3A] = ld_a_hl_dec;
    table[0x3B] = dec_rr;
    table[0x3C] = inc_r8;
    table[0x3D] = dec_r8;
    table[0x3E] = ld_r8_d8;
    table[0x3F] = ccf;

    // ---------------- 0x40 - 0x7F : LD r,r ----------------
    table[0x40] = ld_r_r; table[0x41] = ld_r_r; table[0x42] = ld_r_r; table[0x43] = ld_r_r;
    table[0x44] = ld_r_r; table[0x45] = ld_r_r; table[0x46] = ld_r_r; table[0x47] = ld_r_r;
    table[0x48] = ld_r_r; table[0x49] = ld_r_r; table[0x4A] = ld_r_r; table[0x4B] = ld_r_r;
    table[0x4C] = ld_r_r; table[0x4D] = ld_r_r; table[0x4E] = ld_r_r; table[0x4F] = ld_r_r;

    table[0x50] = ld_r_r; table[0x51] = ld_r_r; table[0x52] = ld_r_r; table[0x53] = ld_r_r;
    table[0x54] = ld_r_r; table[0x55] = ld_r_r; table[0x56] = ld_r_r; table[0x57] = ld_r_r;
    table[0x58] = ld_r_r; table[0x59] = ld_r_r; table[0x5A] = ld_r_r; table[0x5B] = ld_r_r;
    table[0x5C] = ld_r_r; table[0x5D] = ld_r_r; table[0x5E] = ld_r_r; table[0x5F] = ld_r_r;

    table[0x60] = ld_r_r; table[0x61] = ld_r_r; table[0x62] = ld_r_r; table[0x63] = ld_r_r;
    table[0x64] = ld_r_r; table[0x65] = ld_r_r; table[0x66] = ld_r_r; table[0x67] = ld_r_r;
    table[0x68] = ld_r_r; table[0x69] = ld_r_r; table[0x6A] = ld_r_r; table[0x6B] = ld_r_r;
    table[0x6C] = ld_r_r; table[0x6D] = ld_r_r; table[0x6E] = ld_r_r; table[0x6F] = ld_r_r;

    table[0x70] = ld_r_r; table[0x71] = ld_r_r; table[0x72] = ld_r_r; table[0x73] = ld_r_r;
    table[0x74] = ld_r_r; table[0x75] = ld_r_r; table[0x76] = halt;  table[0x77] = ld_r_r;
    table[0x78] = ld_r_r; table[0x79] = ld_r_r; table[0x7A] = ld_r_r; table[0x7B] = ld_r_r;
    table[0x7C] = ld_r_r; table[0x7D] = ld_r_r; table[0x7E] = ld_r_r; table[0x7F] = ld_r_r;

    // ---------------- 0x80 - 0xBF : ALU A,r ----------------
    table[0x80] = alu_r; table[0x81] = alu_r; table[0x82] = alu_r; table[0x83] = alu_r; // ADD
    table[0x84] = alu_r; table[0x85] = alu_r; table[0x86] = alu_r; table[0x87] = alu_r;
    table[0x88] = alu_r; table[0x89] = alu_r; table[0x8A] = alu_r; table[0x8B] = alu_r; // ADC
    table[0x8C] = alu_r; table[0x8D] = alu_r; table[0x8E] = alu_r; table[0x8F] = alu_r;

    table[0x90] = alu_r; table[0x91] = alu_r; table[0x92] = alu_r; table[0x93] = alu_r; // SUB
    table[0x94] = alu_r; table[0x95] = alu_r; table[0x96] = alu_r; table[0x97] = alu_r;
    table[0x98] = alu_r; table[0x99] = alu_r; table[0x9A] = alu_r; table[0x9B] = alu_r; // SBC
    table[0x9C] = alu_r; table[0x9D] = alu_r; table[0x9E] = alu_r; table[0x9F] = alu_r;

    table[0xA0] = alu_r; table[0xA1] = alu_r; table[0xA2] = alu_r; table[0xA3] = alu_r; // AND
    table[0xA4] = alu_r; table[0xA5] = alu_r; table[0xA6] = alu_r; table[0xA7] = alu_r;
    table[0xA8] = alu_r; table[0xA9] = alu_r; table[0xAA] = alu_r; table[0xAB] = alu_r; // XOR
    table[0xAC] = alu_r; table[0xAD] = alu_r; table[0xAE] = alu_r; table[0xAF] = alu_r;

    table[0xB0] = alu_r; table[0xB1] = alu_r; table[0xB2] = alu_r; table[0xB3] = alu_r; // OR
    table[0xB4] = alu_r; table[0xB5] = alu_r; table[0xB6] = alu_r; table[0xB7] = alu_r;
    table[0xB8] = alu_r; table[0xB9] = alu_r; table[0xBA] = alu_r; table[0xBB] = alu_r; // CP
    table[0xBC] = alu_r; table[0xBD] = alu_r; table[0xBE] = alu_r; table[0xBF] = alu_r;

    // ---------------- 0xC0 - 0xFF ----------------
    table[0xC0] = ret_cond;
    table[0xC1] = pop_rr;
    table[0xC2] = jp_cond;
    table[0xC3] = jp;
    table[0xC4] = call_cond;
    table[0xC5] = push_rr;
    table[0xC6] = alu_imm;
    table[0xC7] = rst;

    table[0xC8] = ret_cond;
    table[0xC9] = ret;
    table[0xCA] = jp_cond;
    table[0xCB] = prefix_cb;
    table[0xCC] = call_cond;
    table[0xCD] = call;
    table[0xCE] = alu_imm;
    table[0xCF] = rst;

    table[0xD0] = ret_cond;
    table[0xD1] = pop_rr;
    table[0xD2] = jp_cond;
    table[0xD3] = illegal;
    table[0xD4] = call_cond;
    table[0xD5] = push_rr;
    table[0xD6] = alu_imm;
    table[0xD7] = rst;

    table[0xD8] = ret_cond;
    table[0xD9] = reti;
    table[0xDA] = jp_cond;
    table[0xDB] = illegal;
    table[0xDC] = call_cond;
    table[0xDD] = illegal;
    table[0xDE] = alu_imm;
    table[0xDF] = rst;

    table[0xE0] = ldh_a8_a;
    table[0xE1] = pop_rr;
    table[0xE2] = ld_c_a;
    table[0xE3] = illegal;
    table[0xE4] = illegal;
    table[0xE5] = push_rr;
    table[0xE6] = alu_imm;
    table[0xE7] = rst;

    table[0xE8] = add_sp_r8;
    table[0xE9] = jp_hl;
    table[0xEA] = ld_a16_a;
    table[0xEB] = illegal;
    table[0xEC] = illegal;
    table[0xED] = illegal;
    table[0xEE] = alu_imm;
    table[0xEF] = rst;

    table[0xF0] = ldh_a_a8;
    table[0xF1] = pop_rr;
    table[0xF2] = ld_a_c;
    table[0xF3] = di;
    table[0xF4] = illegal;
    table[0xF5] = push_rr;
    table[0xF6] = alu_imm;
    table[0xF7] = rst;

    table[0xF8] = ld_hl_sp_r8;
    table[0xF9] = ld_sp_hl;
    table[0xFA] = ld_a_a16;
    table[0xFB] = ei;
    table[0xFC] = illegal;
    table[0xFD] = illegal;
    table[0xFE] = alu_imm;
    table[0xFF] = rst;

    table
    }
}

// ============================================================
// Handlers with a tiny inner table
// ============================================================

fn ld_r_r<B: Bus>(cpu: &mut Cpu, bus: &mut B, op: u8) -> Result<()> {
    let value = read_r8(cpu, bus, op & 0x07)?;
    write_r8(cpu, bus, (op >> 3) & 0x07, value)
}

/// Indexed by bits 3-5 of the opcode.
const ALU_OPS: [fn(&mut Cpu, u8); 8] = [add_a, adc_a, sub_a, sbc_a, and_a, xor_a, or_a, cp_a];

fn alu_r<B: Bus>(cpu: &mut Cpu, bus: &mut B, op: u8) -> Result<()> {
    let value = read_r8(cpu, bus, op & 0x07)?;
    ALU_OPS[((op >> 3) & 0x07) as usize](cpu, value);
    Ok(())
}

fn alu_imm<B: Bus>(cpu: &mut Cpu, bus: &mut B, op: u8) -> Result<()> {
    let value = cpu.fetch(bus)?;
    ALU_OPS[((op >> 3) & 0x07) as usize](cpu, value);
    Ok(())
}

// ============================================================
// r8 helpers (B,C,D,E,H,L,(HL),A)
// ============================================================

#[inline(always)]
fn read_r8<B: Bus>(cpu: &Cpu, bus: &mut B, index: u8) -> Result<u8> {
    Ok(match index {
        0 => cpu.b(),
        1 => cpu.c(),
        2 => cpu.d(),
        3 => cpu.e(),
        4 => cpu.h(),
        5 => cpu.l(),
        6 => bus.read(cpu.hl)?,
        7 => cpu.a(),
        _ => unreachable!(),
    })
}

#[inline(always)]
fn write_r8<B: Bus>(cpu: &mut Cpu, bus: &mut B, index: u8, value: u8) -> Result<()> {
    match index {
        0 => cpu.bc.set_hi(value),
        1 => cpu.bc.set_lo(value),
        2 => cpu.de.set_hi(value),
        3 => cpu.de.set_lo(value),
        4 => cpu.hl.set_hi(value),
        5 => cpu.hl.set_lo(value),
        6 => bus.write(cpu.hl, value)?,
        7 => cpu.af.set_hi(value),
        _ => unreachable!(),
    }
    Ok(())
}

// ============================================================
// TODO: Spcial handlers
// ============================================================

fn illegal<B: Bus>(cpu: &mut Cpu, _: &mut B, op: u8) -> Result<()>
{
    println!("illegal opcode {op:#04X} at PC={:#06X}", cpu.pc.wrapping_sub(1));
    Ok(())
}

macro_rules! stubs {
    ($($name:ident),* $(,)?) => {
        $(
            fn $name<B: Bus>(cpu: &mut Cpu, _: &mut B, op: u8) -> Result<()> {
                println!(
                    "TODO: implement opcode {op:#04X} ({}) at PC={:#06X}",
                    stringify!($name),
                    cpu.pc.wrapping_sub(1)
                );
                Ok(())
            }
        )*
    };
}

// TODO: When an instructions of stubs is coded, you have to remove it from the stubs!
//
stubs!(
    halt, nop, stop,
    rlca, rrca, rla, rra,
    ld_rr_d16, ld_indirect_rr_a, ld_a_indirect_rr, ld_a16_sp,
    inc_rr, dec_rr, inc_r8, dec_r8, ld_r8_d8, add_hl_rr,
    jr, jr_cond, jp, jp_cond, jp_hl,
    call, call_cond, ret, ret_cond, reti, rst,
    ld_hl_inc_a, ld_a_hl_inc, ld_hl_dec_a, ld_a_hl_dec,
    daa, cpl, scf, ccf,
    pop_rr, push_rr,
    ldh_a8_a, ldh_a_a8, ld_c_a, ld_a_c, ld_a16_a, ld_a_a16,
    add_sp_r8, ld_hl_sp_r8, ld_sp_hl, di, ei,
);

fn prefix_cb<B: Bus>(cpu: &mut Cpu, bus: &mut B, _: u8) -> Result<()> {
    let op = cpu.fetch(bus)?;
    Opcodes::<B>::CB_TABLE[op as usize](cpu, bus, op)
}

fn do_rlc(cpu: &mut Cpu, v: u8) -> u8 {
    let r = v.rotate_left(1);
    cpu.set_flags(r == 0, false, false, v & 0x80 != 0);
    r
}
fn do_rrc(cpu: &mut Cpu, v: u8) -> u8 {
    let r = v.rotate_right(1);
    cpu.set_flags(r == 0, false, false, v & 0x01 != 0);
    r
}
fn do_rl(cpu: &mut Cpu, v: u8) -> u8 {
    let r = (v << 1) | cpu.cf() as u8;
    cpu.set_flags(r == 0, false, false, v & 0x80 != 0);
    r
}
fn do_rr(cpu: &mut Cpu, v: u8) -> u8 {
    let r = (v >> 1) | ((cpu.cf() as u8) << 7);
    cpu.set_flags(r == 0, false, false, v & 0x01 != 0);
    r
}
fn do_sla(cpu: &mut Cpu, v: u8) -> u8 {
    let r = v << 1;
    cpu.set_flags(r == 0, false, false, v & 0x80 != 0);
    r
}
fn do_sra(cpu: &mut Cpu, v: u8) -> u8 {
    let r = (v >> 1) | (v & 0x80); // bit 7 stays
    cpu.set_flags(r == 0, false, false, v & 0x01 != 0);
    r
}
fn do_swap(cpu: &mut Cpu, v: u8) -> u8 {
    let r = v.rotate_left(4);
    cpu.set_flags(r == 0, false, false, false);
    r
}
fn do_srl(cpu: &mut Cpu, v: u8) -> u8 {
    let r = v >> 1;
    cpu.set_flags(r == 0, false, false, v & 0x01 != 0);
    r
}

/// Generates a handler: read r8 (bits 0-2), apply the operation, write back.
macro_rules! cb_rot {
    ($($name:ident => $op:ident),* $(,)?) => {
        $(
            fn $name<B: Bus>(cpu: &mut Cpu, bus: &mut B, op: u8) -> Result<()> {
                let r = op & 0x07;
                let v = read_r8(cpu, bus, r)?;
                let res = $op(cpu, v);
                write_r8(cpu, bus, r, res)
            }
        )*
    };
}

//TODO: Same as stubs
cb_rot!(
    rlc => do_rlc, rrc => do_rrc, rl => do_rl, rr => do_rr,
    sla => do_sla, sra => do_sra, swap => do_swap, srl => do_srl,
);

fn bit_r<B: Bus>(cpu: &mut Cpu, bus: &mut B, op: u8) -> Result<()> {
    let v = read_r8(cpu, bus, op & 0x07)?;
    let bit = (op >> 3) & 0x07;

    let c = cpu.cf(); // carry untouched 
    cpu.set_flags(v & (1 << bit) == 0, false, true, c);
    Ok(()) // no write-back, so BIT (HL) costs one memory access less
}

fn res_r<B: Bus>(cpu: &mut Cpu, bus: &mut B, op: u8) -> Result<()> {
    let r = op & 0x07;
    let v = read_r8(cpu, bus, r)?;
    write_r8(cpu, bus, r, v & !(1 << ((op >> 3) & 0x07)))
}

fn set_r<B: Bus>(cpu: &mut Cpu, bus: &mut B, op: u8) -> Result<()> {
    let r = op & 0x07;
    let v = read_r8(cpu, bus, r)?;
    write_r8(cpu, bus, r, v | (1 << ((op >> 3) & 0x07)))
}

fn add_a(_: &mut Cpu, _: u8) {}
fn adc_a(_: &mut Cpu, _: u8) {}
fn sub_a(_: &mut Cpu, _: u8) {}
fn sbc_a(_: &mut Cpu, _: u8) {}
fn and_a(_: &mut Cpu, _: u8) {}
fn xor_a(_: &mut Cpu, _: u8) {}
fn or_a(_: &mut Cpu, _: u8) {}
fn cp_a(_: &mut Cpu, _: u8) {}
