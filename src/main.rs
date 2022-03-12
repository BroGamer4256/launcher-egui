#![feature(derive_default_enum)]
use eframe::{egui, epi};
use std::{
	hash::{Hash, Hasher},
	str::FromStr,
};
use strum::IntoEnumIterator;
use strum_macros::{EnumIter, EnumString, IntoStaticStr};

static mut INT_TEXT_BOX_COUNT: i32 = 0;

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

#[allow(non_camel_case_types)]
#[derive(Debug, PartialEq, Eq, Clone, Copy, EnumString, EnumIter, Hash, IntoStaticStr)]
pub enum Buttons {
	F1,
	F2,
	F3,
	F4,
	F5,
	F6,
	F7,
	F8,
	F9,
	F10,
	F11,
	F12,
	NUM0,
	NUM1,
	NUM2,
	NUM3,
	NUM4,
	NUM5,
	NUM6,
	NUM7,
	NUM8,
	NUM9,
	Q,
	W,
	E,
	R,
	T,
	Y,
	U,
	I,
	O,
	P,
	A,
	S,
	D,
	F,
	G,
	H,
	J,
	K,
	L,
	Z,
	X,
	C,
	V,
	B,
	N,
	M,
	UPARROW,
	LEFTARROW,
	DOWNARROW,
	RIGHTARROW,
	ENTER,
	SPACE,
	CONTROL,
	SHIFT,
	TAB,
	SDL_A,
	SDL_B,
	SDL_X,
	SDL_Y,
	SDL_BACK,
	SDL_GUIDE,
	SDL_START,
	SDL_LSHOULDER,
	SDL_LTRIGGER,
	SDL_RSHOULDER,
	SDL_RTRIGGER,
	SDL_DPAD_UP,
	SDL_DPAD_LEFT,
	SDL_DPAD_DOWN,
	SDL_DPAD_RIGHT,
	SDL_MISC,
	SDL_PADDLE1,
	SDL_PADDLE2,
	SDL_PADDLE3,
	SDL_PADDLE4,
	SDL_TOUCHPAD,
	SDL_LSTICK_UP,
	SDL_LSTICK_LEFT,
	SDL_LSTICK_RIGHT,
	SDL_LSTICK_DOWN,
	SDL_LSTICK_PRESS,
	SDL_RSTICK_UP,
	SDL_RSTICK_LEFT,
	SDL_RSTICK_RIGHT,
	SDL_RSTICK_DOWN,
	SDL_RSTICK_PRESS,
}

#[derive(Default, Debug, PartialEq, Clone)]
pub struct AllConfig {
	config: Config,
	keyconfig: Keyconfig,
	patches: Vec<Patch>,
	translations: Vec<Translation>,
}

#[derive(Default, Debug, PartialEq, Clone)]
pub struct Config {
	fps: i32,
	internal_res_x: i32,
	internal_res_y: i32,
	fullscreen: bool,
	rumble_intensity: i32,
}

#[derive(Default, Debug, PartialEq, Clone)]
pub struct Keyconfig {
	test: Vec<Buttons>,
	service: Vec<Buttons>,
	advertise: Vec<Buttons>,
	game: Vec<Buttons>,
	data_test: Vec<Buttons>,
	test_mode: Vec<Buttons>,
	app_error: Vec<Buttons>,

	start: Vec<Buttons>,
	triangle: Vec<Buttons>,
	square: Vec<Buttons>,
	cross: Vec<Buttons>,
	circle: Vec<Buttons>,
	left_left: Vec<Buttons>,
	left_right: Vec<Buttons>,
	right_left: Vec<Buttons>,
	right_right: Vec<Buttons>,

	camera_unlock_toggle: Vec<Buttons>,
	camera_move_forward: Vec<Buttons>,
	camera_move_backward: Vec<Buttons>,
	camera_move_left: Vec<Buttons>,
	camera_move_right: Vec<Buttons>,
	camera_move_up: Vec<Buttons>,
	camera_move_down: Vec<Buttons>,
	camera_rotate_cw: Vec<Buttons>,
	camera_rotate_ccw: Vec<Buttons>,
	camera_zoom_in: Vec<Buttons>,
	camera_zoom_out: Vec<Buttons>,
	camera_move_fast: Vec<Buttons>,
	camera_move_slow: Vec<Buttons>,
}

#[derive(Default, Debug, PartialEq, Clone)]
pub struct Patch {
	name: String,
	author: String,
	enabled: bool,
	patches: Vec<InternalPatch>,
}

#[allow(non_camel_case_types)]
#[derive(Debug, PartialEq, Eq, Clone, Copy, EnumString, EnumIter, IntoStaticStr)]
pub enum DataTypes {
	i8,
	i8_arr,
	u8,
	u8_arr,
	i16,
	i16_arr,
	u16,
	u16_arr,
	i32,
	i32_arr,
	u32,
	u32_arr,
	i64,
	i64_arr,
	string,
}

impl Default for DataTypes {
	fn default() -> Self {
		Self::u8_arr
	}
}

#[derive(Default, Debug, PartialEq, Clone)]
pub struct InternalPatch {
	address: i64,
	data_type: DataTypes,
	data_int: i64,
	data_int_arr: Vec<i64>,
	data_string: String,
}

#[derive(Default, Debug, PartialEq, Clone)]
pub struct Translation {
	language: String,
	author: String,
	enabled: bool,
	translations: Vec<InternalTranslation>,
}

