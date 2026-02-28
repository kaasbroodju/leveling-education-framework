use crate::domain::Activiteit::{Adviseren, Analyseren, Manage_and_Control, Ontwerpen, Realiseren};
use crate::domain::Architectuurlaag::{
	Gebruikersinteractie, Hardwareinterfacing, Infrastructuur, Organisatieprocessen, Software,
};
use serde::{Deserialize, Serialize};
use std::collections::{BTreeMap, HashMap};

#[derive(
	Debug, Serialize, Deserialize, Clone, Eq, PartialEq, Hash, PartialOrd, Ord, FromFormField,
)]
pub enum Skill {
	#[serde(rename(serialize = "Overzicht creëren", deserialize = "Overzicht creëren"))]
	#[field(value = "Overzicht creëren")]
	Overzicht_creëren,

	#[serde(rename(serialize = "Kritisch oordelen", deserialize = "Kritisch oordelen"))]
	#[field(value = "Kritisch oordelen")]
	Kritisch_oordelen,

	#[serde(rename(
		serialize = "Juiste kennis ontwikkelen",
		deserialize = "Juiste kennis ontwikkelen"
	))]
	#[field(value = "Juiste kennis ontwikkelen")]
	Juiste_kennis_ontwikkelen,

	#[serde(rename(
		serialize = "Kwalitatief product maken",
		deserialize = "Kwalitatief product maken"
	))]
	#[field(value = "Kwalitatief product maken")]
	Kwalitatief_product_maken,

	#[serde(rename(serialize = "Plannen", deserialize = "Plannen"))]
	#[field(value = "Plannen")]
	Plannen,

	#[serde(rename(serialize = "Boodschap delen", deserialize = "Boodschap delen"))]
	#[field(value = "Boodschap delen")]
	Boodschap_delen,

	#[serde(rename(serialize = "Samenwerken", deserialize = "Samenwerken"))]
	#[field(value = "Samenwerken")]
	Samenwerken,

	#[serde(rename(serialize = "Flexibel opstellen", deserialize = "Flexibel opstellen"))]
	#[field(value = "Flexibel opstellen")]
	Flexibel_opstellen,

	#[serde(rename(serialize = "Pro-actief handelen", deserialize = "Pro-actief handelen"))]
	#[field(value = "Pro-actief handelen")]
	#[field(value = "Proactief handelen")]
	Proactief_handelen,

	#[serde(rename(serialize = "Reflecteren", deserialize = "Reflecteren"))]
	#[field(value = "Reflecteren")]
	Reflecteren,
}

impl Icon for Skill {
	fn to_icon(&self) -> &str {
		match &self {
			Skill::Overzicht_creëren => "explore",
			Skill::Kritisch_oordelen => "announcement",
			Skill::Juiste_kennis_ontwikkelen => "menu_book",
			Skill::Kwalitatief_product_maken => "biotech",
			Skill::Plannen => "calendar_month",
			Skill::Boodschap_delen => "co_present",
			Skill::Samenwerken => "handshake",
			Skill::Flexibel_opstellen => "accessibility_new",
			Skill::Proactief_handelen => "directions_run",
			Skill::Reflecteren => "psychology",
		}
	}
}

impl Skill {
	pub fn to_text(&self) -> &str {
		match &self {
			Skill::Overzicht_creëren => "Overzicht creëren",
			Skill::Kritisch_oordelen => "Kritisch oordelen",
			Skill::Juiste_kennis_ontwikkelen => "Juiste kennis ontwikkelen",
			Skill::Kwalitatief_product_maken => "Kwalitatief product maken",
			Skill::Plannen => "Plannen",
			Skill::Boodschap_delen => "Boodschap delen",
			Skill::Samenwerken => "Samenwerken",
			Skill::Flexibel_opstellen => "Flexibel opstellen",
			Skill::Proactief_handelen => "Pro-actief handelen",
			Skill::Reflecteren => "Reflecteren",
		}
	}
}

#[derive(Debug, Serialize, Deserialize, Clone, Eq, PartialEq, Hash, PartialOrd, Ord)]
pub enum Level {
	#[serde(rename(serialize = "1", deserialize = "1"))]
	Level1,

	#[serde(rename(serialize = "2", deserialize = "2"))]
	Level2,

	#[serde(rename(serialize = "3", deserialize = "3"))]
	Level3,

	#[serde(rename(serialize = "4", deserialize = "4"))]
	Level4,
}

