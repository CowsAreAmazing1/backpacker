use backpacker::GameEgui;

fn main() -> eframe::Result<()> {
    eframe::run_native(
        "grahpher",
        eframe::NativeOptions::default(),
        Box::new(|cc| Ok(Box::new(GameEgui::new(cc)))),
    )
}
