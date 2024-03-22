/// Trait to encode objects to machine code.
pub trait SerializeMachineCode {
    fn write_bytes(&self, bytes: &mut Vec<u8>);
}

/// Trait to encode instruction blocks to C code strings.
pub trait CEncoder {
    fn encode_to_c(&self) -> String;
}
