
use eframe::egui;

pub enum Base
{
    Hex,
    Dec,
    Bin,

}
#[repr(C)]
union Value
{
    u_byte: u8,
    u_double: u16,
    u_triple: u32,
    u_quadruple: u64,
    i_byte: i8,
    i_double: i16,
    i_triple: i32,
    i_quadruple: i32,
}
pub struct Word
{

    source: Value,
    res: Value,
    source_base: Base,
    res_base: Base,


}
impl Default for Word
{
    fn default() -> Self {
        Word{
            source: Value{u_byte : 0},
            res :  Value{u_byte : 0},
            source_base: Base::Hex,
            res_base: Base::Dec
        }
    }
}
struct App
{
    word: Word
}
impl Default for App
{
    fn default() -> Self {
        App{
            word: Word::default()
        }
    }
}

impl eframe::App for App{

}