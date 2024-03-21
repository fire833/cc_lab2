// X86_64 words are LITTLE ENDIAN, meaning the least significant byte is first.

use std::fmt::Display;

/// Enumeration of the available X86_64 Instructions that we
/// are utilizing for generating output programs. This is not
/// a complete list of all instructions by ANY means.
pub enum Instruction {
    /// Move Quadword
    MoveQ(Operand, Operand),
    MoveL(Operand, Operand),
    Return,
}

impl Instruction {
    fn write_bytes(&self, program: &mut Vec<u8>) {
        match &self {
            Instruction::MoveQ(src, dst) => todo!(),
            Instruction::MoveL(src, dst) => todo!(),
            Instruction::Return => todo!(),
        }
    }
}

/// Operands for use within assembly. Please refer to this awesome
/// video for information about addressing modes from which this code
/// is derived: https://www.youtube.com/watch?v=lUbPUWtmVUU
#[derive(Debug)]
pub enum Operand {
    /// Provide integer data as an operand.
    Immediate(i32),
    /// Directly use a register as an operand.
    Register(Register),
    /// Reference the value within a register as an operand.
    Memory(Register),
    /// Get the address of an operand by taking the first register,
    /// and summing with the value of the second as the indexing
    /// register.
    Index(Register, Register),
    /// Get the address of an operand by adding the first register value
    /// with a scaled value from the second register.
    ScaledIndex(Register, Register, i32),
    /// Statically define a displacement from the value summed with the
    /// second register value scaled by the second integer value to use
    /// as the effective address.
    ScaledDisplacedIndex(i32, Register, i32),
}

impl Display for Operand {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self {
            Operand::Immediate(val) => write!(f, "${}", val),
            Operand::Register(register) => write!(f, "{}", register),
            Operand::Memory(register) => write!(f, "({})", register),
            Operand::Index(reg1, reg2) => write!(f, "({}, {})", reg1, reg2),
            Operand::ScaledIndex(reg1, reg2, val) => write!(f, "({}, {}, {})", reg1, reg2, val),
            Operand::ScaledDisplacedIndex(displ, reg, scalar) => {
                write!(f, "{}(, {}, {})", displ, reg, scalar)
            }
        }
    }
}

#[derive(Debug)]
pub enum Register {
    /// 64 Bit Accumulator Register
    RAX,
    /// 32 Bit Accumulator Register
    EAX,
    RBX,
    EBX,
    RCX,
    ECX,
    /// 64 Bit Data Register
    RDX,
    /// 32 Bit Data Register
    EDX,
    /// Stack pointer Register (Sometimes called SP)
    RSP,
    /// 32 bit stack pointer register
    ESP,
    /// 64 bit Destination register
    RDI,
    /// 32 bit Destination register
    EDI,
    /// 64 bit Source register
    RSI,
    /// 32 bit Source register
    ESI,
    /// 64 bit Stack base pointer
    RBP,
    /// 32 bit Stack base pointer
    EBP,

    /// 64 bit Instruction Pointer
    RIP,
    /// 32 bit Instruction Pointer
    EIP,

    /// General purpose registers
    R8,
    R9,
    R10,
    R11,
    R12,
    R13,
    R14,
    R15,

    /// AVX Register 0
    YMM0,
    /// AVX Register 1
    YMM1,
    /// AVX Register 2
    YMM2,
    /// AVX Register 3
    YMM3,
    /// AVX Register 4
    YMM4,
    /// AVX Register 5
    YMM5,
    /// AVX Register 6
    YMM6,
    /// AVX Register 7
    YMM7,
    /// AVX Register 8
    YMM8,
    /// AVX Register 9
    YMM9,
    /// AVX Register 10
    YMM10,
    /// AVX Register 11
    YMM11,
    /// AVX Register 12
    YMM12,
    /// AVX Register 13
    YMM13,
    /// AVX Register 14
    YMM14,
    /// AVX Register 15
    YMM15,

    /// The EFLAGS register
    EFLAGS,
}

impl Display for Register {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self {
            Register::RAX => write!(f, "%rax"),
            Register::EAX => write!(f, "%eax"),
            Register::RBX => write!(f, "%rbx"),
            Register::EBX => write!(f, "%ebx"),
            Register::RCX => write!(f, "%rcx"),
            Register::ECX => write!(f, "%ecx"),
            Register::RDX => write!(f, "%rdx"),
            Register::EDX => write!(f, "%edx"),
            Register::RSP => write!(f, "%rsp"),
            Register::ESP => write!(f, "%esp"),
            Register::RDI => write!(f, "%rdi"),
            Register::EDI => write!(f, "%edi"),
            Register::RSI => write!(f, "%rsi"),
            Register::ESI => write!(f, "%esi"),
            Register::RBP => write!(f, "%rbp"),
            Register::EBP => write!(f, "%ebp"),
            Register::RIP => write!(f, "%rip"),
            Register::EIP => write!(f, "%eip"),
            Register::R8 => write!(f, "%r8"),
            Register::R9 => write!(f, "%r9"),
            Register::R10 => write!(f, "%r10"),
            Register::R11 => write!(f, "%r11"),
            Register::R12 => write!(f, "%r12"),
            Register::R13 => write!(f, "%r13"),
            Register::R14 => write!(f, "%r14"),
            Register::R15 => write!(f, "%r15"),
            Register::YMM0 => write!(f, "%ymm0"),
            Register::YMM1 => write!(f, "%ymm1"),
            Register::YMM2 => write!(f, "%ymm2"),
            Register::YMM3 => write!(f, "%ymm3"),
            Register::YMM4 => write!(f, "%ymm4"),
            Register::YMM5 => write!(f, "%ymm5"),
            Register::YMM6 => write!(f, "%ymm6"),
            Register::YMM7 => write!(f, "%ymm7"),
            Register::YMM8 => write!(f, "%ymm8"),
            Register::YMM9 => write!(f, "%ymm9"),
            Register::YMM10 => write!(f, "%ymm10"),
            Register::YMM11 => write!(f, "%ymm11"),
            Register::YMM12 => write!(f, "%ymm12"),
            Register::YMM13 => write!(f, "%ymm13"),
            Register::YMM14 => write!(f, "%ymm14"),
            Register::YMM15 => write!(f, "%ymm15"),
            Register::EFLAGS => write!(f, "%eflags"),
        }
    }
}
