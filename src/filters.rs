use crate::domain::{
	Activiteit, Architectuurlaag, BeroepsRollenResponseBody, Guild, HBOIExampleResponse,
	HBOIResponseBody, Level, LevelDescription, Skill, SkillDescription, VaardighedenResponseBody,
};
use std::collections::BTreeMap;

pub fn filter_vaardigheden(
	data: &VaardighedenResponseBody,
	vaardigheid: Option<Skill>,
	niveau: Option<Level>,
) -> VaardighedenResponseBody {
	let mut result = BTreeMap::new();

	for (skill, skill_description) in data {
		if let Some(vaardigheid) = &vaardigheid {
			if skill != vaardigheid {
				continue;
			}
		}

		let level_description: BTreeMap<Level, LevelDescription> = skill_description
			.level_description
			.iter()
			.filter(|(level, _)| match &niveau {
				Some(niveau) => *level == niveau,
				None => true,
			})
			.map(|(level, description)| (level.clone(), description.clone()))
			.collect();

		result.insert(
			skill.clone(),
			SkillDescription {
				description: skill_description.description.clone(),
				level_description,
			},
		);
	}

	result
}

pub fn filter_beroepsrollen(
	data: &BeroepsRollenResponseBody,
	gilde: Option<Guild>,
) -> BeroepsRollenResponseBody {
	data
		.iter()
		.filter(|(guild, _)| match &gilde {
			Some(gilde) => *guild == gilde,
			None => true,
		})
		.map(|(guild, rol)| (guild.clone(), rol.clone()))
		.collect()
}

pub fn filter_hboi(
	data: &HBOIResponseBody,
	architectuurlaag: Option<Architectuurlaag>,
	activiteit: Option<Activiteit>,
	niveau: Option<Level>,
) -> HBOIResponseBody {
	let mut result = BTreeMap::new();

	for (key, levels) in data {
		if let Some(architectuurlaag) = &architectuurlaag {
			if &key.architectuurlaag != architectuurlaag {
				continue;
			}
		}
		if let Some(activiteit) = &activiteit {
			if &key.activiteit != activiteit {
				continue;
			}
		}

		let levels: BTreeMap<Level, LevelDescription> = levels
			.iter()
			.filter(|(level, _)| match &niveau {
				Some(niveau) => *level == niveau,
				None => true,
			})
			.map(|(level, description)| (level.clone(), description.clone()))
			.collect();

		result.insert(key.clone(), levels);
	}

	result
}

pub fn filter_beroepsproducten(
	data: &[HBOIExampleResponse],
	architectuurlaag: Option<Architectuurlaag>,
	activiteit: Option<Activiteit>,
	gilde: Option<Guild>,
) -> Vec<HBOIExampleResponse> {
	data
		.iter()
		.filter(|example| match &architectuurlaag {
			Some(architectuurlaag) => &example.architecture_layer == architectuurlaag,
			None => true,
		})
		.filter(|example| match &activiteit {
			Some(activiteit) => &example.activity == activiteit,
			None => true,
		})
		.filter(|example| match &gilde {
			Some(gilde) => &example.guild == gilde,
			None => true,
		})
		.cloned()
		.collect()
}