#[allow(non_camel_case_types)]
#[derive(Default, Debug, PartialEq, Eq, Clone, Copy, EnumString, EnumIter, IntoStaticStr)]
pub enum SubGameStates {
	SUB_DATA_INITIALIZE,
	SUB_SYSTEM_STARTUP,
	SUB_SYSTEM_STARTUP_ERROR,
	SUB_WARNING,
	SUB_LOGO,
	SUB_RATING,
	SUB_DEMO,
	SUB_TITLE,
	SUB_RANKING,
	SUB_SCORE_RANKING,
	SUB_CM,
	SUB_PHOTO_MODE_DEMO,
	SUB_SELECTOR,
	SUB_GAME_MAIN,
	SUB_GAME_SEL,
	SUB_STAGE_RESULT,
	SUB_SCREEN_SHOT_SEL,
	SUB_SCREEN_SHOT_RESULT,
	SUB_GAME_OVER,
	SUB_DATA_TEST_MAIN,
	SUB_DATA_TEST_MISC,
	SUB_DATA_TEST_OBJ,
	SUB_DATA_TEST_STG,
	SUB_DATA_TEST_MOT,
	SUB_DATA_TEST_COLLISION,
	SUB_DATA_TEST_SPR,
	SUB_DATA_TEST_AET,
	SUB_DATA_TEST_AUTH_3D,
	SUB_DATA_TEST_CHR,
	SUB_DATA_TEST_ITEM,
	SUB_DATA_TEST_PERF,
	SUB_DATA_TEST_PVSCRIPT,
	SUB_DATA_TEST_PRINT,
	SUB_DATA_TEST_CARD,
	SUB_DATA_TEST_OPD,
	SUB_DATA_TEST_SLIDER,
	SUB_DATA_TEST_GLITTER,
	SUB_DATA_TEST_GRAPHICS,
	SUB_DATA_TEST_COLLECTION_CARD,
	SUB_TEST_MODE_MAIN,
	SUB_APP_ERROR,
	#[default]
	SUB_MAX,
}

#[derive(Default, Debug, PartialEq, Clone)]
pub struct InternalTranslation {
	old: String,
	new: String,
	state: SubGameStates,
}

#[derive(Default)]
pub struct DocFileIndex {
	doc: toml_edit::Document,
	file: String,
	index: usize,
}

#[derive(Default)]
pub struct App {
	current_tab: &'static str,

	config: AllConfig,
	config_doc: toml_edit::Document,
	keyconfig_doc: toml_edit::Document,
	patches_docs: Vec<DocFileIndex>,
	have_translation: bool,
	translation_docs: Vec<DocFileIndex>,
}

impl epi::App for App {
	fn name(&self) -> &str {
		"launcher-egui"
	}

