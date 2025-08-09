use std::collections::BTreeMap;
use tidos::{scoped_css, view, Component, Page};
use crate::components::card::Card;
use crate::components::navigation::skill_filter_matrix::SkillFilterMatrix;
use crate::domain::{Level, LevelDescription, Skill, VaardighedenResponseBody};

pub struct SkillContent {
    pub filter: Option<Skill>
}

impl Component for SkillContent {
    fn to_render(&self, page: &mut Page) -> String {
        let content = include_str!("../../../app/data/vaardigheden-nl.json");
        let content = serde_json::from_str::<VaardighedenResponseBody>(content).unwrap();
        tidos::head! {
            {#for (skill, _) in content.iter()}
                <link rel="prefetch" href={format!("/?vaardigheid={skill:#?}")} />
            {/for}
        }
        
        view! {
            <Card content={
                view!{ <SkillFilterMatrix filter={&self.filter} /> }
            } />
            {#for (skill, levels) in content
                .iter()
                .filter(|(s, _)| self.filter.as_ref().map_or(true, |f| f == *s))
            }
                <Card content={
                    view!{ <Description title={skill.to_text().to_string()} levels={levels} />}
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