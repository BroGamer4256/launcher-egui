use detour::static_detour;
use eframe::{egui, epi};

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
			initial_window_size: Some(egui::vec2(400.0, 480.0)),
			..Default::default()
		},
	)
}

pub type DisplayFormat = u32;
pub const WINDOWED: DisplayFormat = 0;
pub const POPUP: DisplayFormat = 1;
pub const EXCLUSIVE: DisplayFormat = 2;
pub const BORDERLESS: DisplayFormat = 3;

pub type StatusIcons = u32;
pub const DEFAULT: StatusIcons = 0;
pub const HIDDEN: StatusIcons = 1;
pub const ERROR: StatusIcons = 2;
pub const OK: StatusIcons = 3;
pub const PARTIALOK: StatusIcons = 4;

pub struct App {
	current_tab: &'static str,

	// Resolution
	display_format: DisplayFormat,
	window_size_x: i32,
	window_size_y: i32,
	internal_resolution_enabled: bool,
	internal_resolution_x: i32,
	internal_resolution_y: i32,

	// Graphics
	taa_enabled: bool,
	mlaa_enabled: bool,
	glare_enabled: bool,
	depth_of_field_enabled: bool,
	frame_rate: i32,
	gamma: i32,
	reflections_enabled: bool,
	shadows_enabled: bool,
	transparency_enabled: bool,
	disable_3d: bool,

	// Patches
	disable_movies: bool,
	show_cursor: bool,
	disable_slider_emu: bool,
	stage_count: i32,
	disable_volume_buttons: bool,
	disable_photo_ui: bool,
	disable_watermark: bool,
	status_icons: StatusIcons,
	disable_lyrics: bool,
	disable_error_banner: bool,
	disable_credits_text: bool,
	show_pdloader_text: bool,
	disable_credits: bool,
	disable_selection_timer: bool,
	disable_timer_sprite: bool,

