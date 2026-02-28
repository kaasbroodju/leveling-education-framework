use crate::domain::{HBOIExampleResponse, HBOIResponseBody, VaardighedenResponseBody};
use lazy_static::lazy_static;

const SKILL_CONTENT: &'static str = include_str!("./../app/data/vaardigheden-nl.json");
lazy_static! {
	pub static ref SKILL_DATA: VaardighedenResponseBody =
		serde_json::from_str::<VaardighedenResponseBody>(SKILL_CONTENT).unwrap();
}

const HBOI_CONTENT: &'static str = include_str!("./../app/data/hboi-nl.json");
lazy_static! {
	pub static ref HBOI_DATA: HBOIResponseBody =
		serde_json::from_str::<HBOIResponseBody>(HBOI_CONTENT).unwrap();
}

const EXAMPLES_CONTENT: &'static str = include_str!("./../app/data/examples.json");
lazy_static! {
	pub static ref EXAMPLES_DATA: Vec<HBOIExampleResponse> =
		serde_json::from_str::<Vec<HBOIExampleResponse>>(EXAMPLES_CONTENT).unwrap();
}

#[cfg(test)]
mod compile_time_tests {
	use super::*;

	// Test 1: Simpele compile-time test - als dit compileert, zijn de basics OK
	#[test]
	fn test_skill_data_compiles() {
		let _data = &*SKILL_DATA;
		// Als dit compileert, betekent het dat:
		// 1. De JSON syntax correct is
		// 2. De structuur matched met VaardighedenResponseBody
		// 3. serde_json::from_str succesvol is
	}

	#[test]
	fn test_hboi_data_compiles() {
		let _data = &*HBOI_DATA;
		// Zelfde logica voor HBOI data
	}

	#[test]
	fn test_examples_data_compiles() {
		let _data = &*EXAMPLES_DATA;
		// Zelfde logica voor Examples data
	}

	// Test 2: Verificatie van niet-lege data
	#[test]
	fn test_data_not_empty() {
		// Test dat de data daadwerkelijk content heeft
		let skill_data = &*SKILL_DATA;
		// Voeg hier checks toe afhankelijk van je struct layout
		// Bijvoorbeeld:
		// assert!(!skill_data.skills.is_empty());

		let hboi_data = &*HBOI_DATA;
		// assert!(!hboi_data.activities.is_empty());

		let examples_data = &*EXAMPLES_DATA;
		assert!(!examples_data.is_empty());
	}

	// Test 3: Structuur validatie
	#[test]
	fn test_skill_data_structure() {
		let skill_data = &*SKILL_DATA;

		// Test specifieke eigenschappen van je data structuur
		// Bijvoorbeeld als je verwacht dat bepaalde keys bestaan:
		// for skill in &skill_data.skills {
		//     assert!(!skill.name.is_empty());
		//     assert!(!skill.description.is_empty());
		// }
	}

	// Test 4: Cross-referentie validatie
	#[test]
	fn test_data_consistency() {
		let skill_data = &*SKILL_DATA;
		let hboi_data = &*HBOI_DATA;
		let examples_data = &*EXAMPLES_DATA;

		// Test dat data consistent is tussen verschillende bestanden
		// Bijvoorbeeld: alle skills in examples moeten bestaan in skill_data
		// Of: alle HBOI referenties in examples moeten bestaan in hboi_data
	}
}
