use crate::backend::{
    build_livestream, bulletin_categorizer, get_config, get_help, save_obs_file, Config,
};
use eframe::egui::{self, Color32, RichText, Vec2};
use std::{
    fs::File,
    io::{BufRead, BufReader},
};

const NUMBER_MEANING: [&str; 10] = [
    "0 - Ignore Text",
    "1 - Credits",
    "2 - Readings",
    "3 - Hymns",
    "4 - P: C:",
    "5 - Empty Scene",
    "6 - Service Name",
    "7 - Ignore Text",
    "8 - Special Music",
    "9 - With Previous",
];

struct Ui {
    config: Config,
    data: Option<Vec<(u8, String, bool)>>,
    in_preview_mode: bool,
    paragraph_value: u8,
}

impl Ui {
    fn new(cc: &eframe::CreationContext<'_>) -> Self {
        let mut fonts = egui::FontDefinitions::default();
        fonts.font_data.insert(
            "jbmono".to_owned(),
            std::sync::Arc::new(egui::FontData::from_static(include_bytes!(
                "../assets/jbmono.ttf"
            ))),
        );
        fonts
            .families
            .entry(egui::FontFamily::Proportional)
            .or_default()
            .insert(0, "jbmono".to_owned());
        cc.egui_ctx.set_fonts(fonts);

        Ui {
            config: get_config(),
            data: None,
            in_preview_mode: false,
            paragraph_value: 0,
        }
    }
}

impl eframe::App for Ui {
    fn ui(&mut self, ui: &mut egui::Ui, frame: &mut eframe::Frame) {
        ui.add_space(16.0);
        ui.horizontal(|ui| {
            ui.add_space(16.0);
            ui.heading("OBS Church Automator");
            ui.checkbox(&mut self.in_preview_mode, "Preview Mode");
            if ui.button("Help").clicked() {
                get_help();
            }
        });
        match self.in_preview_mode {
            false => {
                ui.horizontal(|ui| {
                    ui.add_space(16.0);
                    if ui.button("Load File").clicked() {
                        if let Some(file) = load_file() {
                            self.data = Some(
                                bulletin_categorizer(file, self.config.clone())
                                    .into_iter()
                                    .map(|(k, v)| (k, v, false))
                                    .collect(),
                            );
                        }
                    }
                    if ui.button("Save File").clicked() {
                        if self.data != None {
                            save_obs_file(build_livestream(
                                self.data
                                    .as_ref()
                                    .unwrap()
                                    .clone()
                                    .into_iter()
                                    .map(|(x, y, _)| (x, y))
                                    .collect(),
                            ));
                        }
                    }
                    if ui.button("Apply Number").clicked() {
                        if self.data != None {
                            self.data = Some(apply_number(
                                self.data.clone().unwrap(),
                                self.paragraph_value,
                            ));
                        }
                    }
                    egui::ComboBox::from_label("")
                        .selected_text(NUMBER_MEANING[self.paragraph_value as usize])
                        .height(f32::INFINITY)
                        .show_ui(ui, |ui| {
                            for i in 0..10 {
                                ui.selectable_value(
                                    &mut self.paragraph_value,
                                    i as u8,
                                    NUMBER_MEANING[i],
                                );
                            }
                        });
                });
                if let Some(data) = &mut self.data {
                    egui::ScrollArea::vertical().show(ui, |ui| {
                        for i in 0..data.len() {
                            ui.horizontal(|ui| {
                                display_row(data, ui, i);
                            });
                        }
                    });
                }
            }
            _ => (),
        }
    }
}

fn apply_number(data: Vec<(u8, String, bool)>, value: u8) -> Vec<(u8, String, bool)> {
    data.into_iter()
        .map(|(x, y, z)| {
            if z == true {
                (value, y, false)
            } else {
                (x, y, z)
            }
        })
        .collect()
}

fn load_file() -> Option<Vec<String>> {
    let Some(path) = rfd::FileDialog::new()
        .add_filter("text", &["txt"])
        .add_filter("all", &["*"])
        .pick_file()
    else {
        return None;
    };
    let Ok(f) = File::open(path) else {
        return None;
    };
    Some(
        BufReader::new(f)
            .lines()
            .map(|x| x.unwrap_or("".to_string()))
            .collect(),
    )
}

fn display_row(data: &mut Vec<(u8, String, bool)>, ui: &mut egui::Ui, i: usize) {
    ui.add_space(2.0);
    ui.label(
        RichText::new(data[i].0.to_string())
            .size(20.0)
            .color(Color32::ORANGE),
    );
    ui.checkbox(&mut data[i].2, "")
        .set_intrinsic_size(Vec2 { x: 20.0, y: 20.0 });
    let label = ui
        .add(
            egui::Label::new(RichText::new(&data[i].1).size(20.0))
                .wrap()
                .sense(egui::Sense::click()),
        )
        .on_hover_cursor(egui::CursorIcon::Default);
    if label.clicked() {
        data[i].2 = !data[i].2;
    }
}

pub fn ui() -> eframe::Result<()> {
    eframe::run_native(
        "OBS Church Automator",
        eframe::NativeOptions::default(),
        Box::new(|cc| Ok(Box::new(Ui::new(cc)))),
    )
}
