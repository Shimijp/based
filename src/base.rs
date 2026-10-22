use eframe::egui;
use std::fmt;
use std::fmt::Formatter;
use strum_macros::Display;

#[derive(PartialEq, Debug, Display)]
pub enum Theme {
    Light,
    Dark,
    Default,
}
#[derive(Debug, PartialEq)]
pub enum Base {
    Hex,
    Dec,
    Bin,
}
impl fmt::Display for Base {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Base::Hex => write!(f, "Hexadecimal"),
            Base::Dec => write!(f, "Decimal"),
            Base::Bin => write!(f, "Binary"),
        }
    }
}
#[derive(PartialEq)]
pub enum Size {
    Byte,
    TwoByte,
    FourByte,
    EightByte,
}
impl fmt::Display for Size {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Size::Byte => write!(f, "Byte"),
            Size::TwoByte => write!(f, "Two Byte"),
            Size::FourByte => write!(f, "Four Byte"),
            Size::EightByte => write!(f, "Eight Byte"),
        }
    }
}

pub struct App {
    src_base: Base,
    size: Size,
    res_base: Base,
    src_str: String,
    res_str: String,
    is_signed: bool,
    theme: Theme
}
impl Default for App {
    fn default() -> Self {
        App {
            is_signed: false,
            src_base: Base::Hex,
            size: Size::Byte,
            res_base: Base::Dec,
            src_str: "0000".to_string(),
            res_str: "0000".to_string(),
            theme: Theme::Default,
        }
    }
}
impl App {
    fn convert_src_res(&mut self) {
        let bits = match self.size {
            Size::Byte => 8,
            Size::TwoByte => 16,
            Size::FourByte => 32,
            Size::EightByte => 64,
        };
        // i128 holds every signed and unsigned 64-bit input without overflow.
        let modulus = 1_i128 << bits;
        let radix = match self.src_base {
            Base::Hex => 16,
            Base::Dec => 10,
            Base::Bin => 2,
        };
        let input = self.src_str.trim();
        let input = match self.src_base {
            Base::Hex => input
                .strip_prefix("0x")
                .or_else(|| input.strip_prefix("0X"))
                .unwrap_or(input),
            Base::Bin => input
                .strip_prefix("0b")
                .or_else(|| input.strip_prefix("0B"))
                .unwrap_or(input),
            Base::Dec => input,
        };
        let value = match i128::from_str_radix(input, radix) {
            Ok(value) => value,
            Err(_) => {
                self.res_str = "Error: invalid number or value too large".to_string();
                return;
            }
        };
        let (min, max) = if self.is_signed && self.src_base == Base::Dec {
            (-modulus / 2, modulus / 2 - 1)
        } else {
            (0, modulus - 1)
        };
        if value < min || value > max {
            self.res_str = format!("Error: value out of range ({min} to {max})");
            return;
        }

        let raw = if value < 0 { value + modulus } else { value };
        self.res_str = match self.res_base {
            Base::Hex => format!("0x{:0width$X}", raw, width = bits / 4),
            Base::Bin => format!("{:0width$b}", raw, width = bits),
            Base::Dec => {
                let signed = if self.is_signed && raw >= modulus / 2 {
                    raw - modulus
                } else {
                    raw
                };
                signed.to_string()
            }
        };
    }
}

impl eframe::App for App {
    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {

        egui::CentralPanel::default().show(ui, |ui| {
            ui.heading("Based");
            egui::ComboBox::from_label("Select mode")
                .selected_text(self.theme.to_string())
                .show_ui(ui, |ui| {
                    ui.selectable_value(&mut self.theme, Theme::Light, "Light");
                    ui.selectable_value(&mut self.theme, Theme::Dark, "Dark");
                });
            ui.ctx().set_theme(
                match self. theme{
                    Theme::Light => egui::Theme::Light,
                    Theme::Dark => egui::Theme::Dark,
                    _ => ui.theme(),
                }
            );

            ui.horizontal(|ui| {
                ui.group(|ui| {
                    ui.vertical(|ui| {
                        ui.add(
                            egui::TextEdit::singleline(&mut self.src_str).hint_text("enter a value"),
                        );
                        ui.add_space(4.0);
                        ui.code(egui::RichText::new(&self.res_str));

                    });
                });

                ui.add_space(10.0);
                ui.group(|ui| {
                    ui.vertical(|ui| {
                        ui.add_space(4.0);

                        egui::ComboBox::from_label("Select base")
                            .selected_text(self.src_base.to_string())
                            .show_ui(ui, |ui| {
                                ui.selectable_value(&mut self.src_base, Base::Hex, "Hexadecimal");
                                ui.selectable_value(&mut self.src_base, Base::Dec, "Decimal");
                                ui.selectable_value(&mut self.src_base, Base::Bin, "Binary");
                            });
                        ui.add_space(4.0);
                        ui.horizontal(|ui| {
                            ui.checkbox(&mut self.is_signed, "Signed");
                            egui::ComboBox::from_label("Size")
                                .selected_text(self.size.to_string())
                                .show_ui(ui, |ui| {
                                    ui.selectable_value(&mut self.size, Size::Byte, "Byte");
                                    ui.selectable_value(&mut self.size, Size::TwoByte, "Two Byte");
                                    ui.selectable_value(&mut self.size, Size::FourByte, "Four Byte");
                                    ui.selectable_value(&mut self.size, Size::EightByte, "Eight Byte");
                                });
                        });

                        ui.add_space(4.0);
                        egui::ComboBox::from_label("Result base")
                            .selected_text(self.res_base.to_string())
                            .show_ui(ui, |ui| {
                                ui.selectable_value(&mut self.res_base, Base::Hex, "Hexadecimal");
                                ui.selectable_value(&mut self.res_base, Base::Dec, "Decimal");
                                ui.selectable_value(&mut self.res_base, Base::Bin, "Binary");
                            });
                    });
                });

            });
            ui.add_space(10.0);
            ui.vertical_centered(|ui| {
                if ui.button("Convert").clicked() {
                    self.convert_src_res();
                    ui.ctx().request_repaint();
                }
            })
        });
    }
}