	fn setup(
		&mut self,
		ctx: &egui::Context,
		frame: &epi::Frame,
		_storage: Option<&dyn epi::Storage>,
	) {
		ctx.set_visuals(egui::Visuals::dark());

		let mut fonts = egui::FontDefinitions::default();
		fonts.font_data.insert(
			"JP_Font".to_owned(),
			egui::FontData::from_static(include_bytes!("../SourceHanSansJP-Medium.otf")),
		);
		fonts
			.families
			.get_mut(&egui::FontFamily::Proportional)
			.unwrap()
			.insert(0, "JP_Font".to_owned());
		fonts
			.families
			.get_mut(&egui::FontFamily::Monospace)
			.unwrap()
			.push("JP_Font".to_owned());
		ctx.set_fonts(fonts);

		let mut style = egui::Style::default();
		style.override_font_id = Some(egui::FontId::proportional(18.0));
		ctx.set_style(style);

		self.current_tab = "config";
		self.have_translation = std::path::Path::new("lang.dll").exists();

		if !std::path::Path::new("config.toml").exists() {
			frame.quit();
			return;
		}
		let config_str = std::fs::read_to_string("config.toml").unwrap();
		let keyconfig_str = std::fs::read_to_string("keyconfig.toml").unwrap();
		self.config_doc = config_str.parse::<toml_edit::Document>().unwrap();
		self.keyconfig_doc = keyconfig_str.parse::<toml_edit::Document>().unwrap();
		for file in std::fs::read_dir(std::path::Path::new("patches")).unwrap() {
			let file = file.unwrap();
			let patch_str = std::fs::read_to_string(file.path()).unwrap();
			self.patches_docs.push(DocFileIndex {
				doc: patch_str.parse::<toml_edit::Document>().unwrap(),
				file: file.path().to_string_lossy().to_string(),
				index: 0,
			});
		}

		self.config.config.fps = self.config_doc["fps"].as_integer().unwrap() as i32;
		self.config.config.internal_res_x =
			self.config_doc["internalRes"]["x"].as_integer().unwrap() as i32;
		self.config.config.internal_res_y =
			self.config_doc["internalRes"]["y"].as_integer().unwrap() as i32;
		self.config.config.fullscreen = self.config_doc["fullscreen"].as_bool().unwrap();
		self.config.config.rumble_intensity =
			self.config_doc["rumbleIntensity"].as_integer().unwrap() as i32;

		read_toml_array_to_vec(&mut self.config.keyconfig.test, &self.keyconfig_doc["TEST"]);
		read_toml_array_to_vec(
			&mut self.config.keyconfig.service,
			&self.keyconfig_doc["SERVICE"],
		);
		read_toml_array_to_vec(
			&mut self.config.keyconfig.advertise,
			&self.keyconfig_doc["ADVERTISE"],
		);
		read_toml_array_to_vec(&mut self.config.keyconfig.game, &self.keyconfig_doc["GAME"]);
		read_toml_array_to_vec(
			&mut self.config.keyconfig.data_test,
			&self.keyconfig_doc["DATA_TEST"],
		);
		read_toml_array_to_vec(
			&mut self.config.keyconfig.test_mode,
			&self.keyconfig_doc["TEST_MODE"],
		);
		read_toml_array_to_vec(
			&mut self.config.keyconfig.app_error,
			&self.keyconfig_doc["APP_ERROR"],
		);

		read_toml_array_to_vec(
			&mut self.config.keyconfig.start,
			&self.keyconfig_doc["START"],
		);
		read_toml_array_to_vec(
			&mut self.config.keyconfig.triangle,
			&self.keyconfig_doc["TRIANGLE"],
		);
		read_toml_array_to_vec(
			&mut self.config.keyconfig.square,
			&self.keyconfig_doc["SQUARE"],
		);
		read_toml_array_to_vec(
			&mut self.config.keyconfig.cross,
			&self.keyconfig_doc["CROSS"],
		);
		read_toml_array_to_vec(
			&mut self.config.keyconfig.circle,
			&self.keyconfig_doc["CIRCLE"],
		);
		read_toml_array_to_vec(
			&mut self.config.keyconfig.left_left,
			&self.keyconfig_doc["LEFT_LEFT"],
		);
		read_toml_array_to_vec(
			&mut self.config.keyconfig.left_right,
			&self.keyconfig_doc["LEFT_RIGHT"],
		);
		read_toml_array_to_vec(
			&mut self.config.keyconfig.right_left,
			&self.keyconfig_doc["RIGHT_LEFT"],
		);
		read_toml_array_to_vec(
			&mut self.config.keyconfig.right_right,
			&self.keyconfig_doc["RIGHT_RIGHT"],
		);

		read_toml_array_to_vec(
			&mut self.config.keyconfig.camera_unlock_toggle,
			&self.keyconfig_doc["CAMERA_UNLOCK_TOGGLE"],
		);
		read_toml_array_to_vec(
			&mut self.config.keyconfig.camera_move_forward,
			&self.keyconfig_doc["CAMERA_MOVE_FORWARD"],
		);
		read_toml_array_to_vec(
			&mut self.config.keyconfig.camera_move_backward,
			&self.keyconfig_doc["CAMERA_MOVE_BACKWARD"],
		);
		read_toml_array_to_vec(
			&mut self.config.keyconfig.camera_move_left,
			&self.keyconfig_doc["CAMERA_MOVE_LEFT"],
		);
		read_toml_array_to_vec(
			&mut self.config.keyconfig.camera_move_right,
			&self.keyconfig_doc["CAMERA_MOVE_RIGHT"],
		);
		read_toml_array_to_vec(
			&mut self.config.keyconfig.camera_move_up,
			&self.keyconfig_doc["CAMERA_MOVE_UP"],
		);
		read_toml_array_to_vec(
			&mut self.config.keyconfig.camera_move_down,
			&self.keyconfig_doc["CAMERA_MOVE_DOWN"],
		);
		read_toml_array_to_vec(
			&mut self.config.keyconfig.camera_rotate_cw,
			&self.keyconfig_doc["CAMERA_ROTATE_CW"],
		);
		read_toml_array_to_vec(
			&mut self.config.keyconfig.camera_rotate_ccw,
			&self.keyconfig_doc["CAMERA_ROTATE_CCW"],
		);
		read_toml_array_to_vec(
			&mut self.config.keyconfig.camera_zoom_in,
			&self.keyconfig_doc["CAMERA_ZOOM_IN"],
		);
		read_toml_array_to_vec(
			&mut self.config.keyconfig.camera_zoom_out,
			&self.keyconfig_doc["CAMERA_ZOOM_OUT"],
		);
		read_toml_array_to_vec(
			&mut self.config.keyconfig.camera_move_fast,
			&self.keyconfig_doc["CAMERA_MOVE_FAST"],
		);
		read_toml_array_to_vec(
			&mut self.config.keyconfig.camera_move_slow,
			&self.keyconfig_doc["CAMERA_MOVE_SLOW"],
		);

		for patch_doc in &mut self.patches_docs {
			let patch_data = Patch {
				name: patch_doc.doc["name"].as_str().unwrap().to_string(),
				author: patch_doc.doc["author"].as_str().unwrap().to_string(),
				enabled: patch_doc.doc["enabled"].as_bool().unwrap(),
				patches: vec![],
			};
			self.config.patches.push(patch_data.clone());
			patch_doc.index = self
				.config
				.patches
				.iter()
				.rposition(|x| x == &patch_data)
				.unwrap();

			for table in patch_doc.doc["patch"].as_array_of_tables().unwrap() {
				let data_type = DataTypes::from_str(table["data_type"].as_str().unwrap()).unwrap();
				let mut patch: InternalPatch = InternalPatch::default();
				patch.address = table["address"].as_integer().unwrap();
				patch.data_type = data_type;
				match data_type {
					DataTypes::i8
					| DataTypes::u8
					| DataTypes::i16
					| DataTypes::u16
					| DataTypes::i32
					| DataTypes::u32
					| DataTypes::i64 => patch.data_int = table["data"].as_integer().unwrap(),
					DataTypes::i8_arr
					| DataTypes::u8_arr
					| DataTypes::i16_arr
					| DataTypes::u16_arr
					| DataTypes::i32_arr
					| DataTypes::u32_arr
					| DataTypes::i64_arr => {
						read_toml_integer_array_to_vec(&mut patch.data_int_arr, &table["data"])
					}
					DataTypes::string => {
						patch.data_string = table["data"].as_str().unwrap().to_string()
					}
				};
				self.config.patches.last_mut().unwrap().patches.push(patch);
			}
		}

		if !self.have_translation {
			return;
		}

		for file in std::fs::read_dir(std::path::Path::new("translations")).unwrap() {
			let file = file.unwrap();
			let translations_str = std::fs::read_to_string(file.path()).unwrap();
			self.translation_docs.push(DocFileIndex {
				doc: translations_str.parse::<toml_edit::Document>().unwrap(),
				file: file.path().to_string_lossy().to_string(),
				index: 0,
			});
		}

		for translation_doc in &mut self.translation_docs {
			let translation_data = Translation {
				language: translation_doc.doc["language"]
					.as_str()
					.unwrap()
					.to_string(),
				author: translation_doc.doc["author"].as_str().unwrap().to_string(),
				enabled: translation_doc.doc["enabled"].as_bool().unwrap(),
				translations: vec![],
			};
			self.config.translations.push(translation_data.clone());
			translation_doc.index = self
				.config
				.translations
				.iter()
				.rposition(|x| x == &translation_data)
				.unwrap();

			for table in translation_doc.doc["translation"]
				.as_array_of_tables()
				.unwrap()
			{
				let state = if table.contains_key("state") {
					SubGameStates::from_str(table["state"].as_str().unwrap()).unwrap()
				} else {
					SubGameStates::SUB_MAX
				};
				self.config
					.translations
					.last_mut()
					.unwrap()
					.translations
					.push(InternalTranslation {
						old: table["old"].as_str().unwrap().to_string(),
						new: table["new"].as_str().unwrap().to_string(),
						state: state,
					});
			}
		}
	}

