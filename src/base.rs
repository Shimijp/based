use std::fmt;
use std::fmt::{format, Formatter};
use eframe::egui;

pub enum Base
{
    Hex,
    Dec,
    Bin,

}

#[derive(Debug)]
pub enum Value
{
    UByte(u8),
    UDouble(u16),
    UQuad(u32),
    UEightByte(u64),
    IByte(u8),
    IDouble(u16),
    IQuad(u32),
    IEightByte(u64)

}


pub struct Word
{


    src_base: Base,
    src_val: Value,
    res_base: Base,
    res_val: Value,


}

impl Word
{
    pub fn src_to_str(&self) ->String
    {
        match self.src_base {
            Base::Hex =>
                {
                    match self.src_val
                    {
                        Value::UByte(byte) | Value::IByte(byte) => format!("{:02x}", byte),
                        Value::UDouble(bytes) | Value::IDouble(bytes) => format!("{:04x}", bytes),
                        Value::UQuad(bytes) | Value::IQuad(bytes) => format!("{:08x}", bytes),
                        Value::UEightByte(bytes) | Value::IEightByte(bytes) => format!("{}")

                    };
                },
            Base::Dec => {},
            Base::Bin => {  },

        }
        "".to_string()
    }
}

impl Default for Word
{
    fn default() -> Self {
        Word{

            src_base: Base::Hex,
            src_val: Value::UByte(0),
            res_base: Base::Dec,
            res_val : Value::UByte(0),
        }
    }
}
pub struct App
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
    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame)
    {

        egui::CentralPanel::default().show(ui, |ui| {
            ui.heading("Based");
            ui.horizontal(|ui|
                {
                    ui.add(
                        egui::TextEdit::singleline(&mut self.word.src_val)
                            .hint_text("enter a value")
                    )
                });
        });
    }
}