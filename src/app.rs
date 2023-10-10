use egui::{containers::*, widgets::*, *};

#[derive(Debug, PartialEq)]
enum Mode {
    Preset,
    Custom,
}

#[derive(Debug, PartialEq)]
enum Algorithm {
    AStar,
    Greedy,
    Dijkstra,
}

#[derive(PartialEq)]
pub struct App {
    mode: Mode,
    area: String,
    algo: Algorithm,
    paused: bool,
    speed: f32,
    code: String,
}

impl Default for App {
    fn default() -> Self {
        Self {
            mode: Mode::Preset,
            area: "Chicago".to_string(),
            algo: Algorithm::AStar,
            paused: false,
            speed: 1.0,
            code: "".to_string(),
        }
    }
}

impl eframe::App for App {
    fn update(&mut self, ctx: &Context, _frame: &mut eframe::Frame) {
        CentralPanel::default()
            .frame(Frame::dark_canvas(&ctx.style()))
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
            mode,
            area,
            algo,
            paused,
            speed,
            code,
        } = self;

        ui.horizontal(|ui| {
            ui.selectable_value(mode, Mode::Preset, "Preset");
            let clicked = ui.selectable_value(mode, Mode::Custom, "Custom").clicked();

            if clicked {
                *code = format!(
r#"[out:json];
area[name="{}"]->.searchArea;
way(area.searchArea)["highway"];
out body;
>;
out skel qt;"#,
                    area
                )
            }

            ui.label("Selection location");
        });

        if self.mode == Mode::Preset {
            ui.add(TextEdit::singleline(area).hint_text("Enter a city..."));
        } else {
            let theme = egui_extras::syntax_highlighting::CodeTheme::from_memory(ui.ctx());
            let mut layouter = |ui: &Ui, string: &str, wrap_width: f32| {
                let mut layout_job =
                    egui_extras::syntax_highlighting::highlight(ui.ctx(), &theme, string, "c");
                layout_job.wrap.max_width = wrap_width;
                ui.fonts(|f| f.layout_job(layout_job))
            };

            ScrollArea::vertical().show(ui, |ui| {
                ui.add(
                    TextEdit::multiline(code)
                        .font(egui::TextStyle::Monospace) // for cursor height
                        .code_editor()
                        .desired_rows(1)
                        .lock_focus(true)
                        .desired_width(f32::INFINITY)
                        .layouter(&mut layouter),
                );
            });
        }

        ui.separator();

        ui.heading("Visualizer");

        ComboBox::from_label("algorithm")
            .selected_text(format!("{algo:?}"))
            .show_ui(ui, |ui| {
                ui.style_mut().wrap = Some(false);
                ui.set_min_width(60.0);
                ui.selectable_value(algo, Algorithm::AStar, "A* Search");
                ui.selectable_value(algo, Algorithm::Greedy, "Greedy Best-first Search");
                ui.selectable_value(algo, Algorithm::Dijkstra, "Dijkstra's Algorithm");
            });

        ui.add(Slider::new(speed, 0.1..=10.0).text("speed"));

        reset_button(ui, self);
    }
}
