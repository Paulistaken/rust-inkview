#[macro_use]
extern crate lazy_static;

use bindgen::callbacks::{ParseCallbacks, IntKind, EnumVariantValue};
use std::collections::BTreeMap;
use std::env;
use std::fs::OpenOptions;
use std::io::Write;
use std::path::PathBuf;
use std::sync::{Arc, Mutex};


struct Enum {
	name: String,
	kind: IntKind,
	variant: BTreeMap<i64, String>,
}

impl Enum {
	fn new(name: String, kind: IntKind) -> Self {
		Self {
			name: name,
			kind: kind,
			variant: BTreeMap::new(),
		}
	}
}

lazy_static! {
	static ref ENUMS: Arc<Mutex<BTreeMap<String, Enum>>> = Arc::new(Mutex::new(BTreeMap::new()));
}

fn main() -> std::io::Result<()> {
	// Tell cargo to tell rustc to link the inkview
	// shared library.
	println!("cargo:rustc-link-lib=inkview");

	#[derive(Debug)]
	struct InkViewTypeChooser;

	impl ParseCallbacks for InkViewTypeChooser {
		fn int_macro(&self, name: &str, value: i64) -> Option<IntKind> {
			let mutex = Arc::clone(&ENUMS);
			let mut enum_map = mutex.lock().unwrap();
			for (prefix, enum_kind) in &mut *enum_map {
				if name.starts_with(prefix.as_str()) {
					let mut variant_name = String::from(&name[prefix.len()..]);
					if let Some(first) = variant_name.chars().next() {
						if first.is_digit(10) {
							variant_name.insert_str(0, &enum_kind.name.to_uppercase());
						}
					}
					enum_kind.variant.insert(value, variant_name);
					return Some(enum_kind.kind);
				}
			}
			if value >= i32::min_value() as i64 &&
			   value <= i32::max_value() as i64 {
					Some(IntKind::I32)
			} else {
					None
			}
		}

		fn enum_variant_name(&self, enum_name: Option<&str>, original_variant_name: &str, _variant_value: EnumVariantValue) -> Option<String> {
			match enum_name {
				Some("PANEL_FLAGS") => Some(original_variant_name[6..].to_string()),
				_ => None
			}
		}

		fn item_name(&self, original_item_name: &str) -> Option<String> {
			match original_item_name {
				"PANEL_FLAGS" => Some(String::from("PanelType")),
				_ => None
			}
		}
	}

	{
		let mutex = Arc::clone(&ENUMS);
		let mut enum_map = mutex.lock().unwrap();
		enum_map.insert(String::from("EVT_"), Enum::new(String::from("Event"), IntKind::I32));
		enum_map.insert(String::from("IV_KEY_"), Enum::new(String::from("Key"), IntKind::I32));
		enum_map.insert(String::from("REQ_"), Enum::new(String::from("Request"), IntKind::I32));
		enum_map.insert(String::from("ICON_"), Enum::new(String::from("Icon"), IntKind::I32));
		enum_map.insert(String::from("DEF_"), Enum::new(String::from("Button"), IntKind::I32));
		enum_map.insert(String::from("DITHER_"), Enum::new(String::from("Dither"), IntKind::I32));
	}

	// The bindgen::Builder is the main entry point
	// to bindgen, and lets you build up options for
	// the resulting bindings.
	let bindings = bindgen::Builder::default()
		// The input header we would like to generate
		// bindings for.
		.header_contents(
			"inkview.h",
			"#include <inkview.h>
			void DrawCircleLine(int x1, int y1, int x2, int y2, int width, int color);"
		)
		.allowlist_var("[A-Z]+DIR[0-9]?")
		.allowlist_var("[A-Z]+DATA[0-9]?")
		.allowlist_var("[A-Z]+PATH[0-9]?")
		.allowlist_var("[A-Z]+PROFILES?[0-9]?")
		.allowlist_var("[A-Z]+FILE")
		.allowlist_var("USER[A-Z]+")
		.allowlist_var("SYSTEM[A-Z]+")
		.allowlist_var("[A-Z][0-9A-Z_]*_APP(_PATH)?")
		.allowlist_var("STATECLEANER")
		.allowlist_var("[A-Z]+SCRIPT")
		.allowlist_var("NETAGENT[A-Z]*")
		.allowlist_var("[A-Z]+APP")
		.allowlist_var("POCKETBOOKSIG")
		.allowlist_var("LASTOPENBOOKS")
		.allowlist_var("CURRENTBOOK_SAVE")
		.allowlist_var("FAVORITES")
		.allowlist_var("CURRENTBOOK")
		.allowlist_var("BOOKSHELFSTATE")
		.allowlist_var("BOOKSHELFSTATE_NV")
		.allowlist_var("DICKEYBOARD")
		.allowlist_var("URLHISTORY")
		.allowlist_var("WEBCACHE[A-Z]*")
		.allowlist_var("WIDGETS[A-Z]+")
		.allowlist_var("SWUPDATESTATUS")
		.allowlist_var("[A-Z]+FOLDER")
		.allowlist_var("SOCIAL[A-Z_]+")
		.allowlist_var("[A-Z][0-9A-Z_]*_DIRECTORY")
		.allowlist_var("[A-Z][0-9A-Z_]*_FILE")
		.allowlist_var("[A-Z][0-9A-Z_]*_CFG")
		.allowlist_var("[A-Z][0-9A-Z_]*_PATH")
		.allowlist_var("BROWSER_FOR_AUTH")
		.allowlist_var("READER_[0-9A-Z_]+")
		.allowlist_var("OBREEY_[0-9A-Z_]+")
		.allowlist_var("PROFILE_[0-9A-Z_]+")
		.allowlist_var("SYSTEMDEPTH")
		.allowlist_var("MAXMSGSIZE")
		.allowlist_type("AvrcpCommands")
		.allowlist_function("IS[A-Z]+EVENT")
		//.allowlist_var("EVT_[0-9A-Z_]+")
		//.allowlist_var("IV_KEY_[0-9A-Z_]+")
		.allowlist_var("KEYMAPPING_KEY_[0-9A-Z_]+")
		.allowlist_var("BLACK")
		.allowlist_var("[DL]GRAY")
		.allowlist_var("WHITE")
		.allowlist_var("ITEM_[0-9A-Z_]+")
		.allowlist_var("KBD_[0-9A-Z_]+")
		//.allowlist_var("ICON_[0-9A-Z_]+")
		//.allowlist_var("DEF_BUTTON[0-9]")
		.allowlist_var("NO_DISMISS")
		.allowlist_var("WITH_SIZE")
		.allowlist_var("PANELICON_[0-9A-Z_]+")
		.allowlist_var("LIST(FLAG)?_[0-9A-Z_]+")
		.allowlist_var("BMK_[0-9A-Z_]+")
		.allowlist_var("CFG_[0-9A-Z_]+")
		.allowlist_var("[A-Z]+TASKS?")
		.allowlist_var("TASK_[0-9A-Z_]+")
		.allowlist_var("RQL_[0-9A-Z_]+")
		//.allowlist_var("REQ_[0-9A-Z_]+")
		.allowlist_var("ALIGN_[A-Z]+")
		.allowlist_var("VALIGN_[A-Z]+")
		.allowlist_var("ROTATE")
		.allowlist_var("HYPHENS")
		.allowlist_var("DOTS")
		.allowlist_var("RTLAUTO")
		.allowlist_var("UNDERLINE")
		.allowlist_var("STRETCH")
		.allowlist_var("TILE")
		.allowlist_var("TO_UPPER")
		.allowlist_var("FR_[A-Z]+")
		.allowlist_var("ARROW_[A-Z]+")
		.allowlist_var("SYMBOL_[A-Z]+")
		.allowlist_var("IMAGE_[A-Z]+")
		.allowlist_var("ROTATE[0-9]+")
		.allowlist_var("[XY]MIRROR")
		.allowlist_var("A2DITHER")
		//.allowlist_var("DITHER_[A-Z]+")
		.allowlist_var("QN_[A-Z]+")
		.allowlist_type("PB_(TTS_)?STATE")
		.allowlist_var("MP_[A-Z]+")
		.allowlist_var("FTYPE_[A-Z]+")
		.allowlist_var("OB_[A-Z]+")
		.allowlist_var("NET_[0-9A-Z]+")
		.allowlist_var("CONN_[0-9A-Z]+")
		.allowlist_var("BLUETOOTH_[A-Z]+")
		.allowlist_type("WIFI_SECURITY")
		.allowlist_type("NET_STATE")
		.allowlist_var("VN_[A-Z]+")
		.allowlist_var("A2DP_[0-9A-Z_]+")
		.allowlist_var("CF_[0-9A-Z_]+")
		.allowlist_var("FONT_ACTIVATE_CODE")
		.allowlist_function("TOUCHDRAGDEADZONE")
		.allowlist_type("FONT_TYPE")
		.allowlist_type("SideFlags")
		.allowlist_type("PANEL_FLAGS")
		.allowlist_function("iv_[0-9a-z_]+")
		.allowlist_function("DEFAULTFONT[A-Z]*")
		.allowlist_type("irect")
		.allowlist_type("ibitmap")
		.allowlist_type("control_panel")
		.allowlist_type("TransparentHandle")
		.allowlist_type("ihash(_item)?")
		.allowlist_type("ifont[0-9a-z_]+")
		.allowlist_type("FONT_MENU_FLAGS")
		.allowlist_type("iuser_font")
		.allowlist_type("imenu[0-9a-z_]+")
		.allowlist_type("icanvas")
		.allowlist_type("icontext_menu[0-9a-z_]+")
		.allowlist_type("font_selector_properties")
		.allowlist_type("iapp_caption")
		.allowlist_type("itaskmgr")
		.allowlist_type("ipager")
		.allowlist_type("iselection")
		.allowlist_type("iappstyle")
		.allowlist_type("ievent")
		.allowlist_type("iconfig(edit)?")
		.allowlist_type("oldconfigedit")
		.allowlist_type("tocentry")
		.allowlist_type("itimer")
		.allowlist_type("bookinfo")
		.allowlist_type("iv_[0-9a-z_]+")
		.allowlist_type("(sub)?taskinfo")
		.allowlist_type("network_interface[a-z_]*")
		.allowlist_type("bt_[0-9a-z_]+")
		.allowlist_type("obex_[0-9a-z_]+")
		.allowlist_type("audio_output[a-z_]*")
		.allowlist_type("icolor_map")
		.allowlist_type("APPLICATION_ATTRIBUTE")
		.allowlist_function("(Open|Close)[A-Z][A-Za-z]*")
		.allowlist_function("InkViewMain")
		.allowlist_function("CloseApp")
		.allowlist_function("InitInkview")
		.allowlist_function("iRect")
		.allowlist_function("Screen(Width|Height)")
		.allowlist_function("[SG]et[A-Z][A-Za-z]*")
		.allowlist_function("[SG]et(Global|GSensor)?Orientation")
		.allowlist_var("GSENSOR_[A-Z]+")
		.allowlist_function("[A-Z][a-z]+GSensor(Enabled)?")
		.allowlist_type("estyle")
		.allowlist_function("Clear[A-Z][A-Za-z]*")
		.allowlist_function("ClearScreen")
		.allowlist_function("([SG]et|Merge)Clip(Rect)?")
		.allowlist_function("Draw[A-Z][A-Za-z]*")
		.allowlist_function("Fill[A-Z][A-Za-z]*")
		.allowlist_function("Invert[A-Z][A-Za-z]*")
		.allowlist_function("ColorMap[A-Z][A-Za-z]*")
		.allowlist_function("Dim[A-Z][A-Za-z]*")
		.allowlist_function("DitherArea((Quick|Pattern)2Level)?")
		.allowlist_function("QuickFloyd16Dither")
		.allowlist_function("Stretch[A-Z][A-Za-z]*")
		.allowlist_function("[SG]etCanvas")
		.allowlist_function("Repaint")
		.allowlist_function("CheckFramePointer")
		.allowlist_function("(Get|Is)?Pager[A-Z][A-Za-z]*")
		.allowlist_function("Transparent(Rect)?")
		.allowlist_function("(Load|Save)[A-Z][A-Za-z]*")
		.allowlist_function("(zLoad|New|Copy|Move|Tile|Mirror)Bitmap([A-Z][A-Za-z]*)?")
		.allowlist_function("SetTransparentColor")
		.allowlist_function("EnumFonts([A-Z][A-Za-z]*)?")
		.allowlist_function("FreeFontsForSort")
		.allowlist_function("(Open|Close|[SG]et)Font")
		.allowlist_function("TextRectHeight(Ex)?")
		.allowlist_function("(MinimalTextRect|Char|String|GetMultilineString)Width(Ext)?")
		.allowlist_function("RegisterFontList")
		.allowlist_function("SetTextStrength")
		.allowlist_function("(Full|Soft|Partial|Dynamic|Exit|IsInA2|Fine|HQ|Schedule|WaitFor)Update([A-Z][A-Za-z0-9]*)?")
		.allowlist_function("[SG]etEventHandler(Ex)?")
		.allowlist_function("SendEvent(Ex)?")
		.allowlist_function("(Flush|Is)AnyEvents")
		.allowlist_function("GetCurrentEventExData")
		.allowlist_function("ProcessEventLoop(Quick)?")
		.allowlist_function("PrepareForLoop")
		.allowlist_function("ClearOnExit")
		.allowlist_function("(Set(Hard|Weak)|Query|Clear)Timer(Ex|ByName)?")
		.allowlist_function("(Open|Update)Menu(Ex|3x3)?")
		.allowlist_function("(Open|Set|Create|Close)ContextMenu")
		.allowlist_function("GetMenuRect(Ex)?")
		.allowlist_function("Open(Dummy)?List")
		.allowlist_function("[SG]etListHeaderLevel")
		.allowlist_function("EnumKeyboards")
		.blocklist_item("O_[A-Z]+")
		.blocklist_item("E(NOT|IS)DIR")
		.blocklist_item("E[NM]FILE")
		.blocklist_item("ENODATA")
		.ctypes_prefix("c_types")
		.default_enum_style(bindgen::EnumVariation::Rust{non_exhaustive: false})
		.bitfield_enum("PanelType")
		.generate_comments(true)
		.layout_tests(false)
		.parse_callbacks(Box::new(InkViewTypeChooser))
		.prepend_enum_name(false)
		.rustfmt_bindings(true)
		.use_core()
		// Finish the builder and generate the bindings.
		.generate()
		// Unwrap the Result and panic on failure.
		.expect("Unable to generate bindings");

	// Write the bindings to the $OUT_DIR/bindings.rs file.
	let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
	let file = OpenOptions::new()
		.write(true)
		.truncate(true)
		.create(true)
		.open(out_path.join("bindings.rs"))?;
	let mut file_copy = file.try_clone()?;
	bindings.write(Box::new(file))?;
	{
		let mutex = Arc::clone(&ENUMS);
		let enum_map = mutex.lock().unwrap();
		for (_prefix, enum_kind) in &*enum_map {
			match enum_kind.kind {
				IntKind::I32 => {
					writeln!(file_copy, "#[repr(i32)]")?;
				},
				IntKind::U32 => {
					writeln!(file_copy, "#[repr(u32)]")?;
				},
				_ => {},
			}
			writeln!(file_copy, "#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, FromPrimitive)]")?;
			writeln!(file_copy, "pub enum {} {{", enum_kind.name)?;
			for (variant_value, variant_name) in &enum_kind.variant {
				writeln!(file_copy, "    {} = {},", variant_name, variant_value)?;
			}
			writeln!(file_copy, "}}")?;
		}
	}
	Ok(())
}
