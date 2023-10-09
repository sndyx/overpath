use egui::{containers::*, widgets::*, *};

#[derive(Debug, PartialEq)]
enum Algorithm {
    AStar,
    Greedy,
    Dijkstra,
}

#[derive(PartialEq)]
pub struct App {
    area: String,
    algo: Algorithm,
    paused: bool,
    speed: f32,
    code: String,
}

impl Default for App {
    fn default() -> Self {
        Self {
            area: "Chicago".to_string(),
            algo: Algorithm::AStar,
            paused: false,
            speed: 1.0,
            code: "".to_string(),
        }
    }
}

impl eframe::App for App {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default()
            .frame(egui::Frame::dark_canvas(&ctx.style()))
            .show(ctx, |ui| {
                self.ui(ui);
            });
    }
}

impl App {
    fn ui(&mut self, ui: &mut Ui) {
        Frame::popup(ui.style())
            .stroke(Stroke::NONE)
            .show(ui, |ui| {
                ui.set_max_width(270.0);
                CollapsingHeader::new("Settings").show(ui, |ui| self.options_ui(ui));
            });
    }

    fn options_ui(&mut self, ui: &mut Ui) {
        let Self {
            area,
            algo,
            paused,
            speed,
            code,
        } = self;

        ui.add(egui::TextEdit::singleline(area).hint_text("Road selection area"));

        let theme = egui_extras::syntax_highlighting::CodeTheme::from_memory(ui.ctx());
        let mut layouter = |ui: &egui::Ui, string: &str, wrap_width: f32| {
            let mut layout_job =
                egui_extras::syntax_highlighting::highlight(ui.ctx(), &theme, string, "c");
            layout_job.wrap.max_width = wrap_width;
            ui.fonts(|f| f.layout_job(layout_job))
        };

        ui.collapsing("Advanced", |ui| {
            ui.group(|ui| {
                egui::ScrollArea::vertical().show(ui, |ui| {
                    ui.add(
                        egui::TextEdit::multiline(&mut self.code)
                            .font(egui::TextStyle::Monospace) // for cursor height
                            .code_editor()
                            .desired_rows(3)
                            .lock_focus(true)
                            .desired_width(f32::INFINITY)
                            .layouter(&mut layouter),
                    );
                });
            });
        });

        ui.separator();

        egui::ComboBox::from_label("algorithm")
            .selected_text(format!("{algo:?}"))
            .show_ui(ui, |ui| {
                ui.style_mut().wrap = Some(false);
                ui.set_min_width(60.0);
                ui.selectable_value(algo, Algorithm::AStar, "A* Search");
                ui.selectable_value(algo, Algorithm::Greedy, "Greedy Best-first Search");
                ui.selectable_value(algo, Algorithm::Dijkstra, "Dijkstra's Algorithm");
            });

        ui.add(Slider::new(speed, 0.1..=10.0).text("speed"));

        egui::reset_button(ui, self);
    }
}
