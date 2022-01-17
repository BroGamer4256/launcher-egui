pub mod components;
pub mod graphics;
pub mod patch;
mod utils;
pub mod video;

use utils::*;

use std::fmt::Display;

use components::AppComponents;
use detour::static_detour;
use eframe::{egui, epi};
use graphics::AppGraphics;
use ini::{Ini, Properties, SectionSetter};
use patch::AppPatches;
use video::AppVideo;

#[no_mangle]
extern "system" fn DllMain(_: u32, reason: u32, _: *const u8) -> u32 {
	if reason != 1 {
		return 1;
	}
	for argument in std::env::args() {
		if argument == "--launch" {
			return 1;
		}
	}

	unsafe {
		let diva_main_ptr: FnMain = std::mem::transmute(0x140194D90 as *const ());

		divaMain
			.initialize(diva_main_ptr, hooked_main)
			.unwrap()
			.enable()
			.unwrap();
	}

	1
}

type FnMain = unsafe extern "cdecl" fn(i32, *const *const i8, *const *const i8) -> i32;
static_detour! {
	static divaMain: unsafe extern "cdecl" fn(i32, *const *const i8, *const *const i8) -> i32;
}
fn hooked_main(_: i32, _: *const *const i8, _: *const *const i8) -> i32 {
	eframe::run_native(
		Box::new(App {
			current_tab: "video",
			..Default::default()
		}),
		eframe::NativeOptions {
			initial_window_size: Some(egui::vec2(450.0, 4750.0)),
			..Default::default()
		},
	)
}

#[repr(u32)] // is this needed?
#[derive(Debug, PartialEq, Eq)]
pub enum DisplayFormat {
	Windowed,
	Popup,
	Exclusive,
	Borderless,
}

impl DisplayFormat {
	//TODO: find a better way to do this
	const fn from_u8(val: u8) -> Option<Self> {
		const WINDOWED: u8 = DisplayFormat::Windowed as u8;
		const POPUP: u8 = DisplayFormat::Popup as u8;
		const EXCLUSIVE: u8 = DisplayFormat::Exclusive as u8;
		const BORDERLESS: u8 = DisplayFormat::Borderless as u8;
		match val {
			WINDOWED => Some(Self::Windowed),
			POPUP => Some(Self::Popup),
			EXCLUSIVE => Some(Self::Exclusive),
			BORDERLESS => Some(Self::Borderless),
			_ => None,
		}
	}
}

impl Default for DisplayFormat {
	fn default() -> Self {
		Self::Windowed
	}
}

impl Display for DisplayFormat {
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

#[repr(u32)] // is this needed?
#[derive(Debug, PartialEq, Eq)]
pub enum StatusIcons {
	Default,
	Hidden,
	Error,
	Ok,
	PartialOk,
}

impl StatusIcons {
	//TODO: find a better way to do this
	const fn from_u8(val: u8) -> Option<Self> {
		const DEFAULT: u8 = StatusIcons::Default as u8;
		const HIDDEN: u8 = StatusIcons::Hidden as u8;
		const ERROR: u8 = StatusIcons::Error as u8;
		const OK: u8 = StatusIcons::Ok as u8;
		const PARTIALOK: u8 = StatusIcons::PartialOk as u8;
		match val {
			DEFAULT => Some(Self::Default),
			HIDDEN => Some(Self::Hidden),
			ERROR => Some(Self::Error),
			OK => Some(Self::Ok),
			PARTIALOK => Some(Self::PartialOk),
			_ => None,
		}
	}
}

impl Default for StatusIcons {
	fn default() -> Self {
		Self::Default
	}
}

impl Display for StatusIcons {
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

#[derive(Default)]
pub struct App {
	current_tab: &'static str,

	video: AppVideo,
	graphics: AppGraphics,
	patches: AppPatches,
	components: AppComponents,
}

impl epi::App for App {
	fn name(&self) -> &str {
		"launcher-egui`"
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
			(egui::FontFamily::Proportional, 18.5),
		);
		ctx.set_fonts(fonts);

		if !std::path::Path::new("plugins/config.ini").exists()
			|| !std::path::Path::new("plugins/components.ini").exists()
		{
			return;
		}
		let config_ini = ini::Ini::load_from_file("plugins/config.ini").unwrap();

		let resolution_section = config_ini.section(Some("Resolution")).unwrap();
		self.video = AppVideo::read(&config_ini).unwrap().unwrap();
		self.graphics = AppGraphics::read(&config_ini).unwrap().unwrap();
		self.patches = AppPatches::read(&config_ini).unwrap().unwrap();

