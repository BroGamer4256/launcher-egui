use eframe::{egui, epi};
use serde::{Deserialize, Serialize};

fn main() {
	eframe::run_native(
		Box::new(App {
			current_tab: "window",
			..Default::default()
		}),
		eframe::NativeOptions {
			initial_window_size: Some(egui::vec2(450.0, 475.0)),
			..Default::default()
		},
	)
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Serialize, Deserialize)]
pub enum DisplayFormat {
	Windowed,
	Popup,
	Exclusive,
	Borderless,
}

impl Default for DisplayFormat {
	fn default() -> Self {
		Self::Windowed
	}
}

impl std::fmt::Display for DisplayFormat {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		let val = match self {
			DisplayFormat::Windowed => "Windowed",
			DisplayFormat::Popup => "Popup",
			DisplayFormat::Exclusive => "Exclusive Fullscreen",
			DisplayFormat::Borderless => "Borderless",
		};
		f.write_str(val)
	}
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Serialize, Deserialize)]
pub enum StatusIcons {
	Default,
	Hidden,
	Error,
	Ok,
	PartialOk,
}

impl Default for StatusIcons {
	fn default() -> Self {
		Self::Default
	}
}

impl std::fmt::Display for StatusIcons {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		let val = match self {
			StatusIcons::Default => "Default",
			StatusIcons::Hidden => "Hidden",
			StatusIcons::Error => "Error",
			StatusIcons::Ok => "Ok",
			StatusIcons::PartialOk => "Partial Ok",
		};
		f.write_str(val)
	}
}

#[derive(Default, Debug, PartialEq, Clone, Copy, Serialize, Deserialize)]
pub struct Config {
	patches: Patches,
	io: Io,
	window: Window,
	graphics: Graphics,
}

#[derive(Default, Debug, PartialEq, Clone, Copy, Serialize, Deserialize)]
pub struct Patches {
	disable_movies: bool,
	disable_volume_buttons: bool,
	disable_error_banner: bool,
	disable_credits_text: bool,
	disable_selection_timer: bool,
	disable_selection_timer_sprite: bool,
	disable_pv_watermark: bool,
	disable_pv_ui: bool,
	disable_lyrics: bool,
	stage_count: i32,
	status_icons: StatusIcons,
	disable_credits: bool,
	camera_control: bool,
	fast_loading: bool,
	hold_transfer: bool,
	pausing: bool,
	score_saving: bool,
}

#[derive(Default, Debug, PartialEq, Clone, Copy, Serialize, Deserialize)]
pub struct Io {
	input_emulator: bool,
	slider_emulator: bool,
}

#[derive(Default, Debug, PartialEq, Clone, Copy, Serialize, Deserialize)]
pub struct Window {
	display_format: DisplayFormat,
	match_screen: bool,
	x: i32,
	y: i32,
	internal_match_screen: bool,
	internal_x: i32,
	internal_y: i32,
	scaling: bool,
}

#[derive(Default, Debug, PartialEq, Clone, Copy, Serialize, Deserialize)]
pub struct Graphics {
	taa: bool,
	mlaa: bool,
	glare: bool,
	dof: bool,
	frame_rate: i32,
}

#[derive(Default, Serialize, Deserialize)]
pub struct App {
	path: std::path::PathBuf,
	#[cfg(not(target_os = "windows"))]
	wine_prefix: std::path::PathBuf,
	#[serde(skip)]
	current_tab: &'static str,

	config: Config,
}

impl epi::App for App {
	fn name(&self) -> &str {
		"launcher-egui"
	}

	fn setup(
		&mut self,
		ctx: &egui::CtxRef,
		_frame: &epi::Frame,
		_storage: Option<&dyn epi::Storage>,
	) {
		let mut fonts = egui::FontDefinitions::default();
		fonts.family_and_size.insert(
			egui::TextStyle::Body,
			(egui::FontFamily::Proportional, 18.0),
		);
		fonts.family_and_size.insert(
			egui::TextStyle::Button,
			(egui::FontFamily::Proportional, 19.0),
		);
		ctx.set_fonts(fonts);

		if let Some(storage) = _storage {
			*self = epi::get_value(storage, epi::APP_KEY).unwrap_or_default()
		}

		while !self.path.exists() && self.path.file_name().unwrap_or_default() != "diva.exe" {
			self.path = rfd::FileDialog::new()
				.add_filter("exe", &["exe"])
				.set_directory(".")
				.set_file_name("diva.exe")
				.set_title("Select diva.exe")
				.pick_file()
				.unwrap_or_default();
		}

		if !std::path::Path::new(&self.path.parent().unwrap().join("plugins/config.toml")).exists()
		{
			return;
		}
		let config_str =
			std::fs::read_to_string(&self.path.parent().unwrap().join("plugins/config.toml"))
				.unwrap();
		self.config = toml::from_str(config_str.as_str()).unwrap();
	}

