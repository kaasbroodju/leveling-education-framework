use chrono::{DateTime, Utc};
use rocket::fs::NamedFile;
use rocket::http::{Header, Status};
use rocket::response::{Responder, status};
use rocket::{Request, Response};
use std::path::{Path, PathBuf};
mod components;
mod data;
mod domain;
mod filters;
mod markdown_render;
mod pages;

#[macro_use]
extern crate rocket;

use crate::components::card::Card;
use crate::components::layout::Layout;
use crate::domain::{
	Activiteit, Architectuurlaag, BeroepsRollenResponseBody, DeprecatedVaardighedenResponseBody,
	Guild, HBOIExampleResponse, HBOIResponseBody, Level, Skill, VaardighedenResponseBody,
};
use pages::about_lef::AboutLef;
use pages::beroepsproducten_content::BeroepsproductenContent;
use pages::beroepsrollen::BeroepsRollenContent;
use pages::beroepstaken_content::BeroepstakenContent;
use pages::leeswijzer::LeeswijzerContent;
use pages::skill_content::SkillContent;
use tidos::{Page, page};

#[derive(Responder)]
#[response(status = 200, content_type = "html")]
struct CachedHtml {
	inner: Page,
	cache_control: Header<'static>,
}

impl From<Page> for CachedHtml {
	fn from(value: Page) -> Self {
		CachedHtml {
			inner: value,
			cache_control: Header::new(
				"Cache-Control",
				"public, max-age=300, stale-while-revalidate=86400",
			),
		}
	}
}

#[get("/robots.txt")]
fn robots() -> &'static str {
	include_str!("../app/data/robots.txt")
}

#[get("/sitemap.xml")]
fn sitemap() -> &'static str {
	include_str!("../app/data/sitemap.xml")
}

#[get("/llms.txt")]
fn llms() -> &'static str {
	include_str!("../app/data/llms.txt")
}

