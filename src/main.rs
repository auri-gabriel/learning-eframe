use eframe::egui;

#[derive(Default)]
struct PersonRow {
    name: String,
    age: i32,
    active: bool,
}

fn main() -> eframe::Result<()> {
    let options = eframe::NativeOptions::default();

    eframe::run_native(
        "Structured egui app",
        options,
        Box::new(|cc| Ok(Box::new(MyEguiApp::new(cc)))),
    )
}

#[derive(Default)]
struct MyEguiApp {
    counter: i32,
    name: String,
    dark_mode: bool,
    ui_scale: f32,
    people: Vec<PersonRow>,
}

impl MyEguiApp {
    fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        Self {
            counter: 0,
            name: String::new(),
            dark_mode: false,
            ui_scale: 1.5,
            people: vec![
                PersonRow {
                    name: "Alice".to_string(),
                    age: 28,
                    active: true,
                },
                PersonRow {
                    name: "Bruno".to_string(),
                    age: 34,
                    active: false,
                },
            ],
        }
    }

    // -------- PANELS --------

    fn top_bar(&mut self, ctx: &egui::Context) {
        egui::TopBottomPanel::top("top_bar").show(ctx, |ui| {
            ui.horizontal(|ui| {
                ui.heading("My egui app");

                if ui.button("Reset").clicked() {
                    self.counter = 0;
                }
            });
        });
    }

    fn side_panel(&mut self, ctx: &egui::Context) {
        egui::SidePanel::left("side_panel").show(ctx, |ui| {
            ui.heading("Settings");

            ui.checkbox(&mut self.dark_mode, "Dark mode");

            ui.add(egui::Slider::new(&mut self.ui_scale, 0.5..=3.0).text("UI Scale"));

            ui.separator();

            ui.label("Your name:");
            ui.text_edit_singleline(&mut self.name);
        });
    }

    fn main_panel(&mut self, ctx: &egui::Context) {
        egui::CentralPanel::default().show(ctx, |ui| {
            self.counter_ui(ui);
            ui.separator();
            self.people_table_ui(ui);
            ui.separator();
            let (response, painter) = ui.allocate_painter(ui.available_size(), egui::Sense::hover());

            let rect = response.rect;

            let center = rect.center();

            painter.circle_filled(center, 50.0, egui::Color32::LIGHT_BLUE);
        });
    }

    // -------- COMPONENTS --------

    fn counter_ui(&mut self, ui: &mut egui::Ui) {
        ui.heading("Counter");

        ui.horizontal(|ui| {
            if ui.button("-").clicked() {
                self.counter -= 1;
            }

            ui.label(self.counter.to_string());

            if ui.button("+").clicked() {
                self.counter += 1;
            }
        });

        ui.separator();

        if !self.name.is_empty() {
            ui.label(format!("Hello, {}", self.name));
        }
    }

    fn people_table_ui(&mut self, ui: &mut egui::Ui) {
        ui.heading("People Table");

        let mut row_to_remove = None;

        egui::Grid::new("people_table")
            .num_columns(5)
            .striped(true)
            .show(ui, |ui| {
                ui.strong("#");
                ui.strong("Name");
                ui.strong("Age");
                ui.strong("Active");
                ui.strong("Actions");
                ui.end_row();

                for (idx, person) in self.people.iter_mut().enumerate() {
                    ui.label((idx + 1).to_string());
                    ui.text_edit_singleline(&mut person.name);
                    ui.add(egui::DragValue::new(&mut person.age).range(0..=120));
                    ui.checkbox(&mut person.active, "");

                    if ui.button("Delete").clicked() {
                        row_to_remove = Some(idx);
                    }

                    ui.end_row();
                }
            });

        if let Some(idx) = row_to_remove {
            self.people.remove(idx);
        }

        if ui.button("Add row").clicked() {
            self.people.push(PersonRow {
                name: "New person".to_string(),
                age: 18,
                active: false,
            });
        }
    }
}

impl eframe::App for MyEguiApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        ctx.set_pixels_per_point(self.ui_scale);

        // layout structure
        self.top_bar(ctx);
        self.side_panel(ctx);
        self.main_panel(ctx);

        // theme switching
        if self.dark_mode {
            ctx.set_visuals(egui::Visuals::dark());
        } else {
            ctx.set_visuals(egui::Visuals::light());
        }
    }
}
