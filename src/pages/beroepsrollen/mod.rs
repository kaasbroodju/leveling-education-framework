use tidos::{scoped_css, view, Component, Page};
use crate::components::card::Card;
use crate::components::navigation::guild_filter_matrix::GuildFilterMatrix;
use crate::data::BEROEPSROLLEN_DATA;
use crate::domain::{BeroepsRol, Guild, ARCHITECTUURLAGEN};

pub struct BeroepsRollenContent;

impl Component for BeroepsRollenContent {
	fn to_render(&self, page: &mut Page) {
		let beroepsrollen: Vec<(Guild, BeroepsRol)> = BEROEPSROLLEN_DATA
			.iter()
			.map(|(guild, rol)| (guild.clone(), rol.clone()))
			.collect();

		tidos::head! {
			<script>@html{include_str!("guild_filter.js")}</script>
		}

		view! {
			<div @class={"beroepsrollen.css"}>
				<GuildFilterMatrix />
				{#for (guild, rol) in beroepsrollen.into_iter()}
					<div data-guild={guild.get_short_name()}>
						<BeroepsRolCards rol={rol} />
					</div>
				{/for}
				<div data-guild={"HCAI"}>
					<Card>
						<h2>{"HCAI"}</h2>
						<p>{"Je ontwerpt en ontwikkelt een praktijkoplossing voor een AI-vraagstuk. Je integreert daarin kennis van technische mogelijkheden en meerdere human-centered (bijvoorbeeld ethische, sociaalmaatschappelijke en juridische) aspecten, zoekt hierin op kritische wijze een passende balans en schrijft een wetenschappelijke onderbouwing van gemaakte keuzes."}</p>
					</Card>
				</div>
			</div>
		}
	}
}

struct BeroepsRolCards {
	rol: BeroepsRol,
}

impl Component for BeroepsRolCards {
	fn to_render(&self, page: &mut Page) {
		// Each `{#slot:content}` block below compiles to a `Box<dyn Fn(&mut Page)>`
		// (implicitly `'static`), so it can't borrow from `self` — everything it
		// uses has to be owned locally first.
		let name = self.rol.name.clone();
		let name_lower = name.to_lowercase();
		let description = self.rol.description.clone();
		let examples = self.rol.examples.clone();
		let primary_layer = self.rol.primary_layer.clone();
		let secondary_layers = self.rol.secondary_layers.clone();
		let resources = self.rol.roadmap.resources.clone();
		let level_one = self.rol.roadmap.level_one.clone();
		let level_two = self.rol.roadmap.level_two.clone();
		let level_three = self.rol.roadmap.level_three.clone();
		let example_jobs: &Vec<(String, String)> = &self
			.rol
			.example_jobs
			.iter()
			.map(|(k, v)| (k.clone(), v.clone()))
			.collect();

		view! {
			<Card>
				<h2>{name}</h2>
				<hr/>
				<div class={"beroepsrol"}>
					<div>
						<section>
							<h3>{"Algemeen"}</h3>
							<p>{description}</p>
						</section>
						<section>
							<h3>{"Wat voor producten maak je"}</h3>
							<p>{examples}{", "}<a href="/beroepsproducten">{"verdere voorbeelden"}</a></p>
						</section>
						<section>
							<h3>{"HBO-i architectuurlagen"}</h3>
							<div class="hboi-layout">
								<div class="hboi">
									{#for layer in ARCHITECTUURLAGEN}
										<a href={"/beroepstaken?architectuurlaag={:?}", layer} class={"{}{}", if layer == primary_layer { "primary-layer" } else { "" }, if secondary_layers.contains(&layer) {"secondary-layer"} else { "" } }>{"{:?}", layer}</a>
									{/for}
								</div>
								<div class="hboi-text">
								<p>{"Een ICT-project omvat altijd de activiteiten "}<b>{"analyseren, adviseren, ontwerpen, realiseren"}</b>{" en "}<b>{"manage & control"}</b>{". Deze activiteiten worden in samenhang uitgevoerd."}</p>
								<p>{"Binnen je beroepsrol werk je aan een specifieke "}<b>{"hoofdlaag"}</b>{". Daarnaast werk je aan één of meer "}<b>{"extra lagen"}</b>{" om je te verbreden of te verdiepen. Zo ontstaat een eigen kleuring van je ICT-profiel, met een gezamenlijke basis als uitgangspunt."}</p>
								</div>
							</div>
						</section>
					</div>
					<div>
						<section>
							<h3>{"Roadmap"}</h3>
							<ul>
								{#for resource in resources.iter()}
									<li><a href={&resource.url} target="_blank">{&resource.text}</a></li>
								{/for}
							</ul>
							<h4>{"Taakgericht"}</h4>
							<p>{level_one}</p>
							<h4>{"Probleemgericht"}</h4>
							<p>{level_two}</p>
							<h4>{"Situatiegericht"}</h4>
							<p>{level_three}</p>

						</section>
					</div>
				</div>
			</Card>
			<Card>
				<h3>{"Mogelijke functies binnen de rol {}", name_lower}</h3>
				<hr/>
				<div class={"beroepsfuncties"}>
					{#for (key, value) in example_jobs.into_iter()}
						<section>
							<h4>{key}</h4>
							<p>{value}</p>
						</section>
					{/for}
				</div>
			</Card>
		}
	}
}