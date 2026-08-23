use crate::backend::{
    build_livestream, bulletin_categorizer, get_config, get_help, preview_builder, save_obs_file,
    Config, Scene,
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
    "3 - Hymn",
    "4 - Call and Response",
    "5 - Blank Scene",
    "6 - Service Name",
    "7 - Ignore Text",
    "8 - Special Music",
    "9 - Add to Previous",
];

#[derive(PartialEq)]
enum UiState {
    Preview,
    Main,
    Edit,
}

impl UiState {
    fn get_state_string(&self) -> String {
        match self {
            UiState::Edit => "Edit Mode".to_string(),
            UiState::Preview => "Preview Mode".to_string(),
            UiState::Main => "Main Mode".to_string(),
        }
    }
}

struct Ui {
    config: Config,
    data: Option<Vec<(u8, String, bool)>>,
    state: UiState,
    paragraph_value: u8,
    slide: usize,
    scenes: Vec<Scene>,
    compute_scenes: bool,
    file_save_text: Option<std::time::Instant>,
    editing_paragraph: Option<(String, usize)>,
    make_data_2_false: bool,
}

impl Ui {
    fn new(cc: &eframe::CreationContext<'_>) -> Self {
        let mut fonts = egui::FontDefinitions::default();
        fonts.font_data.insert(
            "roboto".to_owned(),
            std::sync::Arc::new(egui::FontData::from_static(include_bytes!(
                "../assets/roboto.ttf"
            ))),
        );
        fonts
            .families
            .entry(egui::FontFamily::Proportional)
            .or_default()
            .insert(0, "roboto".to_owned());
        cc.egui_ctx.set_fonts(fonts);

        Ui {
            config: get_config(),
            data: None,
            state: UiState::Main,
            paragraph_value: 0,
            slide: 0,
            scenes: vec![],
            compute_scenes: false,
            file_save_text: None,
            editing_paragraph: None,
            make_data_2_false: false,
        }
    }
}