impl Level {
	pub fn to_text(&self) -> &str {
		match self {
			Level::Level1 => "Niveau 1",
			Level::Level2 => "Niveau 2",
			Level::Level3 => "Niveau 3",
			Level::Level4 => "Niveau 4",
		}
	}
}

#[derive(
	Debug, Serialize, Deserialize, Clone, Eq, PartialEq, Hash, FromFormField, Ord, PartialOrd,
)]
pub enum Activiteit {
	#[serde(rename(serialize = "Analyseren", deserialize = "Analyseren"))]
	Analyseren,

	#[serde(rename(serialize = "Adviseren", deserialize = "Adviseren"))]
	Adviseren,

	#[serde(rename(serialize = "Ontwerpen", deserialize = "Ontwerpen"))]
	Ontwerpen,

	#[serde(rename(serialize = "Realiseren", deserialize = "Realiseren"))]
	Realiseren,

	#[serde(rename(serialize = "Manage & Control", deserialize = "Manage & Control"))]
	#[field(value = "Manage & Control")]
	#[field(value = "Manage %26 Control")]
	#[field(value = "Manage_and_Control")]
	Manage_and_Control,
}

impl Icon for Activiteit {
	fn to_icon(&self) -> &str {
		match &self {
			Activiteit::Analyseren => "biotech",
			Activiteit::Adviseren => "chat",
			Activiteit::Ontwerpen => "design_services",
			Activiteit::Realiseren => "build",
			Activiteit::Manage_and_Control => "tune",
		}
	}
}

impl Activiteit {
	pub fn to_text(&self) -> &str {
		match &self {
			Activiteit::Manage_and_Control => "Manage & Control",
			Activiteit::Analyseren => "Analyseren",
			Activiteit::Adviseren => "Adviseren",
			Activiteit::Ontwerpen => "Ontwerpen",
			Activiteit::Realiseren => "Realiseren",
		}
	}
}

#[derive(
	Debug, Serialize, Deserialize, Clone, Eq, PartialEq, Hash, FromFormField, Ord, PartialOrd,
)]

pub enum Architectuurlaag {
	#[serde(rename(
		serialize = "Gebruikersinteractie",
		deserialize = "Gebruikersinteractie"
	))]
	Gebruikersinteractie,

	#[serde(rename(
		serialize = "Organisatieprocessen",
		deserialize = "Organisatieprocessen"
	))]
	Organisatieprocessen,

	#[serde(rename(serialize = "Infrastructuur", deserialize = "Infrastructuur"))]
	Infrastructuur,

	#[serde(rename(serialize = "Software", deserialize = "Software"))]
	Software,

	#[serde(rename(serialize = "Hardwareinterfacing", deserialize = "Hardwareinterfacing"))]
	Hardwareinterfacing,
}

impl Icon for Architectuurlaag {
	fn to_icon(&self) -> &str {
		match &self {
			Architectuurlaag::Gebruikersinteractie => "smart_button",
			Architectuurlaag::Organisatieprocessen => "domain",
			Architectuurlaag::Infrastructuur => "storage",
			Architectuurlaag::Software => "laptop",
			Architectuurlaag::Hardwareinterfacing => "memory",
		}
	}
}

#[derive(Debug, Serialize, Deserialize, Clone, Eq, PartialEq, Hash, Ord, PartialOrd)]

pub enum Guild {
	#[serde(rename(serialize = "AI", deserialize = "AI"))]
	ArtificialIntelligence,

	#[serde(rename(serialize = "BE", deserialize = "BE"))]
	Backend,

	#[serde(rename(serialize = "BIM", deserialize = "BIM"))]
	BusinessItManagement,

	#[serde(rename(serialize = "CS", deserialize = "CS"))]
	CyberSecurity,

	#[serde(rename(serialize = "CI", deserialize = "CI"))]
	CloudInfrastructure,

	#[serde(rename(serialize = "FE", deserialize = "FE"))]
	Frontend,

	#[serde(rename(serialize = "UI/UX", deserialize = "UI/UX"))]
	UIUX,

	#[serde(rename(serialize = "TI", deserialize = "TI"))]
	Embedded,

	#[serde(rename(serialize = "GD", deserialize = "GD"))]
	GameDevelopment,
}

impl Guild {
	pub fn get_short_name(&self) -> &str {
		match &self {
			Guild::ArtificialIntelligence => "AI",
			Guild::Backend => "BE",
			Guild::BusinessItManagement => "BIT",
			Guild::CyberSecurity => "CS",
			Guild::CloudInfrastructure => "CI",
			Guild::Frontend => "FE",
			Guild::UIUX => "UI/UX",
			Guild::Embedded => "TI",
			Guild::GameDevelopment => "GD",
		}
	}

