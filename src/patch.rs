use crate::utils::*;
use crate::{get_ini_value, StatusIcons};

pub struct AppPatches {
	pub(crate) disable_movies: bool,
	pub(crate) show_cursor: bool,
	pub(crate) disable_slider_emu: bool,
	pub(crate) stage_count: i32,
	pub(crate) disable_volume_buttons: bool,
	pub(crate) disable_photo_ui: bool,
	pub(crate) disable_watermark: bool,
	pub(crate) status_icons: StatusIcons,
	pub(crate) disable_lyrics: bool,
	pub(crate) disable_error_banner: bool,
	pub(crate) disable_credits_text: bool,
	pub(crate) show_pdloader_text: bool,
	pub(crate) disable_credits: bool,
	pub(crate) disable_selection_timer: bool,
	pub(crate) disable_timer_sprite: bool,
}

impl IniConfig for AppPatches {
	const SECTION: &'static str = "Patches";

	type Error = std::io::Error;

	fn read_body(section: &ini::Properties) -> Result<Self, Self::Error> {
		let disable_movies = get_ini_value(section, "No_Movies");
		let show_cursor = get_ini_value(section, "Cursor");
		let disable_slider_emu = get_ini_value(section, "Hardware_Slider");
		let stage_count = get_ini_value(section, "Enhanced_Stage_Manager");
		let disable_volume_buttons = get_ini_value(section, "Hide_Volume");
		let disable_photo_ui = get_ini_value(section, "No_PV_UI");
		let disable_watermark = get_ini_value(section, "Hide_PV_Watermark");
		//TODO: handle the error case
		let status_icons = StatusIcons::from_u8(get_ini_value(section, "Status_Icons"))
			.expect("valid status icons value");
		let disable_lyrics = get_ini_value(section, "No_Lyrics");
		let disable_error_banner = get_ini_value(section, "No_Error");
		let disable_credits_text = get_ini_value(section, "Hide_Freeplay");
		let show_pdloader_text = get_ini_value(section, "PDLoaderText");
		let disable_credits = get_ini_value(section, "Freeplay");
		let disable_selection_timer = get_ini_value(section, "No_Timer");
		let disable_timer_sprite = get_ini_value(section, "No_Timer_Sprite");
		Ok(Self {
			disable_movies,
			show_cursor,
			disable_slider_emu,
			stage_count,
			disable_volume_buttons,
			disable_photo_ui,
			disable_watermark,
			status_icons,
			disable_lyrics,
			disable_error_banner,
			disable_credits_text,
			show_pdloader_text,
			disable_credits,
			disable_selection_timer,
			disable_timer_sprite,
		})
	}
}

impl IniConfigWrite for AppPatches {
	fn write_body<'a, 'b>(&'a self, section: &'b mut ini::SectionSetter<'b>) {
		section
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
			.set("Status_Icons", (self.status_icons as i32).to_string())
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
	}
}

impl Default for AppPatches {
	fn default() -> Self {
		Self {
			disable_movies: false,
			show_cursor: false,
			disable_slider_emu: false,
			stage_count: 0,
			disable_volume_buttons: true,
			disable_photo_ui: true,
			disable_watermark: true,
			status_icons: StatusIcons::default(),
			disable_lyrics: false,
			disable_error_banner: true,
			disable_credits_text: true,
			show_pdloader_text: true,
			disable_credits: true,
			disable_selection_timer: true,
			disable_timer_sprite: true,
		}
	}
}
