use std::path::{Path, PathBuf};
use rocket::fs::{FileServer, NamedFile};
use rocket::http::Header;
use rocket::response::Responder;
use tidos::Component;
mod components;
mod domain;

#[macro_use] extern crate rocket;

use tidos::{page, view, Page};
use crate::components::card::Card;
use crate::components::content::beroepstaken_content::BeroepstakenContent;
use crate::components::content::skill_content::SkillContent;
use crate::components::layout::Layout;
use crate::domain::{Activiteit, Architectuurlaag, Skill};

#[get("/?<vaardigheid>")]
fn index(vaardigheid: Option<Skill>) -> Page {
    page! {
        {
            tidos::head! {<title>{"LEF - Vaardigheden"}</title>}
            ""
        }
        <Layout content={view! {<SkillContent filter={vaardigheid} /> }} />
    }
}

#[get("/beroepstaken?<architectuurlaag>&<activiteit>")]
fn beroepstaken(architectuurlaag: Option<Architectuurlaag>, activiteit: Option<Activiteit>) -> Page {
    
    page! {
        {
            tidos::head! {<title>{"LEF - Beroepstaken"}</title>}
            ""
        }
        <Layout content={view! {<BeroepstakenContent architectuurlaag={architectuurlaag} activiteit={activiteit} /> }} />
    }
}

#[catch(404)]
fn index_not_found() -> Page {
    page! {
        {
            tidos::head! {<title>{"LEF - Vaardigheden"}</title>}
            ""
        }
        <Layout content={view! {<h1>{"Page not found"}</h1>}} />
    }
}

pub struct CachedFile(NamedFile);

impl<'r> Responder<'r, 'static> for CachedFile {
    fn respond_to(self, req: &'r rocket::Request<'_>) -> rocket::response::Result<'static> {
        let mut response = self.0.respond_to(req)?;

        // Add cache headers
        response.set_header(Header::new("Cache-Control", "public, max-age=3600"));
        Ok(response)
    }
}

#[get("/<file..>")]
async fn files(file: PathBuf) -> Option<CachedFile> {
    NamedFile::open(Path::new("./app/public/").join(file))
        .await
        .ok()
        .map(CachedFile)
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .register("/", catchers![index_not_found])
        .mount("/", routes![index, beroepstaken, files])
        // .mount("/", FileServer::from("./app/public"))
}
