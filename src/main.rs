use eframe::egui;

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
    dark_mode: bool
}

impl MyEguiApp {
    fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        Self {
            counter: 0,
            name: String::new(),
            dark_mode: false,
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

            ui.separator();

            ui.label("Your name:");
            ui.text_edit_singleline(&mut self.name);
        });
    }

    fn main_panel(&mut self, ctx: &egui::Context) {
        egui::CentralPanel::default().show(ctx, |ui| {
            self.counter_ui(ui);
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
}

impl eframe::App for MyEguiApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
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
