use std::path::{Path, PathBuf};
use chrono::{DateTime, Utc};
use rocket::fs::{FileServer, NamedFile};
use rocket::http::{Header, Status};
use rocket::{Request, Response};
use rocket::response::{status, Responder};
use tidos::Component;
mod components;
mod domain;
mod data;

#[macro_use] extern crate rocket;

use tidos::{page, view, Page};
use crate::components::card::Card;
use crate::components::content::about_lef::AboutLef;
use crate::components::content::beroepsproducten_content::BeroepsproductenContent;
use crate::components::content::beroepstaken_content::BeroepstakenContent;
use crate::components::content::skill_content::SkillContent;
use crate::components::layout::Layout;
use crate::domain::{Activiteit, Architectuurlaag, HBOIExampleResponse, HBOIResponseBody, Skill, VaardighedenResponseBody};

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
            cache_control: Header::new("Cache-Control", "public, max-age=300, stale-while-revalidate=86400"),
        }
    }
}

#[get("/robots.txt")]
fn robots() -> &'static str {
    include_str!("../robots.txt")
}

#[get("/llms.txt")]
fn llms() -> &'static str {
    include_str!("../STUDIEWIJZER_FOR_AI.md")
}

#[get("/?<vaardigheid>")]
fn index(vaardigheid: Option<Skill>) -> CachedHtml {
    page! {
        {
            tidos::head! {<title>{"LEF - Vaardigheden"}</title>}
            ""
        }
        <Layout current_url="/" content={view! {<SkillContent filter={vaardigheid} /> }} />
    }.into()
}

#[get("/beroepstaken?<architectuurlaag>&<activiteit>")]
fn beroepstaken(architectuurlaag: Option<Architectuurlaag>, activiteit: Option<Activiteit>) -> CachedHtml {
    page! {
        {
            tidos::head! {<title>{"LEF - Beroepstaken"}</title>}
            ""
        }
        <Layout current_url="/beroepstaken" content={view! {<BeroepstakenContent architectuurlaag={architectuurlaag} activiteit={activiteit} /> }} />
    }.into()
}

#[get("/beroepsproducten?<architectuurlaag>&<activiteit>")]
fn beroepsproducten(architectuurlaag: Option<Architectuurlaag>, activiteit: Option<Activiteit>) -> CachedHtml {
    page! {
        {
            tidos::head! {<title>{"LEF - Beroepsproducten"}</title>}
            ""
        }
        <Layout current_url="/beroepsproducten" content={view! {<BeroepsproductenContent architectuurlaag={architectuurlaag} activiteit={activiteit} /> }} />
    }.into()
}

#[get("/about")]
fn about() -> CachedHtml {
    page! {
        {
            tidos::head! {<title>{"LEF - Leveling Education Framework"}</title>}
            ""
        }
        <Layout current_url="/about" content={view! {<AboutLef  /> }} />
    }.into()
}

#[catch(404)]
fn index_not_found() -> Page {
    page! {
        {
            tidos::head! {<title>{"LEF - Vaardigheden"}</title>}
            ""
        }
        <Layout current_url="" content={view! {<h1>{"Page not found"}</h1>}} />
    }
}

// use rocket::http::{Header, Status};
use rocket::response::{Result as ResponseResult};
use rocket::response::status::Accepted;
use rocket::serde::json::Json;
use crate::data::{EXAMPLES_DATA, HBOI_DATA, SKILL_DATA};
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
            if let Ok(client_time) = DateTime::parse_from_rfc2822(if_modified_since)
                .or_else(|_| DateTime::parse_from_str(if_modified_since, "%a, %d %b %Y %H:%M:%S GMT")) {

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
        response.set_header(Header::new("Last-Modified",
                                        self.last_modified.format("%a, %d %b %Y %H:%M:%S GMT").to_string()));

        Ok(response)
    }
}

#[get("/<file..>")]
async fn files(file: PathBuf) -> Option<CachedFile> {
    let path = Path::new("./app/public/").join(file);
    CachedFile::new(path).await.ok()
}

#[get("/vaardigheden")]
async fn vaardighedenApi() -> Json<VaardighedenResponseBody> {
    Json((*SKILL_DATA).clone())
}

#[get("/hboi")]
async fn beroepstakenApi() -> Json<HBOIResponseBody> {
    Json((*HBOI_DATA).clone())
}

#[get("/beroepsproducten")]
async fn beroepsproductenApi() -> Json<Vec<HBOIExampleResponse>> {
    Json((*EXAMPLES_DATA).clone())
}



#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/api/v1", routes![vaardighedenApi, beroepstakenApi, beroepsproductenApi])
        .register("/", catchers![index_not_found])
        .mount("/", routes![index, beroepstaken, beroepsproducten, about, files, robots, llms])
        // .mount("/", FileServer::from("./app/public"))
}