	fn on_exit(&mut self) {
		if !std::path::Path::new("config.toml").exists() {
			return;
		}

		self.config_doc["fps"] = toml_edit::value(self.config.config.fps as i64);
		self.config_doc["internalRes"]["x"] =
			toml_edit::value(self.config.config.internal_res_x as i64);
		self.config_doc["internalRes"]["y"] =
			toml_edit::value(self.config.config.internal_res_y as i64);
		self.config_doc["fullscreen"] = toml_edit::value(self.config.config.fullscreen);
		self.config_doc["rumbleIntensity"] =
			toml_edit::value(self.config.config.rumble_intensity as i64);

		read_vec_to_toml_array(
			&buttons_vec_to_string(&self.config.keyconfig.test),
			&mut self.keyconfig_doc["TEST"],
		);
		read_vec_to_toml_array(
			&buttons_vec_to_string(&self.config.keyconfig.service),
			&mut self.keyconfig_doc["SERVICE"],
		);
		read_vec_to_toml_array(
			&buttons_vec_to_string(&self.config.keyconfig.advertise),
			&mut self.keyconfig_doc["ADVERTISE"],
		);
		read_vec_to_toml_array(
			&buttons_vec_to_string(&self.config.keyconfig.game),
			&mut self.keyconfig_doc["GAME"],
		);
		read_vec_to_toml_array(
			&buttons_vec_to_string(&self.config.keyconfig.data_test),
			&mut self.keyconfig_doc["DATA_TEST"],
		);
		read_vec_to_toml_array(
			&buttons_vec_to_string(&self.config.keyconfig.test_mode),
			&mut self.keyconfig_doc["TEST_MODE"],
		);
		read_vec_to_toml_array(
			&buttons_vec_to_string(&self.config.keyconfig.app_error),
			&mut self.keyconfig_doc["APP_ERROR"],
		);

		read_vec_to_toml_array(
			&buttons_vec_to_string(&self.config.keyconfig.start),
			&mut self.keyconfig_doc["START"],
		);
		read_vec_to_toml_array(
			&buttons_vec_to_string(&self.config.keyconfig.triangle),
			&mut self.keyconfig_doc["TRIANGLE"],
		);
		read_vec_to_toml_array(
			&buttons_vec_to_string(&self.config.keyconfig.square),
			&mut self.keyconfig_doc["SQUARE"],
		);
		read_vec_to_toml_array(
			&buttons_vec_to_string(&self.config.keyconfig.cross),
			&mut self.keyconfig_doc["CROSS"],
		);
		read_vec_to_toml_array(
			&buttons_vec_to_string(&self.config.keyconfig.circle),
			&mut self.keyconfig_doc["CIRCLE"],
		);
		read_vec_to_toml_array(
			&buttons_vec_to_string(&self.config.keyconfig.left_left),
			&mut self.keyconfig_doc["LEFT_LEFT"],
		);
		read_vec_to_toml_array(
			&buttons_vec_to_string(&self.config.keyconfig.left_right),
			&mut self.keyconfig_doc["LEFT_RIGHT"],
		);
		read_vec_to_toml_array(
			&buttons_vec_to_string(&self.config.keyconfig.right_left),
			&mut self.keyconfig_doc["RIGHT_LEFT"],
		);
		read_vec_to_toml_array(
			&buttons_vec_to_string(&self.config.keyconfig.right_right),
			&mut self.keyconfig_doc["RIGHT_RIGHT"],
		);

		read_vec_to_toml_array(
			&buttons_vec_to_string(&self.config.keyconfig.camera_unlock_toggle),
			&mut self.keyconfig_doc["CAMERA_UNLOCK_TOGGLE"],
		);
		read_vec_to_toml_array(
			&buttons_vec_to_string(&self.config.keyconfig.camera_move_forward),
			&mut self.keyconfig_doc["CAMERA_MOVE_FORWARD"],
		);
		read_vec_to_toml_array(
			&buttons_vec_to_string(&self.config.keyconfig.camera_move_backward),
			&mut self.keyconfig_doc["CAMERA_MOVE_BACKWARD"],
		);
		read_vec_to_toml_array(
			&buttons_vec_to_string(&self.config.keyconfig.camera_move_left),
			&mut self.keyconfig_doc["CAMERA_MOVE_LEFT"],
		);
		read_vec_to_toml_array(
			&buttons_vec_to_string(&self.config.keyconfig.camera_move_right),
			&mut self.keyconfig_doc["CAMERA_MOVE_RIGHT"],
		);
		read_vec_to_toml_array(
			&buttons_vec_to_string(&self.config.keyconfig.camera_move_up),
			&mut self.keyconfig_doc["CAMERA_MOVE_UP"],
		);
		read_vec_to_toml_array(
			&buttons_vec_to_string(&self.config.keyconfig.camera_move_down),
			&mut self.keyconfig_doc["CAMERA_MOVE_DOWN"],
		);
		read_vec_to_toml_array(
			&buttons_vec_to_string(&self.config.keyconfig.camera_rotate_cw),
			&mut self.keyconfig_doc["CAMERA_ROTATE_CW"],
		);
		read_vec_to_toml_array(
			&buttons_vec_to_string(&self.config.keyconfig.camera_rotate_ccw),
			&mut self.keyconfig_doc["CAMERA_ROTATE_CCW"],
		);
		read_vec_to_toml_array(
			&buttons_vec_to_string(&self.config.keyconfig.camera_zoom_in),
			&mut self.keyconfig_doc["CAMERA_ZOOM_IN"],
		);
		read_vec_to_toml_array(
			&buttons_vec_to_string(&self.config.keyconfig.camera_zoom_out),
			&mut self.keyconfig_doc["CAMERA_ZOOM_OUT"],
		);
		read_vec_to_toml_array(
			&buttons_vec_to_string(&self.config.keyconfig.camera_move_fast),
			&mut self.keyconfig_doc["CAMERA_MOVE_FAST"],
		);
		read_vec_to_toml_array(
			&buttons_vec_to_string(&self.config.keyconfig.camera_move_slow),
			&mut self.keyconfig_doc["CAMERA_MOVE_SLOW"],
		);

		let config_str = self.config_doc.to_string();
		let keyconfig_str = self.keyconfig_doc.to_string();
		std::fs::write("config.toml", config_str).unwrap();
		std::fs::write("keyconfig.toml", keyconfig_str).unwrap();

		for patch_doc in &mut self.patches_docs {
			let patch_data = &self.config.patches[patch_doc.index];
			patch_doc.doc["name"] = toml_edit::value(&patch_data.name);
			patch_doc.doc["author"] = toml_edit::value(&patch_data.author);
			patch_doc.doc["enabled"] = toml_edit::value(patch_data.enabled);

			let diff = patch_data.patches.len()
				- patch_doc.doc["patch"].as_array_of_tables().unwrap().len();
			for _ in 0..diff {
				patch_doc.doc["patch"]
					.as_array_of_tables_mut()
					.unwrap()
					.push(toml_edit::Table::default());
			}

			for (i, table) in patch_doc.doc["patch"]
				.as_array_of_tables_mut()
				.unwrap()
				.iter_mut()
				.enumerate()
			{
				table["address"] = toml_edit::value(patch_data.patches[i].address);
				let data_type_str: &'static str = patch_data.patches[i].data_type.into();
				table["data_type"] = toml_edit::value(data_type_str);
				match patch_data.patches[i].data_type {
					DataTypes::i8
					| DataTypes::u8
					| DataTypes::i16
					| DataTypes::u16
					| DataTypes::i32
					| DataTypes::u32
					| DataTypes::i64 => table["data"] = toml_edit::value(patch_data.patches[i].data_int),
					DataTypes::i8_arr
					| DataTypes::u8_arr
					| DataTypes::i16_arr
					| DataTypes::u16_arr
					| DataTypes::i32_arr
					| DataTypes::u32_arr
					| DataTypes::i64_arr => {
						read_vec_to_toml_array(
							&patch_data.patches[i].data_int_arr,
							&mut table["data"],
						);
					}
					DataTypes::string => {
						table["data"] = toml_edit::value(&patch_data.patches[i].data_string)
					}
				};
			}

			let patch_str = patch_doc.doc.to_string();
			std::fs::write(&patch_doc.file, patch_str).unwrap();
		}

		if !self.have_translation {
			return;
		}
		for translation_doc in &mut self.translation_docs {
			let translations_data = &self.config.translations[translation_doc.index];
			translation_doc.doc["language"] = toml_edit::value(&translations_data.language);
			translation_doc.doc["author"] = toml_edit::value(&translations_data.author);
			translation_doc.doc["enabled"] = toml_edit::value(translations_data.enabled);

			let diff = translations_data.translations.len()
				- translation_doc.doc["translation"]
					.as_array_of_tables()
					.unwrap()
					.len();
			for _ in 0..diff {
				translation_doc.doc["translation"]
					.as_array_of_tables_mut()
					.unwrap()
					.push(toml_edit::Table::default());
			}

			for (i, table) in translation_doc.doc["translation"]
				.as_array_of_tables_mut()
				.unwrap()
				.iter_mut()
				.enumerate()
			{
				table["old"] = toml_edit::value(&translations_data.translations[i].old);
				table["new"] = toml_edit::value(&translations_data.translations[i].new);
				if table.contains_key("state") {
					let state_str: &'static str = translations_data.translations[i].state.into();
					table["state"] = toml_edit::value(state_str);
				}
			}

			let translation_str = translation_doc.doc.to_string();
			std::fs::write(&translation_doc.file, translation_str).unwrap();
		}
	}

