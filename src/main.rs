#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

fn main() -> Result<(), eframe::Error> {
    env_logger::init();

    let options = eframe::NativeOptions {
        drag_and_drop_support: true,

        initial_window_size: Some([1280.0, 1024.0].into()),

        #[cfg(feature = "wgpu")]
        renderer: eframe::Renderer::Wgpu,

        ..Default::default()
    };
    eframe::run_native(
        "Overpath",
        options,
        Box::new(|cc| {
            let style = egui::Style {
                visuals: egui::Visuals::dark(),
                ..egui::Style::default()
            };
            cc.egui_ctx.set_style(style);
            Box::new(overpath::App::default())
        }),
    )
}