#[get("/")]
fn index() -> CachedHtml {
	let mut page = page! {
		<Layout current_url="/">
			{#slot:content}
				<SkillContent />
			{/slot}
		</Layout>
	};
	tidos::head! {<title>{"LEF - Vaardigheden"}</title>}
	page.into()
}

#[get("/beroepstaken")]
fn beroepstaken() -> CachedHtml {
	let mut page = page! {
		<Layout current_url="/beroepstaken">
			{#slot:content}
				<BeroepstakenContent />
			{/slot}
		</Layout>
	};
	tidos::head! {<title>{"LEF - Beroepstaken"}</title>}
	page.into()
}

#[get("/beroepsproducten")]
fn beroepsproducten() -> CachedHtml {
	let mut page = page! {
		<Layout current_url="/beroepsproducten">
			{#slot:content}
				<BeroepsproductenContent />
			{/slot}
		</Layout>
	};
	tidos::head! {<title>{"LEF - Beroepsproducten"}</title>}
	page.into()
}

#[get("/beroepsrollen")]
fn beroepsrollen() -> CachedHtml {
	let mut page = page! {
		<Layout current_url="/beroepsrollen">
			{#slot:content}
				<BeroepsRollenContent />
			{/slot}
		</Layout>
	};
	tidos::head! {<title>{"LEF - Beroepsrollen"}</title>}
	page.into()
}

#[get("/about")]
fn about() -> CachedHtml {
	let mut page = page! {
		<Layout current_url="/about">
			{#slot:content}
				<AboutLef />
			{/slot}
		</Layout>
	};
	tidos::head! {<title>{"LEF - Leveling Education Framework"}</title>}
	page.into()
}

#[get("/leeswijzer")]
fn leeswijzer() -> CachedHtml {
	let mut page = page! {
		<Layout current_url="/leeswijzer">
			{#slot:content}
				<LeeswijzerContent />
			{/slot}
		</Layout>
	};
	tidos::head! {<title>{"LEF - Leeswijzer"}</title>}
	page.into()
}

#[catch(404)]
fn index_not_found() -> Page {
	let mut page = page! {
		<Layout current_url="">
			{#slot:content}
				<h1>{"Page not found"}</h1>
			{/slot}
		</Layout>
	};
	tidos::head! {<title>{"LEF - Vaardigheden"}</title>}
	page
}

use crate::data::{BEROEPSROLLEN_DATA, DEPRECATED_SKILL_DATA, EXAMPLES_DATA, HBOI_DATA, SKILL_DATA};
use rocket::response::Result as ResponseResult;
use rocket::response::status::Accepted;
use rocket::serde::json::Json;

pub struct CachedFile {
	named_file: NamedFile,
	last_modified: DateTime<Utc>,
}

impl CachedFile {
	pub async fn new(path: impl AsRef<Path>) -> std::io::Result<Self> {
		let named_file = NamedFile::open(&path).await?;

		// Haal last modified tijd op en converteer naar UTC
		let metadata = std::fs::metadata(&path)?;
		let modified = metadata.modified()?;
		let last_modified = DateTime::<Utc>::from(modified);

		Ok(CachedFile {
			named_file,
			last_modified,
		})
	}
}

impl<'r> Responder<'r, 'static> for CachedFile {
	fn respond_to(self, req: &'r Request<'_>) -> ResponseResult<'static> {
		// Check If-Modified-Since header
		if let Some(if_modified_since) = req.headers().get_one("If-Modified-Since") {
			if let Ok(client_time) = DateTime::parse_from_rfc2822(if_modified_since).or_else(|_| {
				DateTime::parse_from_str(if_modified_since, "%a, %d %b %Y %H:%M:%S GMT")
			}) {
				let client_utc = client_time.with_timezone(&Utc);

				// Vergelijk op seconde-niveau (HTTP heeft geen sub-seconde precisie)
				if self.last_modified.timestamp() <= client_utc.timestamp() {
					return Response::build()
						.status(Status::NotModified)
						.header(Header::new("Cache-Control", "public, max-age=10"))
						.ok();
				}
			}
		}

		// File is gewijzigd of geen If-Modified-Since header
		let mut response = self.named_file.respond_to(req)?;

		// Voeg cache headers toe
		response.set_header(Header::new("Cache-Control", "public, max-age=360"));
		response.set_header(Header::new(
			"Last-Modified",
			self.last_modified
				.format("%a, %d %b %Y %H:%M:%S GMT")
				.to_string(),
		));

		Ok(response)
	}
}

#[get("/<file..>")]
async fn files(file: PathBuf) -> Option<CachedFile> {
	let path = Path::new("./app/public/").join(file);
	CachedFile::new(path).await.ok()
}

#[derive(Responder)]
struct DeprecatedVaardighedenResponse {
	inner: Json<DeprecatedVaardighedenResponseBody>,
	link: Header<'static>,
	deprecation: Header<'static>,
	sunset: Header<'static>,
}

#[get("/vaardigheden")]
async fn deprecated_vaardigheden_api() -> DeprecatedVaardighedenResponse {
	DeprecatedVaardighedenResponse {
		inner: Json((*DEPRECATED_SKILL_DATA).clone()),
		link: Header::new("Link", r#"</api/v2/vaardigheden>; rel="successor-version""#),
		deprecation: Header::new("Deprecation", "@1788048000"),
		sunset: Header::new("Sunset", "Wed, 30 Sep 2026 00:00:00 GMT"),
	}
}

#[get("/vaardigheden?<vaardigheid>&<niveau>")]
async fn vaardigheden_api(
	vaardigheid: Option<Skill>,
	niveau: Option<Level>,
) -> Json<VaardighedenResponseBody> {
	Json(filters::filter_vaardigheden(&SKILL_DATA, vaardigheid, niveau))
}

#[get("/beroepsrollen?<gilde>")]
async fn beroepsrollen_api(gilde: Option<Guild>) -> Json<BeroepsRollenResponseBody> {
	Json(filters::filter_beroepsrollen(&BEROEPSROLLEN_DATA, gilde))
}

#[get("/hboi?<architectuurlaag>&<activiteit>&<niveau>")]
async fn beroepstaken_api(
	architectuurlaag: Option<Architectuurlaag>,
	activiteit: Option<Activiteit>,
	niveau: Option<Level>,
) -> Json<HBOIResponseBody> {
	Json(filters::filter_hboi(
		&HBOI_DATA,
		architectuurlaag,
		activiteit,
		niveau,
	))
}

#[get("/beroepsproducten?<architectuurlaag>&<activiteit>&<gilde>")]
async fn beroepsproducten_api(
	architectuurlaag: Option<Architectuurlaag>,
	activiteit: Option<Activiteit>,
	gilde: Option<Guild>,
) -> Json<Vec<HBOIExampleResponse>> {
	Json(filters::filter_beroepsproducten(
		&EXAMPLES_DATA,
		architectuurlaag,
		activiteit,
		gilde,
	))
}

#[derive(Responder)]
#[response(content_type = "text/markdown")]
struct Markdown(String);

#[get("/vaardigheden?<vaardigheid>&<niveau>")]
async fn llms_vaardigheden(vaardigheid: Option<Skill>, niveau: Option<Level>) -> Markdown {
	Markdown(markdown_render::vaardigheden_to_markdown(
		&filters::filter_vaardigheden(&SKILL_DATA, vaardigheid, niveau),
	))
}

#[get("/beroepsrollen?<gilde>")]
async fn llms_beroepsrollen(gilde: Option<Guild>) -> Markdown {
	Markdown(markdown_render::beroepsrollen_to_markdown(
		&filters::filter_beroepsrollen(&BEROEPSROLLEN_DATA, gilde),
	))
}

#[get("/hboi?<architectuurlaag>&<activiteit>&<niveau>")]
async fn llms_beroepstaken(
	architectuurlaag: Option<Architectuurlaag>,
	activiteit: Option<Activiteit>,
	niveau: Option<Level>,
) -> Markdown {
	Markdown(markdown_render::hboi_to_markdown(&filters::filter_hboi(
		&HBOI_DATA,
		architectuurlaag,
		activiteit,
		niveau,
	)))
}

#[get("/beroepsproducten?<architectuurlaag>&<activiteit>&<gilde>")]
async fn llms_beroepsproducten(
	architectuurlaag: Option<Architectuurlaag>,
	activiteit: Option<Activiteit>,
	gilde: Option<Guild>,
) -> Markdown {
	Markdown(markdown_render::beroepsproducten_to_markdown(
		&filters::filter_beroepsproducten(&EXAMPLES_DATA, architectuurlaag, activiteit, gilde),
	))
}

#[get("/llms-full.txt")]
fn llms_full() -> &'static str {
	concat!(
		include_str!("../app/data/STUDIEWIJZER_FOR_AI.md"),
		"\n\n",
		include_str!("../app/data/llms-endpoints.md"),
	)
}

#[launch]
fn rocket() -> _ {
	rocket::build()
		.mount(
			"/api/v1",
			routes![deprecated_vaardigheden_api, beroepsrollen_api, beroepstaken_api, beroepsproducten_api],
		)
		.mount(
			"/api/v2",
			routes![vaardigheden_api],
		)
		.mount(
			"/llms",
			routes![
				llms_vaardigheden,
				llms_beroepsrollen,
				llms_beroepstaken,
				llms_beroepsproducten
			],
		)
		.register("/", catchers![index_not_found])
		.mount(
			"/",
			routes![
				index,
				beroepstaken,
				beroepsproducten,
				beroepsrollen,
				about,
				leeswijzer,
				files,
				robots,
				llms,
				llms_full,
				sitemap
			],
		)
}