	fn update(&mut self, ctx: &egui::Context, frame: &epi::Frame) {
		unsafe {
			INT_TEXT_BOX_COUNT = 0;
		}
		egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
			egui::menu::bar(ui, |ui| {
				ui.selectable_value(&mut self.current_tab, "config", "Config");
				ui.selectable_value(&mut self.current_tab, "keyconfig", "Keyconfig");
				ui.selectable_value(&mut self.current_tab, "patches", "Patches");
				if self.have_translation {
					ui.selectable_value(&mut self.current_tab, "translation", "Translations");
				}
			});
		});

		egui::TopBottomPanel::bottom("bottom_panel").show(ctx, |ui| {
			ui.horizontal(|ui| {
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
					"config" => self.draw_config_tab(ui),
					"keyconfig" => self.draw_keyconfig_tab(ui),
					"patches" => self.draw_patches_tab(ui),
					"translation" => self.draw_translation_tab(ui),
					_ => (),
				};
			});
		});
	}
}

impl App {
	fn draw_config_tab(&mut self, ui: &mut egui::Ui) {
		ui.horizontal(|ui| {
			ui.label("FPS limit");
			int_text_box(&mut self.config.config.fps, 10.0, ui);
		});
		ui.horizontal(|ui| {
			ui.label("Internal resolution");
			int_text_box(&mut self.config.config.internal_res_x, 2.0, ui);
			int_text_box(&mut self.config.config.internal_res_y, 1.0, ui);
		});
		simple_checkbox("Fullscreen", &mut self.config.config.fullscreen, ui);
		ui.horizontal(|ui| {
			ui.label("Rumble Intensity");
			ui.add(egui::Slider::new(&mut self.config.config.rumble_intensity, 0..=100).text(""));
		});
	}

