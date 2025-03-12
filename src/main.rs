use egui::FontData;
use egui_file_dialog::FileDialog;
use std::path::PathBuf;
use track2line_lib as t2l;

struct T2lFileDialog {
    file_dialog: FileDialog,
    selected_path: Option<PathBuf>,
    show_dialog: bool,
    pathsets: Option<t2l::PathSets>,
    is_continue: bool,
    loaded: bool,
}

impl T2lFileDialog {
    fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        Self {
            file_dialog: FileDialog::new(),
            selected_path: None,
            show_dialog: false,
            pathsets: None,
            is_continue: false,
            loaded: false,
        }
    }
}

impl eframe::App for T2lFileDialog {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        let mut fonts = egui::FontDefinitions::default();
        fonts.font_data.insert(
            "japanese_font".to_owned(),
            FontData::from_static(include_bytes!("../fonts/ipaexg00401/ipaexg.ttf")).into(),
        );
        fonts
            .families
            .get_mut(&egui::FontFamily::Proportional)
            .unwrap()
            .insert(0, "japanese_font".to_owned());

        ctx.set_fonts(fonts);
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("track2line");

            if ui.button("chose folder").clicked() {
                self.file_dialog.pick_directory();
            }

            if self.loaded && ui.button("rename").clicked() {
                if self.selected_path.is_none() {
                    ui.label("select folder");
                    return;
                }
                let a = t2l::PathSets::new(
                    self.selected_path.clone().unwrap_or(PathBuf::from("foo")),
                    "wav",
                    "txt",
                )
                .unwrap();
                self.pathsets = Some(a);
                self.show_dialog = true;
            }

            if self.show_dialog {
                egui::Window::new("continue?")
                    .collapsible(false)
                    .resizable(false)
                    .show(ctx, |ui| {
                        let a = self.pathsets.as_mut().unwrap().check().unwrap();
                        ui.label(format!("{}", a));
                        ui.label("continue?");
                        ui.horizontal(|ui| {
                            if ui.button("yes").clicked() {
                                self.show_dialog = false;
                                self.is_continue = true;
                            }
                            if ui.button("no").clicked() {
                                self.show_dialog = false;
                            }
                        });
                    });
            }

            if self.is_continue {
                self.pathsets.as_mut().unwrap().rename().unwrap();
            }

            if let Some(path) = self.file_dialog.update(ctx).picked() {
                self.selected_path = Some(path.to_path_buf().clone());
                self.loaded = true;
                println!("chose folder: {:?}", path);
            }

            if let Some(path) = &self.selected_path {
                ui.label(format!("{:?}", path));
            }
        });
    }
}

fn main() {
    eframe::run_native(
        "track2line",
        eframe::NativeOptions::default(),
        Box::new(|cc| Ok(Box::new(T2lFileDialog::new(cc)))),
    )
    .unwrap();
}
