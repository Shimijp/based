use crate::base::App;

mod base;

fn main() ->eframe::Result {
    let options =  eframe::NativeOptions::default();
    eframe::run_native(
        "Based",
        options,
        Box::new(|_cc| Ok(Box::new(App::default())))
    )


}
