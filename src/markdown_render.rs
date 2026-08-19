use crate::domain::{
	BeroepsRollenResponseBody, HBOIExampleResponse, HBOIResponseBody, VaardighedenResponseBody,
};

pub fn vaardigheden_to_markdown(data: &VaardighedenResponseBody) -> String {
	let mut out = String::from("# Vaardigheden\n\n");

	for (skill, skill_description) in data {
		out.push_str(&format!("## {}\n\n", skill.to_text()));
		out.push_str(&skill_description.description);
		out.push_str("\n\n");

		for (level, level_description) in &skill_description.level_description {
			out.push_str(&format!("### {}", level.to_text()));
			if let Some(subtitle) = &level_description.subtitle {
				out.push_str(&format!(" — {subtitle}"));
			}
			out.push_str("\n\n");
			out.push_str(&level_description.description);
			out.push_str("\n\n");
			if let Some(extra) = &level_description.extra_description {
				out.push_str("**Extra context:**\n\n");
				out.push_str(extra);
				out.push_str("\n\n");
			}
		}
	}

	out
}

pub fn beroepsrollen_to_markdown(data: &BeroepsRollenResponseBody) -> String {
	let mut out = String::from("# Beroepsrollen\n\n");

	for (guild, rol) in data {
		out.push_str(&format!("## {} — {}\n\n", guild.get_short_name(), rol.name));
		out.push_str(&rol.description);
		out.push_str("\n\n");
		out.push_str(&format!("**Voorbeelden:** {}\n\n", rol.examples));
		out.push_str(&format!(
			"**Primaire architectuurlaag:** {:#?}\n\n",
			rol.primary_layer
		));

		if !rol.secondary_layers.is_empty() {
			let layers: Vec<String> = rol
				.secondary_layers
				.iter()
				.map(|layer| format!("{layer:#?}"))
				.collect();
			out.push_str(&format!(
				"**Secundaire architectuurlagen:** {}\n\n",
				layers.join(", ")
			));
		}

		out.push_str("### Roadmap\n\n");
		out.push_str(&format!("- **Taakgericht:** {}\n", rol.roadmap.level_one));
		out.push_str(&format!("- **Probleemgericht:** {}\n", rol.roadmap.level_two));
		out.push_str(&format!("- **Situatiegericht:** {}\n\n", rol.roadmap.level_three));

		if !rol.roadmap.challenges.is_empty() {
			out.push_str("**Uitdagingen:**\n\n");
			for challenge in &rol.roadmap.challenges {
				out.push_str(&format!("- {challenge}\n"));
			}
			out.push('\n');
		}

		if !rol.roadmap.resources.is_empty() {
			out.push_str("**Bronnen:**\n\n");
			for resource in &rol.roadmap.resources {
				out.push_str(&format!("- [{}]({})\n", resource.text, resource.url));
			}
			out.push('\n');
		}

		if !rol.example_jobs.is_empty() {
			out.push_str("**Voorbeeldvacatures:**\n\n");
			for (title, description) in &rol.example_jobs {
				out.push_str(&format!("- **{title}**: {description}\n"));
			}
			out.push('\n');
		}
	}

	out
}

pub fn hboi_to_markdown(data: &HBOIResponseBody) -> String {
	let mut out = String::from("# Beroepstaken (HBO-i)\n\n");

	for (key, levels) in data {
		out.push_str(&format!("## {key}\n\n"));

		for (level, level_description) in levels {
			out.push_str(&format!("### {}\n\n", level.to_text()));
			if let Some(subtitle) = &level_description.subtitle {
				out.push_str(&format!("*{subtitle}*\n\n"));
			}
			out.push_str(&level_description.description);
			out.push_str("\n\n");
			if let Some(extra) = &level_description.extra_description {
				out.push_str("**Extra context:**\n\n");
				out.push_str(extra);
				out.push_str("\n\n");
			}
		}
	}

	out
}

pub fn beroepsproducten_to_markdown(data: &[HBOIExampleResponse]) -> String {
	let mut out = String::from("# Beroepsproducten\n\n");

	for example in data {
		out.push_str(&format!(
			"- **{}** — {:#?} / {} / {}\n",
			example.title,
			example.architecture_layer,
			example.activity.to_text(),
			example.guild.get_short_name()
		));
	}

	out
}
