/// follows docs [here](https://gbdev.io/pandocs/CPU_Registers_and_Flags.html)
///
/// The flags register (f \in af) can contain:
/// 7/z => Zero flag
/// 6/n => subtraction flag (BCD)
/// 5/h => half carry flag (BCD)
/// 4/c => carry flag
///
/// The Zero Flag (Z)
/// This bit is set if and only if the result of an operation is zero. Used by conditional jumps.
///
/// The Carry Flag (C, or Cy)
/// Is set in these cases:
/// When the result of an 8-bit addition is higher than $FF.
/// When the result of a 16-bit addition is higher than $FFFF.
/// When the result of a subtraction or comparison is lower than zero (like in Z80
///   and x86 CPUs, but unlike in 65XX and ARM CPUs).
/// When a rotate/shift operation shifts out a “1” bit.
/// Used by conditional jumps and instructions such as ADC, SBC, RL, RLA, etc.
///
/// The BCD Flags (N, H)
/// These flags are used by the DAA instruction only. N indicates whether the previous
/// instruction has been a subtraction, and H indicates carry for the lower 4 bits of
/// the result. DAA also uses the C flag, which must indicate carry for the upper 4 bits.
/// After adding/subtracting two BCD numbers, DAA is used to convert the result to BCD
/// format. BCD numbers range from $00 to $99 rather than $00 to $FF. Because only two
/// flags (C and H) exist to indicate carry-outs of BCD digits, DAA is ineffective for
/// 16-bit operations (which have 4 digits), and use for INC/DEC operations (which do
/// not affect C-flag) has limits.
///
/// The memory map can be found here: https://gbdev.io/pandocs/Memory_Map.html#memory-map
#[derive(Debug)]
#[allow(dead_code)]
pub struct Cpu {
    af: [u8; 2],
    bc: [u8; 2],
    de: [u8; 2],
    hl: [u8; 2],
    sp: u16,
    pc: u16,

    /// memory of size 2^16, 64KB
    /// 
    /// not accurate to hardware but this keeps everything simple
    /// for the sake of addressing.
    /// 
    /// - `0x0000-0x3FFF` 16KiB ROM bank 00
    /// - `0x4000-0x7FFF` 16KiB ROM Bank-01NN 
    /// - `0x8000-0x9FFF` 8KiB VRAM 
    /// - `0xA000-0xBFFF` 8KiB external RAM 
    /// - `0xC000-0xCFFF` 4KiB Work RAM 
    /// - `0xD000-0xDFFF` 4KiB Work RAM 
    /// - `0xE000-0xFDFF` Echo RAM (mirror 0xC000-0xDDFF) 
    /// - `0xFE00-0xFE9F` Object Attribute Memory 
    /// - `0xFEA0-0xFEFF` Not usable 
    /// - `0xFF00-0xFF7F` IO registers 
    /// - `0xFF80-0xFFFE` High RAM 
    /// -` 0xFFFF-0xFFFF` Interrupt Enable register (IE)
    memory: [u8; 0x10000],
}
