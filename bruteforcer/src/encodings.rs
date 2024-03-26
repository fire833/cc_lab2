/// Trait to encode objects to machine code.
pub trait SerializeAMD64MachineCode {
    fn write_amd64_bytes(&self, bytes: &mut Vec<u8>);
}

/// Trait to encode instruction blocks to C code strings.
pub trait CEncoder {
    fn encode_to_c(&self, index: u32) -> String;
}
