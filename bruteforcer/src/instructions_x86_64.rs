// X86_64 words are LITTLE ENDIAN, meaning the least significant byte is first.

use std::fmt::Display;

use crate::encodings::SerializeAMD64MachineCode;

/// Enumeration of the available X86_64 Instructions that we
/// are utilizing for generating output programs. This is not
/// a complete list of all instructions by ANY means.
#[allow(unused)]
pub enum Instruction {
    MOV(Operand, Operand),
    MOVQ(Operand, Operand),
    MOVL(Operand, Operand),
    XOR(Operand, Operand),
    RET,
    VPERMPS(Operand, Operand, Operand),
    VPERMILPS(Operand, Operand, Operand),
    VPERMD(Operand, Operand, Operand),
    VPMASKMOVD(Operand, Operand, Operand),
    VMOVDQA(Operand, Operand),
    VZEROUPPER,
    RDTSC,
}

impl SerializeAMD64MachineCode for Instruction {
    fn write_amd64_bytes(&self, program: &mut Vec<u8>) {
        match &self {
            Instruction::MOV(_, _) => todo!(),
            Instruction::MOVQ(_, _) => todo!(),
            Instruction::MOVL(_, _) => todo!(),
            Instruction::RET => program.push(0xc3),
            Instruction::XOR(_, _) => todo!(),
            Instruction::VPERMPS(_, _, _) => todo!(),
            Instruction::VPERMD(_, _, _) => todo!(),
            Instruction::VPMASKMOVD(_, _, _) => todo!(),
            Instruction::VMOVDQA(_, _) => todo!(),
            Instruction::VPERMILPS(_, _, _) => todo!(),
            Instruction::VZEROUPPER => {
                program.push(0xc5);
                program.push(0xf8);
                program.push(0x77);
            }
            Instruction::RDTSC => {
                program.push(0x0f);
                program.push(0x31);
            }
        }
    }
}

impl Display for Instruction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self {
            Instruction::MOV(src, dst) => write!(f, "mov {} {}", src, dst),
            Instruction::MOVQ(src, dst) => write!(f, "movq {} {}", src, dst),
            Instruction::MOVL(src, dst) => write!(f, "movl {} {}", src, dst),
            Instruction::RET => write!(f, "ret"),
            Instruction::XOR(src, dst) => write!(f, "xor {} {}", src, dst),
            Instruction::VPERMPS(reg1, reg2, reg3) => {
                write!(f, "vpermps {} {} {}", reg1, reg2, reg3)
            }
            Instruction::VPERMD(reg1, reg2, reg3) => write!(f, "vpermd {} {} {}", reg1, reg2, reg3),
            Instruction::VPMASKMOVD(reg1, reg2, reg3) => {
                write!(f, "vpmaskmovd {} {} {}", reg1, reg2, reg3)
            }
            Instruction::VMOVDQA(src, dst) => write!(f, "vmovdqa {} {}", src, dst),
            Instruction::VPERMILPS(src, dst, mask) => {
                write!(f, "vpermilps {} {} {}", src, dst, mask)
            }
            Instruction::VZEROUPPER => write!(f, "vzeroupper"),
            Instruction::RDTSC => write!(f, "rdtsc"),
        }
    }
}

/// Operands for use within assembly. Please refer to this awesome
/// video for information about addressing modes from which this code
/// is derived: https://www.youtube.com/watch?v=lUbPUWtmVUU
#[derive(Debug)]
#[allow(unused)]
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

impl SerializeAMD64MachineCode for Operand {
    fn write_amd64_bytes(&self, _bytes: &mut Vec<u8>) {
        match &self {
            Operand::Immediate(_) => todo!(),
            Operand::Register(_) => todo!(),
            Operand::Memory(_) => todo!(),
            Operand::Index(_, _) => todo!(),
            Operand::ScaledIndex(_, _, _) => todo!(),
            Operand::ScaledDisplacedIndex(_, _, _) => todo!(),
        }
    }
}

