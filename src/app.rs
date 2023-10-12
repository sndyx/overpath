use std::fs;
use egui::{containers::*, widgets::*, *};
use crate::node::Map;

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
    map: Map,
}

impl Default for App {
    fn default() -> Self {
        let contents = fs::read_to_string("res/export.json")
            .expect("Couldn't find export.json!");
        Self {
            mode: Mode::Preset,
            area: "Chicago".to_string(),
            algo: Algorithm::AStar,
            paused: false,
            speed: 1.0,
            code: "".to_string(),
            map: Map::from_json(contents.as_str())
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
        if !self.paused {
            ui.ctx().request_repaint();
        }

        let painter = Painter::new(
            ui.ctx().clone(),
            ui.layer_id(),
            ui.available_rect_before_wrap(),
        );
        self.paint(&painter);

        ui.expand_to_include_rect(painter.clip_rect());

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
            paused: _paused,
            speed,
            code,
            map: _map,
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

    fn paint(&mut self, painter: &Painter) {
        let mut shapes: Vec<Shape> = Vec::new();

        let rect = painter.clip_rect();
        let to_screen = emath::RectTransform::from_to(
            Rect::from_center_size(Pos2::ZERO, rect.square_proportions()),
            rect,
        );

        let mut paint_line = |points: [Pos2; 2], color: Color32, width: f32| {
            let line = [to_screen * points[0], to_screen * points[1]];

            if rect.intersects(Rect::from_two_pos(line[0], line[1])) {
                shapes.push(Shape::line_segment(line, (width, color)));
            }
        };

        let point = |lat: f32, lon: f32, map: &Map| {
            let x_scale = map.x1 / map.x2;
            let y_scale = map.y1 / map.y2;
            let max = if x_scale > y_scale { x_scale } else { y_scale };
            return pos2(lat / max, lon / max);
        };

        for node in self.map.nodes.iter() {
            for connection in node.1.connections.iter() {
                let c_node = self.map.nodes.get(connection).expect("Unknown node.");
                let p1 = point(node.1.lat, node.1.lon, &self.map);
                let p2 = point(c_node.lat, c_node.lon, &self.map);
                paint_line([p1, p2], Color32::RED, 2.0);
            }
        }

        painter.extend(shapes);
    }
}