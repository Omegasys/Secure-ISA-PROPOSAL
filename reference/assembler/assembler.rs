use super::encoder::*;
use super::parser::*;
use super::symbol_table::*;

pub fn assemble_line(line: &str, _symbols: &SymbolTable) -> Option<u32> {
    let tokens = tokenize(line);

    if tokens.is_empty() {
        return None;
    }

    match tokens[0].text.as_str() {
        "add" => {
            Some(encode_r(0x10, 1, 2, 3))
        }
        "sub" => {
            Some(encode_r(0x11, 1, 2, 3))
        }
        "halt" => {
            Some(encode_s(0xA0, 0, 0, 0))
        }
        _ => None,
    }
}