impl eframe::App for Ui {
    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        ui.add_space(16.0);
        ui.horizontal(|ui| {
            ui.add_space(16.0);
            ui.heading(RichText::new("OBS Church Automator").size(24.0));
            ui.push_id("State Box", |ui| {
                egui::ComboBox::from_label("")
                    .selected_text(RichText::new(self.state.get_state_string()).size(16.0))
                    .height(f32::INFINITY)
                    .show_ui(ui, |ui| {
                        ui.selectable_value(
                            &mut self.state,
                            UiState::Main,
                            RichText::new(UiState::Main.get_state_string()).size(16.0),
                        );
                        ui.selectable_value(
                            &mut self.state,
                            UiState::Preview,
                            RichText::new(UiState::Preview.get_state_string()).size(16.0),
                        );
                        ui.selectable_value(
                            &mut self.state,
                            UiState::Edit,
                            RichText::new(UiState::Edit.get_state_string()).size(16.0),
                        );
                    });
            });
            if ui.button(RichText::new("Help").size(16.0)).clicked() {
                get_help();
            }
        });
        match self.state {
            UiState::Main => {
                if !self.make_data_2_false {
                    self.make_data_2_false = true;
                }
                ui.horizontal(|ui| {
                    ui.add_space(16.0);
                    if ui.button(RichText::new("Load File").size(16.0)).clicked() {
                        if let Some(file) = load_file() {
                            self.data = Some(
                                bulletin_categorizer(file, self.config.clone())
                                    .into_iter()
                                    .map(|(k, v)| (k, v, false))
                                    .collect(),
                            );
                            self.compute_scenes = false;
                            self.scenes = vec![];
                        }
                    }
                    if ui.button(RichText::new("Save File").size(16.0)).clicked() {
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
                            self.file_save_text =
                                Some(std::time::Instant::now() + std::time::Duration::from_secs(3));
                        }
                    }
                    if ui
                        .button(RichText::new("Apply Number").size(16.0))
                        .clicked()
                    {
                        if self.data != None {
                            self.data = Some(apply_number(
                                self.data.clone().unwrap(),
                                self.paragraph_value,
                            ));
                            self.compute_scenes = true;
                        }
                    }
                    let pressed_num = pressing_number(ui);
                    if pressed_num.0 {
                        if self.data != None {
                            self.data =
                                Some(apply_number(self.data.clone().unwrap(), pressed_num.1));
                            self.paragraph_value = pressed_num.1;
                            self.compute_scenes = true;
                        }
                    }
                    egui::ComboBox::from_label("")
                        .selected_text(
                            RichText::new(NUMBER_MEANING[self.paragraph_value as usize]).size(16.0),
                        )
                        .height(f32::INFINITY)
                        .show_ui(ui, |ui| {
                            for i in 0..10 {
                                ui.selectable_value(
                                    &mut self.paragraph_value,
                                    i as u8,
                                    RichText::new(NUMBER_MEANING[i]).size(16.0),
                                );
                            }
                        });
                    if self.file_save_text > Some(std::time::Instant::now()) {
                        ui.label(RichText::new("File Saved!!").color(Color32::ORANGE));
                        ui.ctx().request_repaint();
                    } else if self.file_save_text != None {
                        self.file_save_text = None;
                    }
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
            UiState::Preview => {
                if self.compute_scenes {
                    self.compute_scenes = false;
                    self.scenes = preview_builder(
                        self.data
                            .clone()
                            .unwrap_or(vec![])
                            .into_iter()
                            .map(|x| (x.0, x.1))
                            .collect(),
                    );
                }
                ui.horizontal(|ui| {
                    ui.add_space(16.0);
                    if ui
                        .button(RichText::new("Previous Scene").size(16.0))
                        .clicked()
                    {
                        if self.slide != 0 {
                            self.slide -= 1;
                        }
                    }
                    if ui.button(RichText::new("Next Scene").size(16.0)).clicked() {
                        if self.slide + 1 < self.scenes.len() {
                            self.slide += 1;
                        }
                    }
                });
                if !self.scenes.is_empty() {
                    ui.add_space(16.0);
                    ui.horizontal(|ui| {
                        ui.add_space(16.0);
                        ui.label(format!("scn_{}", self.scenes[self.slide].name.clone()));
                    });
                    if self.scenes[self.slide].will_it_scroll {
                        ui.horizontal(|ui| {
                            ui.add_space(16.0);
                            ui.label("Text will scroll in OBS");
                        });
                    } else {
                        ui.label("");
                    }
                    ui.horizontal(|ui| {
                        ui.add_space(16.0);
                        ui.label(
                            RichText::new(
                                self.scenes[self.slide]
                                    .contents
                                    .clone()
                                    .unwrap_or("".to_string()),
                            )
                            .color(self.scenes[self.slide].col)
                            .background_color(self.scenes[self.slide].bg),
                        );
                    });
                }
            }
            UiState::Edit => match &self.editing_paragraph {
                Some((s, i)) => {}
                _ => match &mut self.data {
                    Some(data) => {
                        if self.make_data_2_false {
                            for d in data.iter_mut() {
                                d = (d.0, d.1, false);
                            }
                            self.make_data_2_false = false;
                        }
                        for (i, (_, s, b)) in data.iter().enumerate() {
                            if b == &true {
                                self.editing_paragraph = Some((s.clone(), i));
                            }
                        }
                        egui::ScrollArea::vertical().show(ui, |ui| {
                            for i in 0..data.len() {
                                ui.horizontal(|ui| {
                                    display_row(data, ui, i);
                                });
                            }
                        });
                    }
                    _ => (),
                },
            },
        }
    }
}

fn pressing_number(ui: &mut egui::Ui) -> (bool, u8) {
    if ui.input(|i| i.key_pressed(egui::Key::Num0)) {
        (true, 0)
    } else if ui.input(|i| i.key_pressed(egui::Key::Num1)) {
        (true, 1)
    } else if ui.input(|i| i.key_pressed(egui::Key::Num2)) {
        (true, 2)
    } else if ui.input(|i| i.key_pressed(egui::Key::Num3)) {
        (true, 3)
    } else if ui.input(|i| i.key_pressed(egui::Key::Num4)) {
        (true, 4)
    } else if ui.input(|i| i.key_pressed(egui::Key::Num5)) {
        (true, 5)
    } else if ui.input(|i| i.key_pressed(egui::Key::Num6)) {
        (true, 6)
    } else if ui.input(|i| i.key_pressed(egui::Key::Num7)) {
        (true, 7)
    } else if ui.input(|i| i.key_pressed(egui::Key::Num8)) {
        (true, 8)
    } else if ui.input(|i| i.key_pressed(egui::Key::Num9)) {
        (true, 9)
    } else {
        (false, 0)
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