	fn draw_keyconfig_tab(&mut self, ui: &mut egui::Ui) {
		egui::CollapsingHeader::new("Change Game State").show(ui, |ui| {
			App::draw_keyconfig_entry(ui, "TEST".to_string(), &mut self.config.keyconfig.test);
			App::draw_keyconfig_entry(
				ui,
				"SERVICE".to_string(),
				&mut self.config.keyconfig.service,
			);
			App::draw_keyconfig_entry(
				ui,
				"ADVERTISE".to_string(),
				&mut self.config.keyconfig.advertise,
			);
			App::draw_keyconfig_entry(ui, "GAME".to_string(), &mut self.config.keyconfig.game);
			App::draw_keyconfig_entry(
				ui,
				"DATA_TEST".to_string(),
				&mut self.config.keyconfig.data_test,
			);
			App::draw_keyconfig_entry(
				ui,
				"TEST_MODE".to_string(),
				&mut self.config.keyconfig.test_mode,
			);
			App::draw_keyconfig_entry(
				ui,
				"APP_ERROR".to_string(),
				&mut self.config.keyconfig.app_error,
			);
		});

		egui::CollapsingHeader::new("Gameplay").show(ui, |ui| {
			App::draw_keyconfig_entry(ui, "START".to_string(), &mut self.config.keyconfig.start);
			App::draw_keyconfig_entry(
				ui,
				"TRIANGLE".to_string(),
				&mut self.config.keyconfig.triangle,
			);
			App::draw_keyconfig_entry(ui, "SQUARE".to_string(), &mut self.config.keyconfig.square);
			App::draw_keyconfig_entry(ui, "CROSS".to_string(), &mut self.config.keyconfig.cross);
			App::draw_keyconfig_entry(ui, "CIRCLE".to_string(), &mut self.config.keyconfig.circle);
			App::draw_keyconfig_entry(
				ui,
				"LEFT_LEFT".to_string(),
				&mut self.config.keyconfig.left_left,
			);
			App::draw_keyconfig_entry(
				ui,
				"LEFT_RIGHT".to_string(),
				&mut self.config.keyconfig.left_right,
			);
			App::draw_keyconfig_entry(
				ui,
				"RIGHT_LEFT".to_string(),
				&mut self.config.keyconfig.right_left,
			);
			App::draw_keyconfig_entry(
				ui,
				"RIGHT_RIGHT".to_string(),
				&mut self.config.keyconfig.right_right,
			);
		});

		egui::CollapsingHeader::new("Unlocked camera").show(ui, |ui| {
			App::draw_keyconfig_entry(
				ui,
				"CAMERA_UNLOCK_TOGGLE".to_string(),
				&mut self.config.keyconfig.camera_unlock_toggle,
			);
			App::draw_keyconfig_entry(
				ui,
				"CAMERA_MOVE_FORWARD".to_string(),
				&mut self.config.keyconfig.camera_move_forward,
			);
			App::draw_keyconfig_entry(
				ui,
				"CAMERA_MOVE_BACKWARD".to_string(),
				&mut self.config.keyconfig.camera_move_backward,
			);
			App::draw_keyconfig_entry(
				ui,
				"CAMERA_MOVE_LEFT".to_string(),
				&mut self.config.keyconfig.camera_move_left,
			);
			App::draw_keyconfig_entry(
				ui,
				"CAMERA_MOVE_RIGHT".to_string(),
				&mut self.config.keyconfig.camera_move_right,
			);
			App::draw_keyconfig_entry(
				ui,
				"CAMERA_MOVE_UP".to_string(),
				&mut self.config.keyconfig.camera_move_up,
			);
			App::draw_keyconfig_entry(
				ui,
				"CAMERA_MOVE_DOWN".to_string(),
				&mut self.config.keyconfig.camera_move_down,
			);
			App::draw_keyconfig_entry(
				ui,
				"CAMERA_ROTATE_CW".to_string(),
				&mut self.config.keyconfig.camera_rotate_cw,
			);
			App::draw_keyconfig_entry(
				ui,
				"CAMERA_ROTATE_CCW".to_string(),
				&mut self.config.keyconfig.camera_rotate_ccw,
			);
			App::draw_keyconfig_entry(
				ui,
				"CAMERA_ZOOM_IN".to_string(),
				&mut self.config.keyconfig.camera_zoom_in,
			);
			App::draw_keyconfig_entry(
				ui,
				"CAMERA_ZOOM_OUT".to_string(),
				&mut self.config.keyconfig.camera_zoom_out,
			);
			App::draw_keyconfig_entry(
				ui,
				"CAMERA_MOVE_FAST".to_string(),
				&mut self.config.keyconfig.camera_move_fast,
			);
			App::draw_keyconfig_entry(
				ui,
				"CAMERA_MOVE_SLOW".to_string(),
				&mut self.config.keyconfig.camera_move_slow,
			);
		});
	}