		let components_ini = ini::Ini::load_from_file("plugins/components.ini").unwrap();
		self.components = AppComponents::read(&components_ini).unwrap().unwrap();
	}

	fn on_exit(&mut self) {
		let mut config_ini = if std::path::Path::new("plugins/config.ini").exists() {
			ini::Ini::load_from_file("plugins/config.ini").unwrap()
		} else {
			ini::Ini::new()
		};

		self.video.write(&self.graphics, &mut config_ini);
		self.graphics.write(&mut config_ini);
		self.patches.write(&mut config_ini);

		config_ini.write_to_file("plugins/config.ini").unwrap();

		let mut components_ini = if std::path::Path::new("plugins/components.ini").exists() {
			ini::Ini::load_from_file("plugins/components.ini").unwrap()
		} else {
			ini::Ini::new()
		};

		self.components.write(&self.patches, &mut components_ini);

		components_ini
			.write_to_file("plugins/components.ini")
			.unwrap();
	}

	fn update(&mut self, ctx: &egui::CtxRef, frame: &epi::Frame) {
		egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
			egui::menu::bar(ui, |ui| {
				ui.selectable_value(&mut self.current_tab, "video", "Video Options");
				ui.selectable_value(&mut self.current_tab, "patches", "Patches");
				ui.selectable_value(&mut self.current_tab, "ui", "UI Options");
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
					std::process::Command::new("diva.exe")
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
					"video" => App::draw_video_tab(self, ui),
					"patches" => App::draw_patches_tab(self, ui),
					"ui" => App::draw_ui_tab(self, ui),
					_ => (),
				};
			});
		});
	}
}

impl App {
	fn draw_video_tab(&mut self, ui: &mut egui::Ui) {
		const VARIANTS: [DisplayFormat; 4] = [
			DisplayFormat::Windowed,
			DisplayFormat::Popup,
			DisplayFormat::Exclusive,
			DisplayFormat::Borderless,
		];
		ui.horizontal(|ui| {
			ui.label("Display type");
			egui::ComboBox::from_label("")
				.selected_text(self.video.display_format.to_string())
				.width(ui.available_width() / 4.0)
				.show_ui(ui, |ui| {
					for variant in VARIANTS {
						let display = variant.to_string();
						ui.selectable_value(&mut self.video.display_format, variant, display);
					}
				});
		});

		ui.horizontal(|ui| {
			ui.add(egui::Label::new("Window Size"))
				.on_hover_text("-1 in both means to match the size of the screen");

			int_text_box(&mut self.video.window_size_x, 2.0, ui);
			int_text_box(&mut self.video.window_size_y, 1.0, ui);
		});

		ui.horizontal(|ui| {
			ui.add(egui::Label::new("Internal Resolution"))
				.on_hover_text("-1 in both means to match the size of the window");
			ui.checkbox(&mut self.video.internal_resolution_enabled, "");

			int_text_box(&mut self.video.internal_resolution_x, 2.0, ui);
			int_text_box(&mut self.video.internal_resolution_y, 1.0, ui);
		});

		ui.horizontal(|ui| {
			ui.add(egui::Label::new("Frame rate limit"))
				.on_hover_text("-1 to unlock the frame rate");

			int_text_box(&mut self.graphics.frame_rate, 2.0, ui);
		});

		simple_checkbox("Motion Blur", &mut self.graphics.taa_enabled, ui);
		simple_checkbox("MLAA", &mut self.graphics.mlaa_enabled, ui);
		simple_checkbox("Glare", &mut self.graphics.glare_enabled, ui);
		simple_checkbox(
			"Depth of Field",
			&mut self.graphics.depth_of_field_enabled,
			ui,
		);
		simple_checkbox("Reflections", &mut self.graphics.reflections_enabled, ui);
		simple_checkbox("Shadows", &mut self.graphics.shadows_enabled, ui);
		simple_checkbox("Transparency", &mut self.graphics.transparency_enabled, ui);
		simple_checkbox("Disable 3D", &mut self.graphics.disable_3d, ui);
		simple_checkbox("Window scaling", &mut self.components.enable_scaling, ui);

		ui.horizontal(|ui| {
			ui.label("Gamma");
			ui.add(egui::Slider::new(&mut self.graphics.gamma, 0..=200));
		});
	}