	// components
	enable_input_emulator: bool,
	enable_touch_emulator: bool,
	enable_player_data_manager: bool,
	enable_frame_rate_manager: bool,
	enable_fast_loader: bool,
	fast_loader_speed: i32,
	enable_camera_controller: bool,
	enable_scaling: bool,
	enable_dwgui: bool,
	enable_hold_transfer: bool,
	enable_score_saving: bool,
	enable_pausing: bool,
}
impl Default for App {
	fn default() -> Self {
		App {
			current_tab: "video",

			display_format: WINDOWED,
			window_size_x: -1,
			window_size_y: -1,
			internal_resolution_enabled: true,
			internal_resolution_x: 1920,
			internal_resolution_y: 1080,

			taa_enabled: false,
			mlaa_enabled: true,
			glare_enabled: false,
			depth_of_field_enabled: true,
			frame_rate: 60,
			gamma: 100,
			reflections_enabled: true,
			shadows_enabled: true,
			transparency_enabled: true,
			disable_3d: false,

			disable_movies: false,
			show_cursor: false,
			disable_slider_emu: false,
			stage_count: 0,

			disable_volume_buttons: true,
			disable_photo_ui: true,
			disable_watermark: true,
			status_icons: DEFAULT,
			disable_lyrics: false,
			disable_error_banner: true,
			disable_credits_text: true,
			show_pdloader_text: true,
			disable_credits: true,
			disable_selection_timer: true,
			disable_timer_sprite: true,

			enable_input_emulator: true,
			enable_touch_emulator: true,
			enable_player_data_manager: true,
			enable_frame_rate_manager: true,
			enable_fast_loader: true,
			fast_loader_speed: 39,
			enable_camera_controller: true,
			enable_scaling: true,
			enable_dwgui: true,
			enable_hold_transfer: true,
			enable_score_saving: true,
			enable_pausing: true,
		}
	}
}
impl epi::App for App {
	fn name(&self) -> &str {
		"Brogamers Launcher"
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
		self.display_format = get_ini_value::<u32>(resolution_section, "Display");
		self.window_size_x = get_ini_value(resolution_section, "Width");
		self.window_size_y = get_ini_value(resolution_section, "Height");
		self.internal_resolution_enabled = get_ini_value(resolution_section, "r.Enable");
		self.internal_resolution_x = get_ini_value(resolution_section, "r.Width");
		self.internal_resolution_y = get_ini_value(resolution_section, "r.Height");

		let graphics_section = config_ini.section(Some("Graphics")).unwrap();
		self.taa_enabled = get_ini_value(graphics_section, "TAA");
		self.mlaa_enabled = get_ini_value(graphics_section, "MLAA");
		self.glare_enabled = get_ini_value(graphics_section, "Glare");
		self.depth_of_field_enabled = get_ini_value(graphics_section, "DOF");
		self.frame_rate = get_ini_value(graphics_section, "FPS.Limit");
		self.gamma = get_ini_value(graphics_section, "Gamma");
		self.reflections_enabled = get_ini_value(graphics_section, "Reflections");
		self.shadows_enabled = get_ini_value(graphics_section, "Shadows");
		self.transparency_enabled = get_ini_value(graphics_section, "Punchthrough");
		self.disable_3d = get_ini_value(graphics_section, "2D");

		let patches_section = config_ini.section(Some("Patches")).unwrap();
		self.disable_movies = get_ini_value(patches_section, "No_Movies");
		self.show_cursor = get_ini_value(patches_section, "Cursor");
		self.disable_slider_emu = get_ini_value(patches_section, "Hardware_Slider");
		self.stage_count = get_ini_value(patches_section, "Enhanced_Stage_Manager");
		self.disable_volume_buttons = get_ini_value(patches_section, "Hide_Volume");
		self.disable_photo_ui = get_ini_value(patches_section, "No_PV_UI");
		self.disable_watermark = get_ini_value(patches_section, "Hide_PV_Watermark");
		self.status_icons = get_ini_value(patches_section, "Status_Icons");
		self.disable_lyrics = get_ini_value(patches_section, "No_Lyrics");
		self.disable_error_banner = get_ini_value(patches_section, "No_Error");
		self.disable_credits_text = get_ini_value(patches_section, "Hide_Freeplay");
		self.show_pdloader_text = get_ini_value(patches_section, "PDLoaderText");
		self.disable_credits = get_ini_value(patches_section, "Freeplay");
		self.disable_selection_timer = get_ini_value(patches_section, "No_Timer");
		self.disable_timer_sprite = get_ini_value(patches_section, "No_Timer_Sprite");

		let components_ini = ini::Ini::load_from_file("plugins/components.ini").unwrap();
		let components_section = components_ini.section(Some("components")).unwrap();
		self.enable_input_emulator = get_ini_value(components_section, "input_emulator");
		self.enable_touch_emulator = get_ini_value(components_section, "touch_panel_emulator");
		self.enable_player_data_manager = get_ini_value(components_section, "player_data_manager");
		self.enable_frame_rate_manager = get_ini_value(components_section, "frame_rate_manager");
		self.enable_fast_loader = get_ini_value(components_section, "fast_loader");
		self.fast_loader_speed = get_ini_value(components_section, "fast_loader_speed");
		self.enable_camera_controller = get_ini_value(components_section, "camera_controller");
		self.enable_scaling = get_ini_value(components_section, "scale_component");
		self.enable_dwgui = get_ini_value(components_section, "debug_component");
		self.enable_hold_transfer = get_ini_value(components_section, "target_inspector");
		self.enable_score_saving = get_ini_value(components_section, "score_saver");
		self.enable_pausing = get_ini_value(components_section, "pause");
	}

