// #![recursion_limit = "256"]

use chrono::{DateTime, Utc};
use rocket::fs::{FileServer, NamedFile};
use rocket::http::{Header, Status};
use rocket::response::{Responder, status};
use rocket::{Request, Response};
use std::path::{Path, PathBuf};
mod components;
mod data;
mod domain;

#[macro_use]
extern crate rocket;

use crate::components::card::Card;
use crate::components::content::about_lef::AboutLef;
use crate::components::content::beroepsproducten_content::BeroepsproductenContent;
use crate::components::content::beroepstaken_content::BeroepstakenContent;
use crate::components::content::skill_content::SkillContent;
use crate::components::layout::Layout;
use crate::domain::{BeroepsRollenResponseBody, DeprecatedVaardighedenResponseBody, HBOIExampleResponse, HBOIResponseBody, Skill, VaardighedenResponseBody};
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
	include_str!("../app/data/STUDIEWIJZER_FOR_AI.md")
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
	tidos::head! {<title>{"LEF - Beroepsproducten"}</title>}
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

// use rocket::http::{Header, Status};
use crate::data::{BEROEPSROLLEN_DATA, DEPRECATED_SKILL_DATA, EXAMPLES_DATA, HBOI_DATA, SKILL_DATA};
use rocket::response::Result as ResponseResult;
use rocket::response::status::Accepted;
use rocket::serde::json::Json;
use crate::components::content::beroepsrollen::BeroepsRollenContent;
// use rocket::{Request, Response};
// use rocket::fs::NamedFile;
// use std::path::{Path, PathBuf};
// use chrono::{DateTime, Utc};

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

#[get("/vaardigheden")]
async fn vaardigheden_api() -> Json<VaardighedenResponseBody> {
	Json((*SKILL_DATA).clone())
}

#[get("/beroepsrollen")]
async fn beroepsrollen_api() -> Json<BeroepsRollenResponseBody> {
	Json((*BEROEPSROLLEN_DATA).clone())
}

#[get("/hboi")]
async fn beroepstaken_api() -> Json<HBOIResponseBody> {
	Json((*HBOI_DATA).clone())
}

#[get("/beroepsproducten")]
async fn beroepsproducten_api() -> Json<Vec<HBOIExampleResponse>> {
	Json((*EXAMPLES_DATA).clone())
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
		.register("/", catchers![index_not_found])
		.mount(
			"/",
			routes![
				index,
				beroepstaken,
				beroepsproducten,
				beroepsrollen,
				about,
				files,
				robots,
				llms,
				sitemap
			],
		)
	// .mount("/", FileServer::from("./app/public"))
}