	fn draw_patches_tab(&mut self, ui: &mut egui::Ui) {
		simple_checkbox("Disable Movies", &mut self.patches.disable_movies, ui);
		simple_checkbox("Show Cursor", &mut self.patches.show_cursor, ui);
		simple_checkbox_tooltip(
			"Disable Slider Emulation",
			"Must be checked if using a hardware slider",
			&mut self.patches.disable_slider_emu,
			ui,
		);

		// should these really be separate?
		// would anyone want touch emulation without input emulation or vice versa?
		simple_checkbox(
			"Input Emulation",
			&mut self.components.enable_input_emulator,
			ui,
		);
		simple_checkbox(
			"Touch Emulation",
			&mut self.components.enable_touch_emulator,
			ui,
		);

		simple_checkbox(
			"Disable credits check",
			&mut self.patches.disable_credits,
			ui,
		);
		simple_checkbox(
			"Disable selection timer",
			&mut self.patches.disable_selection_timer,
			ui,
		);
		ui.horizontal(|ui| {
			ui.add(egui::Label::new("Number of stages"))
				.on_hover_text("Set to 0 if using online");
			int_text_box(&mut self.patches.stage_count, 5.0, ui);
		});

		ui.horizontal(|ui| {
			ui.add(egui::Label::new("Fast Loader"))
				.on_hover_text("Set to 3 or lower when using online");
			ui.checkbox(&mut self.components.enable_fast_loader, "");
			ui.add(egui::Slider::new(
				&mut self.components.fast_loader_speed,
				0..=100,
			));
		});

		simple_checkbox(
			"Custom player data",
			&mut self.components.enable_player_data_manager,
			ui,
		);
		simple_checkbox_tooltip(
			"Frame Rate Manager",
			"Speeds up animations to match the frame rate\nStops certain sounds from playing",
			&mut self.components.enable_frame_rate_manager,
			ui,
		);
		simple_checkbox_tooltip(
			"Camera Controller",
			"Press F3 to activate",
			&mut self.components.enable_camera_controller,
			ui,
		);
		simple_checkbox("Enable debug menus", &mut self.components.enable_dwgui, ui);
		simple_checkbox(
			"Hold transfers",
			&mut self.components.enable_hold_transfer,
			ui,
		);
		simple_checkbox("Score saving", &mut self.components.enable_score_saving, ui);
		simple_checkbox_tooltip(
			"Pause menu",
			"Entered by pressing JVS_START\nStart on controller or Enter on keyboard by default",
			&mut self.components.enable_pausing,
			ui,
		);
	}

	fn draw_ui_tab(&mut self, ui: &mut egui::Ui) {
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
				.selected_text(self.patches.status_icons.to_string())
				.width(ui.available_width() / 4.0)
				.show_ui(ui, |ui| {
					for variant in VARIANTS {
						let display = variant.to_string();
						ui.selectable_value(&mut self.patches.status_icons, variant, display);
					}
				});
		});

		simple_checkbox(
			"Hide Volume Buttons",
			&mut self.patches.disable_volume_buttons,
			ui,
		);
		simple_checkbox("Hide PV Photo UI", &mut self.patches.disable_photo_ui, ui);
		simple_checkbox("Hide PV Watermark", &mut self.patches.disable_watermark, ui);
		simple_checkbox("Hide Lyrics", &mut self.patches.disable_lyrics, ui);
		simple_checkbox(
			"Hide Error banner",
			&mut self.patches.disable_error_banner,
			ui,
		);
		simple_checkbox(
			"Hide CREDITS text",
			&mut self.patches.disable_credits_text,
			ui,
		);
		simple_checkbox(
			"Swap CREDITS text with PD LOADER",
			&mut self.patches.show_pdloader_text,
			ui,
		);
		simple_checkbox(
			"Hide Selection timer",
			&mut self.patches.disable_timer_sprite,
			ui,
		);
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

#[inline]
fn vec2_x_modify(size: &mut egui::Vec2, modify: f32) -> egui::Vec2 {
	size.x /= modify;
	*size
}

fn get_ini_value<T>(section: &ini::Properties, key: &str) -> T
where
	T: From<bool>,
	T: std::str::FromStr,
	<T as std::str::FromStr>::Err: std::fmt::Debug,
	T: std::default::Default,
{
	let value = section.get(key);
	if value.is_none() {
		return Default::default();
	}
	if std::any::type_name::<T>() == "bool" {
		return T::from(value.unwrap() == "1" || value.unwrap() == "true");
	}
	let value: Result<T, T::Err> = value.unwrap().parse();
	if value.is_err() {
		return Default::default();
	}
	value.unwrap()
}