	fn draw_keyconfig_entry(ui: &mut egui::Ui, name: String, vec: &mut Vec<Buttons>) {
		let mut hasher = std::collections::hash_map::DefaultHasher::new();
		egui::CollapsingHeader::new(name).show(ui, |ui| {
			/* Borrow checker throws a fit if this is at the bottom */
			if ui.add(egui::Button::new("+")).clicked() {
				vec.push(Buttons::F1);
			}
			for (i, button) in vec.into_iter().enumerate() {
				button.hash(&mut hasher);
				ui.horizontal(|ui| {
					if ui.add(egui::Button::new("-")).clicked() {
						//vec.remove(i);
					}
					egui::ComboBox::from_id_source(hasher.finish())
						.selected_text(format!("{:?}", button))
						.width(ui.available_width() / 4.0)
						.show_ui(ui, |ui| {
							for button_variant in Buttons::iter() {
								ui.selectable_value(
									button,
									button_variant,
									format!("{:?}", button_variant),
								);
							}
						});
				});
			}
		});
	}

	fn draw_patches_tab(&mut self, ui: &mut egui::Ui) {
		let mut hasher = std::collections::hash_map::DefaultHasher::new();
		for patch in &mut self.config.patches {
			egui::CollapsingHeader::new(&patch.name).show(ui, |ui| {
				ui.label(format!("Author: {}", patch.author));
				simple_checkbox("Enable", &mut patch.enabled, ui);
				for internal_patch in &mut patch.patches {
					internal_patch.address.hash(&mut hasher);
					egui::CollapsingHeader::new(format!("{:#X}", internal_patch.address)).show(
						ui,
						|ui| {
							ui.horizontal(|ui| {
								ui.label("Address");
								hex_text_box(&mut internal_patch.address, 0, 4.0, ui);
							});
							ui.horizontal(|ui| {
								ui.label("Data Type");
								egui::ComboBox::from_id_source(hasher.finish())
									.selected_text(format!("{:?}", internal_patch.data_type))
									.width(ui.available_width() / 4.0)
									.show_ui(ui, |ui| {
										for variant in DataTypes::iter() {
											ui.selectable_value(
												&mut internal_patch.data_type,
												variant,
												format!("{:?}", variant),
											);
										}
									});
							});
							ui.horizontal(|ui| {
								ui.label("Data");
								match internal_patch.data_type {
									DataTypes::i8 | DataTypes::u8 => {
										hex_text_box(&mut internal_patch.data_int, 2, 5.0, ui);
									}
									DataTypes::i16 | DataTypes::u16 => {
										hex_text_box(&mut internal_patch.data_int, 4, 5.0, ui);
									}
									DataTypes::i32 | DataTypes::u32 => {
										hex_text_box(&mut internal_patch.data_int, 8, 5.0, ui);
									}
									DataTypes::i64 => {
										hex_text_box(&mut internal_patch.data_int, 16, 5.0, ui);
									}
									DataTypes::i8_arr | DataTypes::u8_arr => {
										let mut count = internal_patch.data_int_arr.len();
										for data in &mut internal_patch.data_int_arr {
											hex_text_box(data, 2, count as f32, ui);
											count -= 1;
										}
									}
									DataTypes::i16_arr | DataTypes::u16_arr => {
										let mut count = internal_patch.data_int_arr.len();
										for data in &mut internal_patch.data_int_arr {
											hex_text_box(data, 4, count as f32, ui);
											count -= 1;
										}
									}
									DataTypes::i32_arr | DataTypes::u32_arr => {
										let mut count = internal_patch.data_int_arr.len();
										for data in &mut internal_patch.data_int_arr {
											hex_text_box(data, 8, count as f32, ui);
											count -= 1;
										}
									}
									DataTypes::i64_arr => {
										let mut count = internal_patch.data_int_arr.len();
										for data in &mut internal_patch.data_int_arr {
											hex_text_box(data, 16, count as f32, ui);
											count -= 1;
										}
									}
									DataTypes::string => {
										ui.add_sized(
											vec2_x_modify(&mut ui.available_size(), 4.0),
											egui::TextEdit::singleline(
												&mut internal_patch.data_string,
											),
										);
									}
								}
							});
						},
					);
				}
			});
		}
	}

	fn draw_translation_tab(&mut self, ui: &mut egui::Ui) {
		let mut hasher = std::collections::hash_map::DefaultHasher::new();
		for translation in &mut self.config.translations {
			egui::CollapsingHeader::new(&translation.language).show(ui, |ui| {
				ui.label(format!("Author: {}", translation.author));
				simple_checkbox("Enable", &mut translation.enabled, ui);
				if ui.add(egui::Button::new("+")).clicked() {
					translation
						.translations
						.push(InternalTranslation::default());
				}
				for (i, internal_translation) in
					&mut translation.translations.iter_mut().enumerate()
				{
					ui.horizontal(|ui| {
						if ui.add(egui::Button::new("-")).clicked() {
							//vec.remove(i);
						}
						if internal_translation.state != SubGameStates::SUB_MAX {
							internal_translation.old.hash(&mut hasher);
							egui::ComboBox::from_id_source(hasher.finish())
								.selected_text(format!("{:?}", internal_translation.state))
								.width(ui.available_width() / 4.0)
								.show_ui(ui, |ui| {
									for variant in SubGameStates::iter() {
										if variant == SubGameStates::SUB_MAX {
											continue;
										}

										ui.selectable_value(
											&mut internal_translation.state,
											variant,
											format!("{:?}", variant),
										);
									}
								});
						}
						ui.add_sized(
							vec2_x_modify(&mut ui.available_size(), 2.0),
							egui::TextEdit::singleline(&mut internal_translation.old),
						);
						ui.label("->");
						ui.add_sized(
							ui.available_size(),
							egui::TextEdit::singleline(&mut internal_translation.new),
						);
					});
				}
			});
		}
	}
}