#[derive(Debug)]
#[allow(unused)]
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

    XMM0,
    XMM1,
    XMM2,
    XMM3,
    XMM4,
    XMM5,
    XMM6,
    XMM7,
    XMM8,
    XMM9,
    XMM10,
    XMM11,
    XMM12,
    XMM13,
    XMM14,
    XMM15,

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
            Register::XMM0 => write!(f, "%xmm0"),
            Register::XMM1 => write!(f, "%xmm1"),
            Register::XMM2 => write!(f, "%xmm2"),
            Register::XMM3 => write!(f, "%xmm3"),
            Register::XMM4 => write!(f, "%xmm4"),
            Register::XMM5 => write!(f, "%xmm5"),
            Register::XMM6 => write!(f, "%xmm6"),
            Register::XMM7 => write!(f, "%xmm7"),
            Register::XMM8 => write!(f, "%xmm8"),
            Register::XMM9 => write!(f, "%xmm9"),
            Register::XMM10 => write!(f, "%xmm10"),
            Register::XMM11 => write!(f, "%xmm11"),
            Register::XMM12 => write!(f, "%xmm12"),
            Register::XMM13 => write!(f, "%xmm13"),
            Register::XMM14 => write!(f, "%xmm14"),
            Register::XMM15 => write!(f, "%xmm15"),
        }
    }
}

impl SerializeAMD64MachineCode for Register {
    fn write_amd64_bytes(&self, bytes: &mut Vec<u8>) {
        match &self {
            Register::RAX => bytes.push(0xc0),
            Register::EAX => todo!(),
            Register::RBX => todo!(),
            Register::EBX => todo!(),
            Register::RCX => todo!(),
            Register::ECX => todo!(),
            Register::RDX => todo!(),
            Register::EDX => todo!(),
            Register::RSP => bytes.push(0x24),
            Register::ESP => todo!(),
            Register::RDI => todo!(),
            Register::EDI => todo!(),
            Register::RSI => todo!(),
            Register::ESI => todo!(),
            Register::RBP => todo!(),
            Register::EBP => todo!(),
            Register::RIP => todo!(),
            Register::EIP => todo!(),
            Register::R8 => todo!(),
            Register::R9 => todo!(),
            Register::R10 => todo!(),
            Register::R11 => todo!(),
            Register::R12 => todo!(),
            Register::R13 => todo!(),
            Register::R14 => todo!(),
            Register::R15 => todo!(),
            Register::YMM0 => todo!(),
            Register::YMM1 => todo!(),
            Register::YMM2 => todo!(),
            Register::YMM3 => todo!(),
            Register::YMM4 => todo!(),
            Register::YMM5 => todo!(),
            Register::YMM6 => todo!(),
            Register::YMM7 => todo!(),
            Register::YMM8 => todo!(),
            Register::YMM9 => todo!(),
            Register::YMM10 => todo!(),
            Register::YMM11 => todo!(),
            Register::YMM12 => todo!(),
            Register::YMM13 => todo!(),
            Register::YMM14 => todo!(),
            Register::YMM15 => todo!(),
            Register::XMM0 => todo!(),
            Register::XMM1 => todo!(),
            Register::XMM2 => todo!(),
            Register::XMM3 => todo!(),
            Register::XMM4 => todo!(),
            Register::XMM5 => todo!(),
            Register::XMM6 => todo!(),
            Register::XMM7 => todo!(),
            Register::XMM8 => todo!(),
            Register::XMM9 => todo!(),
            Register::XMM10 => todo!(),
            Register::XMM11 => todo!(),
            Register::XMM12 => todo!(),
            Register::XMM13 => todo!(),
            Register::XMM14 => todo!(),
            Register::XMM15 => todo!(),
            Register::EFLAGS => todo!(),
        }
    }
}
