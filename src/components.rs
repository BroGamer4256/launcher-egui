use crate::{get_ini_value, patch::AppPatches, IniConfig, IniConfigWriteCtx};

pub struct AppComponents {
	pub(crate) enable_input_emulator: bool,
	pub(crate) enable_touch_emulator: bool,
	pub(crate) enable_player_data_manager: bool,
	pub(crate) enable_frame_rate_manager: bool,
	pub(crate) enable_fast_loader: bool,
	pub(crate) fast_loader_speed: i32,
	pub(crate) enable_camera_controller: bool,
	pub(crate) enable_scaling: bool,
	pub(crate) enable_dwgui: bool,
	pub(crate) enable_hold_transfer: bool,
	pub(crate) enable_score_saving: bool,
	pub(crate) enable_pausing: bool,
}

impl IniConfig for AppComponents {
	const SECTION: &'static str = "components";
	type Error = std::io::Error;

	fn read_body(section: &ini::Properties) -> Result<Self, Self::Error> {
		let enable_input_emulator = get_ini_value(section, "input_emulator");
		let enable_touch_emulator = get_ini_value(section, "touch_panel_emulator");
		let enable_player_data_manager = get_ini_value(section, "player_data_manager");
		let enable_frame_rate_manager = get_ini_value(section, "frame_rate_manager");
		let enable_fast_loader = get_ini_value(section, "fast_loader");
		let fast_loader_speed = get_ini_value(section, "fast_loader_speed");
		let enable_camera_controller = get_ini_value(section, "camera_controller");
		let enable_scaling = get_ini_value(section, "scale_component");
		let enable_dwgui = get_ini_value(section, "debug_component");
		let enable_hold_transfer = get_ini_value(section, "target_inspector");
		let enable_score_saving = get_ini_value(section, "score_saver");
		let enable_pausing = get_ini_value(section, "pause");
		Ok(Self {
			enable_input_emulator,
			enable_touch_emulator,
			enable_player_data_manager,
			enable_frame_rate_manager,
			enable_fast_loader,
			fast_loader_speed,
			enable_camera_controller,
			enable_scaling,
			enable_dwgui,
			enable_hold_transfer,
			enable_score_saving,
			enable_pausing,
		})
	}
}

impl IniConfigWriteCtx for AppComponents {
	type Context = AppPatches;

	fn write_body<'a, 'b>(&'a self, section: &'b mut ini::SectionSetter<'b>) {
		section
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
	}

	fn write_additional<'a, 'b>(add: &'a Self::Context, section: &'b mut ini::SectionSetter<'b>) {
		section
			.set(
				"touch_slider_emulator",
				(!add.disable_slider_emu).to_string(),
			)
			.set("sys_timer", add.disable_selection_timer.to_string());
	}
}

impl Default for AppComponents {
	fn default() -> Self {
		Self {
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