	fn save(&mut self, storage: &mut dyn epi::Storage) {
		epi::set_value(storage, epi::APP_KEY, self);
	}

	fn on_exit(&mut self) {
		let config_str = toml::to_string(&self.config).unwrap();
		std::fs::write(
			&self.path.parent().unwrap().join("plugins/config.toml"),
			config_str,
		)
		.unwrap();
	}

	fn update(&mut self, ctx: &egui::CtxRef, frame: &epi::Frame) {
		egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
			egui::menu::bar(ui, |ui| {
				ui.selectable_value(&mut self.current_tab, "window", "Window Options");
				ui.selectable_value(&mut self.current_tab, "graphics", "Graphics Options");
				ui.selectable_value(&mut self.current_tab, "patches", "Patches");
				ui.selectable_value(&mut self.current_tab, "io", "IO");
			});
		});

		egui::TopBottomPanel::bottom("bottom_panel").show(ctx, |ui| {
			ui.horizontal(|ui| {
				if ui
					.add_sized(
						vec2_x_modify(&mut ui.available_size(), 2.0),
						egui::Button::new("Launch"),
					)
					.clicked()
				{
					#[cfg(not(target_os = "windows"))]
					std::process::Command::new("wine")
						.env("WINEPREFIX", self.wine_prefix.as_path())
						.env("WINEDLLOVERRIDES", "dinput8=n,b")
						.arg(self.path.as_path())
						.arg("--launch")
						.spawn()
						.unwrap();
					#[cfg(target_os = "windows")]
					std::process::Command::new(self.path.as_path())
						.arg("--launch")
						.spawn()
						.unwrap();

					frame.quit();
				}
				if ui
					.add_sized(ui.available_size(), egui::Button::new("Quit"))
					.clicked()
				{
					frame.quit();
				}
			});
		});

		egui::CentralPanel::default().show(ctx, |ui| {
			egui::ScrollArea::both().show(ui, |ui| {
				match self.current_tab {
					"window" => App::draw_window_tab(self, ui),
					"graphics" => App::draw_graphics_tab(self, ui),
					"patches" => App::draw_patches_tab(self, ui),
					"io" => App::draw_io_tab(self, ui),
					_ => (),
				};
			});
		});
	}
}

impl App {
	fn draw_window_tab(&mut self, ui: &mut egui::Ui) {
		const VARIANTS: [DisplayFormat; 4] = [
			DisplayFormat::Windowed,
			DisplayFormat::Popup,
			DisplayFormat::Exclusive,
			DisplayFormat::Borderless,
		];
		ui.horizontal(|ui| {
			ui.label("Display type");
			egui::ComboBox::from_label("")
				.selected_text(self.config.window.display_format.to_string())
				.width(ui.available_width() / 4.0)
				.show_ui(ui, |ui| {
					for variant in VARIANTS {
						let display = variant.to_string();
						ui.selectable_value(
							&mut self.config.window.display_format,
							variant,
							display,
						);
					}
				});
		});

		ui.horizontal(|ui| {
			ui.label("Window Resolution");
			simple_checkbox(
				"Match Screen Size",
				&mut self.config.window.match_screen,
				ui,
			);

			int_text_box(&mut self.config.window.x, 2.0, ui);
			int_text_box(&mut self.config.window.y, 1.0, ui);
		});

		ui.horizontal(|ui| {
			ui.label("Internal Resolution");
			simple_checkbox(
				"Match Screen Size",
				&mut self.config.window.internal_match_screen,
				ui,
			);

			int_text_box(&mut self.config.window.internal_x, 2.0, ui);
			int_text_box(&mut self.config.window.internal_y, 1.0, ui);
		});

		simple_checkbox("Window scaling", &mut self.config.window.scaling, ui);

		#[cfg(not(target_os = "windows"))]
		ui.horizontal(|ui| {
			if ui.button("Set wine prefix").clicked() {
				self.wine_prefix = rfd::FileDialog::new().pick_folder().unwrap_or_default();
			}
			ui.add_sized(
				ui.available_size(),
				egui::TextEdit::singleline(&mut self.wine_prefix.to_str().unwrap_or_default()),
			);
		});

		ui.horizontal(|ui| {
			if ui.button("Set diva.exe path").clicked() {
				self.path = rfd::FileDialog::new()
					.add_filter("exe", &["exe"])
					.pick_file()
					.unwrap_or_default();
			}
			ui.add_sized(
				ui.available_size(),
				egui::TextEdit::singleline(&mut self.path.to_str().unwrap_or_default()),
			);
		});
	}