	fn on_exit(&mut self) {
		let mut config_ini = if std::path::Path::new("plugins/config.ini").exists() {
			ini::Ini::load_from_file("plugins/config.ini").unwrap()
		} else {
			ini::Ini::new()
		};

		config_ini
			.with_section(Some("Resolution"))
			.set("Display", self.display_format.to_string())
			.set("Width", self.window_size_x.to_string())
			.set("Height", self.window_size_y.to_string())
			.set(
				"r.Enable",
				(self.internal_resolution_enabled as i32).to_string(),
			)
			.set("r.Width", self.internal_resolution_x.to_string())
			.set("r.Height", self.internal_resolution_y.to_string())
			.set("RefreshRate", self.frame_rate.to_string());

		config_ini
			.with_section(Some("Graphics"))
			.set("TAA", (self.taa_enabled as i32).to_string())
			.set("MLAA", (self.mlaa_enabled as i32).to_string())
			.set("Glare", (self.glare_enabled as i32).to_string())
			.set("DOF", (self.depth_of_field_enabled as i32).to_string())
			.set("FPS.Limit", self.frame_rate.to_string())
			.set("Gamma", self.gamma.to_string())
			.set("Reflections", (self.reflections_enabled as i32).to_string())
			.set("Shadows", (self.shadows_enabled as i32).to_string())
			.set(
				"Punchthrough",
				(self.transparency_enabled as i32).to_string(),
			)
			.set("2D", (self.disable_3d as i32).to_string());

		config_ini
			.with_section(Some("Patches"))
			.set("No_Movies", (self.disable_movies as i32).to_string())
			.set("Cursor", (self.show_cursor as i32).to_string())
			.set(
				"Hardware_Slider",
				(self.disable_slider_emu as i32).to_string(),
			)
			.set("Enhanced_Stage_Manager", self.stage_count.to_string())
			.set(
				"Hide_Volume",
				(self.disable_volume_buttons as i32).to_string(),
			)
			.set("No_PV_UI", (self.disable_photo_ui as i32).to_string())
			.set(
				"Hide_PV_Watermark",
				(self.disable_watermark as i32).to_string(),
			)
			.set("Status_Icons", self.status_icons.to_string())
			.set("No_Lyrics", (self.disable_lyrics as i32).to_string())
			.set("No_Error", (self.disable_error_banner as i32).to_string())
			.set(
				"Hide_Freeplay",
				(self.disable_credits_text as i32).to_string(),
			)
			.set("PDLoaderText", (self.show_pdloader_text as i32).to_string())
			.set("Freeplay", (self.disable_credits as i32).to_string())
			.set(
				"No_Timer",
				(self.disable_selection_timer as i32).to_string(),
			)
			.set(
				"No_Timer_Sprite",
				(self.disable_timer_sprite as i32).to_string(),
			);

		config_ini.write_to_file("plugins/config.ini").unwrap();

		let mut components_ini = if std::path::Path::new("plugins/components.ini").exists() {
			ini::Ini::load_from_file("plugins/components.ini").unwrap()
		} else {
			ini::Ini::new()
		};

		components_ini
			.with_section(Some("components"))
			.set(
				"touch_slider_emulator",
				(!self.disable_slider_emu).to_string(),
			)
			.set("sys_timer", self.disable_selection_timer.to_string())
			.set("input_emulator", self.enable_input_emulator.to_string())
			.set(
				"touch_panel_emulator",
				self.enable_touch_emulator.to_string(),
			)
			.set(
				"player_data_manager",
				self.enable_player_data_manager.to_string(),
			)
			.set(
				"frame_rate_manager",
				self.enable_frame_rate_manager.to_string(),
			)
			.set("fast_loader", self.enable_fast_loader.to_string())
			.set("fast_loader_speed", self.fast_loader_speed.to_string())
			.set(
				"camera_controller",
				self.enable_camera_controller.to_string(),
			)
			.set("scale_component", self.enable_scaling.to_string())
			.set("debug_component", self.enable_dwgui.to_string())
			.set("target_inspector", self.enable_score_saving.to_string())
			.set("pause", self.enable_pausing.to_string());

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
		ui.horizontal(|ui| {
			ui.label("Display type");
			egui::ComboBox::from_label("")
				.selected_text(format!(
					"{}",
					match self.display_format {
						WINDOWED => "Windowed",
						POPUP => "Popup",
						EXCLUSIVE => "Exclusive Fullscreen",
						BORDERLESS => "Borderless",
						_ => "Unknown",
					}
				))
				.width(ui.available_width() / 4.0)
				.show_ui(ui, |ui| {
					ui.selectable_value(&mut self.display_format, WINDOWED, "Windowed");
					ui.selectable_value(&mut self.display_format, POPUP, "Popup");
					ui.selectable_value(
						&mut self.display_format,
						EXCLUSIVE,
						"Exclusive Fullscreen",
					);
					ui.selectable_value(&mut self.display_format, BORDERLESS, "Borderless");
				});
		});

		ui.horizontal(|ui| {
			ui.add(egui::Label::new("Window Size"))
				.on_hover_text("-1 in both means to match the size of the screen");

			int_text_box(&mut self.window_size_x, 2.0, ui);
			int_text_box(&mut self.window_size_y, 1.0, ui);
		});

		ui.horizontal(|ui| {
			ui.add(egui::Label::new("Internal Resolution"))
				.on_hover_text("-1 in both means to match the size of the window");
			ui.checkbox(&mut self.internal_resolution_enabled, "");

			int_text_box(&mut self.internal_resolution_x, 2.0, ui);
			int_text_box(&mut self.internal_resolution_y, 1.0, ui);
		});

		ui.horizontal(|ui| {
			ui.add(egui::Label::new("Frame rate limit"))
				.on_hover_text("-1 to unlock the frame rate");

			int_text_box(&mut self.frame_rate, 2.0, ui);
		});

		simple_checkbox("Motion Blur", &mut self.taa_enabled, ui);
		simple_checkbox("MLAA", &mut self.mlaa_enabled, ui);
		simple_checkbox("Glare", &mut self.glare_enabled, ui);
		simple_checkbox("Depth of Field", &mut self.depth_of_field_enabled, ui);
		simple_checkbox("Reflections", &mut self.reflections_enabled, ui);
		simple_checkbox("Shadows", &mut self.shadows_enabled, ui);
		simple_checkbox("Transparency", &mut self.transparency_enabled, ui);
		simple_checkbox("Disable 3D", &mut self.disable_3d, ui);
		simple_checkbox("Window scaling", &mut self.enable_scaling, ui);

		ui.horizontal(|ui| {
			ui.label("Gamma");
			ui.add(egui::Slider::new(&mut self.gamma, 0..=200));
		});
	}

