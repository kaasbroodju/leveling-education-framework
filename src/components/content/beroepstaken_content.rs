use std::collections::BTreeMap;
use tidos::{scoped_css, view, Component, Page};
use crate::components::card::Card;
use crate::components::navigation::beroepstaken_filter_matrix::BeroepstakenFilterMatrix;
use crate::components::navigation::skill_filter_matrix::SkillFilterMatrix;
use crate::domain::{Activiteit, Architectuurlaag, HBOIResponseBody, Level, LevelDescription, Skill, VaardighedenResponseBody};

pub struct BeroepstakenContent {
    pub architectuurlaag: Option<Architectuurlaag>,
    pub activiteit: Option<Activiteit>
}

impl Component for BeroepstakenContent {
    fn to_render(&self, page: &mut Page) -> String {
        let content = include_str!("../../../app/data/hboi-nl.json");
        let content = serde_json::from_str::<HBOIResponseBody>(content).unwrap();
        tidos::head! {
            {#for (skill, _) in content.iter()}
                <link rel="prefetch" href={format!("/?vaardigheid={skill:#?}")} />
            {/for}
        }

        println!("{:?}", (&self.architectuurlaag, &self.activiteit));
        
        let key = format!(
            "{} {}",
            self.architectuurlaag.as_ref().map_or(String::new(), |x| format!("{x:#?}")),
            self.activiteit.as_ref().map_or("", |x| x.to_text())
        );
        
        println!("{key}");
        
        view! {
            <BeroepstakenFilterMatrix architectuurlaag={&self.architectuurlaag} activiteit={&self.activiteit} />
            {#for (skill, levels) in content
                .iter()
                .filter(|(s, _)| s.matches(&key).collect::<Vec<_>>().len() > 0)
            }
                <Card content={
                    view!{ <Description title={skill.to_owned()} levels={levels} />}
                } />
            {/for}
        }
    }
}

struct Description<'a> {
    pub levels: &'a BTreeMap<Level, LevelDescription>,
    pub title: String,
}

impl Component for Description<'_> {
    fn to_render(&self, page: &mut Page) -> String {
        view!{
            <section class={scoped_css!("skill_content.css")}>
                <h2>{format!("{}", self.title.replace('_', " "))}</h2>
                <hr/>
                <div>
                    {#for (level, description) in self.levels.iter()}
                        <section>
                            <h3>{format!("{:?}", level)}</h3>
                            <p>{&description.title}</p>
                            // @html{markdown::to_html(&description.title)}
                        </section>
                    {/for}
                </div>
            </section>
        }
    }
}