	fn draw_graphics_tab(&mut self, ui: &mut egui::Ui) {
		ui.horizontal(|ui| {
			ui.add(egui::Label::new("Frame rate limit"))
				.on_hover_text("-1 to unlock the frame rate");

			int_text_box(&mut self.config.graphics.frame_rate, 2.0, ui);
		});

		simple_checkbox("Motion Blur", &mut self.config.graphics.taa, ui);
		simple_checkbox("MLAA", &mut self.config.graphics.mlaa, ui);
		simple_checkbox("Glare", &mut self.config.graphics.glare, ui);
		simple_checkbox("Depth of Field", &mut self.config.graphics.dof, ui);
	}

	fn draw_patches_tab(&mut self, ui: &mut egui::Ui) {
		simple_checkbox(
			"Disable Movies",
			&mut self.config.patches.disable_movies,
			ui,
		);

		simple_checkbox(
			"Disable credits check",
			&mut self.config.patches.disable_credits,
			ui,
		);
		simple_checkbox(
			"Disable selection timer",
			&mut self.config.patches.disable_selection_timer,
			ui,
		);
		simple_checkbox(
			"Disable selection timer sprite",
			&mut self.config.patches.disable_selection_timer_sprite,
			ui,
		);
		ui.horizontal(|ui| {
			ui.add(egui::Label::new("Number of stages"))
				.on_hover_text("Set to 0 if using online");
			int_text_box(&mut self.config.patches.stage_count, 5.0, ui);
		});

		simple_checkbox("Fast Loading", &mut self.config.patches.fast_loading, ui);

		simple_checkbox_tooltip(
			"Camera Controller",
			"Press F3 to activate",
			&mut self.config.patches.camera_control,
			ui,
		);
		simple_checkbox("Hold transfers", &mut self.config.patches.hold_transfer, ui);
		simple_checkbox("Score saving", &mut self.config.patches.score_saving, ui);
		simple_checkbox_tooltip(
			"Pause menu",
			"Entered by pressing JVS_START\nStart on controller or Enter on keyboard by default",
			&mut self.config.patches.pausing,
			ui,
		);

		const VARIANTS: [StatusIcons; 5] = [
			StatusIcons::Default,
			StatusIcons::Hidden,
			StatusIcons::Error,
			StatusIcons::Ok,
			StatusIcons::PartialOk,
		];
		ui.horizontal(|ui| {
			ui.label("Status icons");
			egui::ComboBox::from_label("")
				.selected_text(self.config.patches.status_icons.to_string())
				.width(ui.available_width() / 4.0)
				.show_ui(ui, |ui| {
					for variant in VARIANTS {
						let display = variant.to_string();
						ui.selectable_value(
							&mut self.config.patches.status_icons,
							variant,
							display,
						);
					}
				});
		});

		simple_checkbox(
			"Hide Volume Buttons",
			&mut self.config.patches.disable_volume_buttons,
			ui,
		);
		simple_checkbox(
			"Hide PV Photo UI",
			&mut self.config.patches.disable_pv_ui,
			ui,
		);
		simple_checkbox(
			"Hide PV Watermark",
			&mut self.config.patches.disable_pv_watermark,
			ui,
		);
		simple_checkbox("Hide Lyrics", &mut self.config.patches.disable_lyrics, ui);
		simple_checkbox(
			"Hide Error banner",
			&mut self.config.patches.disable_error_banner,
			ui,
		);
		simple_checkbox(
			"Hide CREDITS text",
			&mut self.config.patches.disable_credits_text,
			ui,
		);
	}

	fn draw_io_tab(&mut self, ui: &mut egui::Ui) {
		simple_checkbox("Input Emulation", &mut self.config.io.input_emulator, ui);
		simple_checkbox("Slider Emulation", &mut self.config.io.slider_emulator, ui);
	}
}

#[allow(unused_assignments)]
fn int_text_box(value: &mut i32, size: f32, ui: &mut egui::Ui) {
	let mut value_str = value.to_string();
	ui.add_sized(
		vec2_x_modify(&mut ui.available_size(), size),
		egui::TextEdit::singleline(&mut value_str),
	);

	if value_str.parse::<i32>().is_ok() {
		*value = value_str.parse().unwrap();
	} else if value_str == "" {
		*value = 0;
	}
}

fn simple_checkbox(label: &str, value: &mut bool, ui: &mut egui::Ui) {
	ui.horizontal(|ui| {
		ui.label(label);
		ui.checkbox(value, "");
	});
}

fn simple_checkbox_tooltip(label: &str, tooltip: &str, value: &mut bool, ui: &mut egui::Ui) {
	ui.horizontal(|ui| {
		ui.add(egui::Label::new(label)).on_hover_text(tooltip);
		ui.add(egui::Checkbox::new(value, ""))
			.on_hover_text(tooltip);
	});
}

fn vec2_x_modify(size: &mut egui::Vec2, modify: f32) -> egui::Vec2 {
	size.x /= modify;
	*size
}
