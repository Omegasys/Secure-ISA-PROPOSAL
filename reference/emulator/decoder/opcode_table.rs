pub fn decode_opcode(opcode: u32) -> &'static str {
    match opcode {
        0x00..=0x0F => "DATA_TRANSFER",
        0x10..=0x1F => "ARITHMETIC",
        0x20..=0x2F => "LOGICAL",
        0x30..=0x3F => "BIT_MANIPULATION",
        0x40..=0x4F => "CONTROL_TRANSFER",
        0x60..=0x6F => "FLOATING_POINT",
        0x70..=0x7F => "STRING",
        0x80..=0x8F => "IO",
        0x90..=0x9F => "FLAGS",
        0xA0..=0xAF => "SYSTEM",
        0xB0..=0xBF => "AUDIT",
        0xC0..=0xCF => "SANDBOX",
        0xD0..=0xDF => "REMOTE",
        0xE0..=0xEF => "VIRTUALIZATION",
        0xF0..=0xF5 => "SECURE_BOOT",
        0x100..=0x10F => "CRYPTO",
        0x110..=0x11F => "USER",
        _ => "INVALID",
    }
}