	fn draw_patches_tab(&mut self, ui: &mut egui::Ui) {
		simple_checkbox("Disable Movies", &mut self.disable_movies, ui);
		simple_checkbox("Show Cursor", &mut self.show_cursor, ui);
		simple_checkbox_tooltip(
			"Disable Slider Emulation",
			"Must be checked if using a hardware slider",
			&mut self.disable_slider_emu,
			ui,
		);

		// should these really be separate?
		// would anyone want touch emulation without input emulation or vice versa?
		simple_checkbox("Input Emulation", &mut self.enable_input_emulator, ui);
		simple_checkbox("Touch Emulation", &mut self.enable_touch_emulator, ui);

		simple_checkbox("Disable credits check", &mut self.disable_credits, ui);
		simple_checkbox(
			"Disable selection timer",
			&mut self.disable_selection_timer,
			ui,
		);
		ui.horizontal(|ui| {
			ui.add(egui::Label::new("Number of stages"))
				.on_hover_text("Set to 0 if using online");
			int_text_box(&mut self.stage_count, 5.0, ui);
		});

		ui.horizontal(|ui| {
			ui.add(egui::Label::new("Fast Loader"))
				.on_hover_text("Set to 3 or lower when using online");
			ui.checkbox(&mut self.enable_fast_loader, "");
			ui.add(egui::Slider::new(&mut self.fast_loader_speed, 0..=100));
		});

		simple_checkbox(
			"Custom player data",
			&mut self.enable_player_data_manager,
			ui,
		);
		simple_checkbox_tooltip(
			"Frame Rate Manager",
			"Speeds up animations to match the frame rate\nStops certain sounds from playing",
			&mut self.enable_frame_rate_manager,
			ui,
		);
		simple_checkbox_tooltip(
			"Camera Controller",
			"Press F3 to activate",
			&mut self.enable_camera_controller,
			ui,
		);
		simple_checkbox("Enable debug menus", &mut self.enable_dwgui, ui);
		simple_checkbox("Hold transfers", &mut self.enable_hold_transfer, ui);
		simple_checkbox("Score saving", &mut self.enable_score_saving, ui);
		simple_checkbox_tooltip(
			"Pause menu",
			"Entered by pressing JVS_START\nStart on controller or Enter on keyboard by default",
			&mut self.enable_pausing,
			ui,
		);
	}

	fn draw_ui_tab(&mut self, ui: &mut egui::Ui) {
		ui.horizontal(|ui| {
			ui.label("Status icons");
			egui::ComboBox::from_label("")
				.selected_text(format!(
					"{}",
					match self.status_icons {
						DEFAULT => "Default",
						ERROR => "Error",
						OK => "Ok",
						PARTIALOK => "Partial ok",
						HIDDEN => "Hidden",
						_ => "Unknown",
					}
				))
				.width(ui.available_width() / 4.0)
				.show_ui(ui, |ui| {
					ui.selectable_value(&mut self.status_icons, DEFAULT, "Default");
					ui.selectable_value(&mut self.status_icons, ERROR, "Error");
					ui.selectable_value(&mut self.status_icons, OK, "Ok");
					ui.selectable_value(&mut self.status_icons, PARTIALOK, "Partial Ok");
					ui.selectable_value(&mut self.status_icons, HIDDEN, "Hidden");
				});
		});

		simple_checkbox("Hide Volume Buttons", &mut self.disable_volume_buttons, ui);
		simple_checkbox("Hide PV Photo UI", &mut self.disable_photo_ui, ui);
		simple_checkbox("Hide PV Watermark", &mut self.disable_watermark, ui);
		simple_checkbox("Hide Lyrics", &mut self.disable_lyrics, ui);
		simple_checkbox("Hide Error banner", &mut self.disable_error_banner, ui);
		simple_checkbox("Hide CREDITS text", &mut self.disable_credits_text, ui);
		simple_checkbox(
			"Swap CREDITS text with PD LOADER",
			&mut self.show_pdloader_text,
			ui,
		);
		simple_checkbox("Hide Selection timer", &mut self.disable_timer_sprite, ui);
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