	pub fn get_color(&self) -> &str {
		match &self {
			Guild::ArtificialIntelligence => "#4B0082",
			Guild::Backend => "#B71C1C",
			Guild::BusinessItManagement => "#9A7300",
			Guild::CyberSecurity => "#085308ff",
			Guild::CloudInfrastructure => "#59c759ff",
			Guild::Frontend => "#D35400",
			Guild::UIUX => "#880E4F",
			Guild::Embedded => "#001F3F",
			Guild::GameDevelopment => "#8950C7",
		}
	}
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct LevelDescription {
	pub title: String,
	pub info: Option<String>,
}

#[derive(Debug, serde::Deserialize, serde::Serialize, Clone)]
pub struct HBOIExampleResponse {
	pub architecture_layer: Architectuurlaag,
	pub activity: Activiteit,
	pub guild: Guild,
	pub title: String,
}

pub type VaardighedenResponseBody = BTreeMap<Skill, BTreeMap<Level, LevelDescription>>;

use serde::{Deserializer, Serializer};
use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct HBOIKey {
	pub architectuurlaag: Architectuurlaag,
	pub activiteit: Activiteit,
}

impl HBOIKey {
	pub fn new(architectuurlaag: Architectuurlaag, activiteit: Activiteit) -> Self {
		Self {
			architectuurlaag,
			activiteit,
		}
	}
}

impl fmt::Display for HBOIKey {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		write!(
			f,
			"{} {}",
			serde_json::to_string(&self.architectuurlaag)
				.unwrap_or_default()
				.trim_matches('"'),
			serde_json::to_string(&self.activiteit)
				.unwrap_or_default()
				.trim_matches('"')
		)
	}
}

impl Serialize for HBOIKey {
	fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
	where
		S: Serializer,
	{
		// Serialiseer beide structs naar strings en combineer ze
		let arch_str = serde_json::to_string(&self.architectuurlaag)
			.map_err(serde::ser::Error::custom)?
			.trim_matches('"')
			.to_string();

		let act_str = serde_json::to_string(&self.activiteit)
			.map_err(serde::ser::Error::custom)?
			.trim_matches('"')
			.to_string();

		serializer.serialize_str(&format!("{} {}", arch_str, act_str))
	}
}

impl<'de> Deserialize<'de> for HBOIKey {
	fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
	where
		D: Deserializer<'de>,
	{
		let s = String::deserialize(deserializer)?;
		let mut parts = s.splitn(2, ' ');

		let arch_str = parts
			.next()
			.ok_or_else(|| serde::de::Error::custom("Missing architectuurlaag"))?;

		let act_str = parts
			.next()
			.ok_or_else(|| serde::de::Error::custom("Missing activiteit"))?;

		// Deserialise de string delen terug naar je structs
		let architectuurlaag: Architectuurlaag =
			serde_json::from_str(&format!("\"{}\"", arch_str)).map_err(serde::de::Error::custom)?;

		let activiteit: Activiteit =
			serde_json::from_str(&format!("\"{}\"", act_str)).map_err(serde::de::Error::custom)?;

		Ok(HBOIKey::new(architectuurlaag, activiteit))
	}
}

pub type HBOIResponseBody = BTreeMap<HBOIKey, BTreeMap<Level, LevelDescription>>;

// pub type HBOIResponseBody = BTreeMap<String, BTreeMap<Level, LevelDescription>>;

pub const PRODUCT_SKILLS: [Skill; 4] = [
	Skill::Overzicht_creëren,
	Skill::Kritisch_oordelen,
	Skill::Juiste_kennis_ontwikkelen,
	Skill::Kwalitatief_product_maken,
];
pub const SOCIAL_SKILLS: [Skill; 3] = [Skill::Plannen, Skill::Boodschap_delen, Skill::Samenwerken];
pub const PERSONAL_SKILLS: [Skill; 3] = [
	Skill::Flexibel_opstellen,
	Skill::Proactief_handelen,
	Skill::Reflecteren,
];
pub const ARCHITECTUURLAGEN: [Architectuurlaag; 5] = [
	Gebruikersinteractie,
	Organisatieprocessen,
	Infrastructuur,
	Software,
	Hardwareinterfacing,
];
pub const ACTIVITEITEN: [Activiteit; 5] = [
	Analyseren,
	Adviseren,
	Ontwerpen,
	Realiseren,
	Manage_and_Control,
];

pub trait Icon {
	fn to_icon(&self) -> &str;
}