static mut EDIT_STRING: Option<String> = None;

#[allow(unused_assignments)]
fn int_text_box<T: num::Integer>(value: &mut T, size: f32, ui: &mut egui::Ui)
where
	T: std::fmt::Display,
	T: FromStr,
	<T as FromStr>::Err: std::fmt::Debug,
{
	let mut count = 0;
	unsafe {
		count = INT_TEXT_BOX_COUNT;
	}
	let kb_edit_id = egui::Id::new(format!("int_text_box::{}", count));

	if ui.memory().has_focus(kb_edit_id) {
		let mut text = String::new();
		unsafe {
			text = EDIT_STRING.take().unwrap_or(value.to_string());
		}
		ui.add_sized(
			vec2_x_modify(&mut ui.available_size(), size),
			egui::TextEdit::singleline(&mut text).id(kb_edit_id),
		);
		if ui.input().key_pressed(egui::Key::Enter) {
			ui.memory().surrender_focus(kb_edit_id);
			unsafe {
				let parsed = text.parse::<T>();
				if parsed.is_ok() {
					*value = parsed.unwrap();
				}
				EDIT_STRING = None;
			}
		} else {
			unsafe { EDIT_STRING = Some(text) }
		}
	} else if ui
		.add_sized(
			vec2_x_modify(&mut ui.available_size(), size),
			egui::Button::new(value.to_string()),
		)
		.clicked()
	{
		ui.memory().request_focus(kb_edit_id);
		unsafe {
			EDIT_STRING = None;
		}
	}
	unsafe {
		INT_TEXT_BOX_COUNT += 1;
	}
}

#[allow(unused_assignments)]
fn hex_text_box<T: num::Integer>(value: &mut T, min_bytes: i32, size: f32, ui: &mut egui::Ui)
where
	T: std::fmt::Display,
	T: FromStr,
	<T as FromStr>::Err: std::fmt::Debug,
	<T as num::Num>::FromStrRadixErr: std::fmt::Debug,
	T: std::fmt::UpperHex,
{
	let mut count = 0;
	unsafe {
		count = INT_TEXT_BOX_COUNT;
	}
	let kb_edit_id = egui::Id::new(format!("int_text_box::{}", count));
	let value_str = match min_bytes {
		2 => format!("{:#04X}", value),
		4 => format!("{:#06X}", value),
		8 => format!("{:#010X}", value),
		16 => format!("{:#018X}", value),
		_ => format!("{:#X}", value),
	};

	if ui.memory().has_focus(kb_edit_id) {
		let mut text = String::new();
		unsafe {
			text = EDIT_STRING.take().unwrap_or(value_str);
		}
		ui.add_sized(
			vec2_x_modify(&mut ui.available_size(), size),
			egui::TextEdit::singleline(&mut text).id(kb_edit_id),
		);
		if ui.input().key_pressed(egui::Key::Enter) {
			ui.memory().surrender_focus(kb_edit_id);
			unsafe {
				text = text.trim_start_matches("0x").to_string();
				let parsed = T::from_str_radix(&text, 16);
				if parsed.is_ok() {
					*value = parsed.unwrap();
				}
				EDIT_STRING = None;
			}
		} else {
			unsafe { EDIT_STRING = Some(text) }
		}
	} else if ui
		.add_sized(
			vec2_x_modify(&mut ui.available_size(), size),
			egui::Button::new(value_str),
		)
		.clicked()
	{
		ui.memory().request_focus(kb_edit_id);
		unsafe {
			EDIT_STRING = None;
		}
	}
	unsafe {
		INT_TEXT_BOX_COUNT += 1;
	}
}

fn simple_checkbox(label: &str, value: &mut bool, ui: &mut egui::Ui) {
	ui.horizontal(|ui| {
		ui.label(label);
		ui.checkbox(value, "");
	});
}

fn vec2_x_modify(size: &mut egui::Vec2, modify: f32) -> egui::Vec2 {
	size.x /= modify;
	*size
}

fn read_toml_integer_array_to_vec(vec: &mut Vec<i64>, array: &toml_edit::Item) {
	for value in array.as_array().unwrap() {
		vec.push(value.as_integer().unwrap());
	}
}

fn read_toml_array_to_vec<T>(vec: &mut Vec<T>, array: &toml_edit::Item)
where
	T: FromStr,
	<T as FromStr>::Err: std::fmt::Debug,
{
	for value in array.as_array().unwrap() {
		vec.push(T::from_str(value.as_str().unwrap()).unwrap());
	}
}

#[allow(unused_assignments, unused_variables)]
fn read_vec_to_toml_array<T>(vec: &Vec<T>, array: &mut toml_edit::Item)
where
	T: FromStr,
	<T as FromStr>::Err: std::fmt::Debug,
	toml_edit::Value: From<T>,
	T: Clone,
{
	let diff = vec.len() - array.as_array().unwrap().len();
	for _ in 0..diff {
		array
			.as_array_mut()
			.unwrap()
			.push(toml_edit::Array::default());
	}
	for (i, value) in array.as_array_mut().unwrap().iter_mut().enumerate() {
		*value = toml_edit::value(vec[i].clone()).into_value().unwrap();
	}
}

fn buttons_vec_to_string(vec: &Vec<Buttons>) -> Vec<String> {
	let mut ret: Vec<String> = vec![];

	for button in vec {
		let button_str: &'static str = button.into();
		ret.push(button_str.to_string());
	}

	ret
}
