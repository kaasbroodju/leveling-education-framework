use tidos::{scoped_css, view, Component, Page};
use crate::components::card::Card;
use crate::domain::{Activiteit, Architectuurlaag, Skill};
use crate::domain::Activiteit::{Adviseren, Analyseren, Manage_and_Control, Ontwerpen, Realiseren};
use crate::domain::Architectuurlaag::{Gebruikersinteractie, Hardwareinterfacing, Infrastructuur, Organisatieprocessen, Software};

pub struct BeroepstakenFilterMatrix<'a> {
    pub architectuurlaag: &'a Option<Architectuurlaag>,
    pub activiteit: &'a Option<Activiteit>
}

const ARCHITECTUURLAGEN: [Architectuurlaag; 5] = [Gebruikersinteractie, Organisatieprocessen, Infrastructuur, Software, Hardwareinterfacing];
const ACTIVITEITEN: [Activiteit; 5] = [Analyseren, Adviseren, Ontwerpen, Realiseren, Manage_and_Control];
impl<'a> Component for BeroepstakenFilterMatrix<'a> {
    fn to_render(&self, page: &mut Page) -> String {
        tidos::head! {
            <link rel="stylesheet" href="https://fonts.googleapis.com/css2?family=Material+Symbols+Outlined:opsz,wght,FILL,GRAD@48,400,1,0" />
            <link rel="preconnect" href="https://fonts.googleapis.com" />
            <link rel="preconnect" href="https://fonts.gstatic.com" crossorigin />
            <link href="https://fonts.googleapis.com/css2?family=Roboto:ital,wght@0,100..900;1,100..900&family=Ubuntu:ital,wght@0,300;0,400;0,500;0,700;1,300;1,400;1,500;1,700&display=swap" rel="stylesheet" />
        }
        let css = scoped_css!("beroepstaken_filter_matrix.css");
        view! {
            <header>
                <Card content={view! {
                    <div class={css}>
                        {#for x in ARCHITECTUURLAGEN}
                            <a href={state_dependent_link((&self.architectuurlaag, &self.activiteit), (Some(&x), None))} aria-label={format!("{x:#?}")}>
                                <span class="material-symbols-outlined" style="font-size: 48px;">{x.to_icon()}</span>
                                <span>{format!("{x:#?}")}</span>
                            </a>
                        {/for}
                    </div>
                }} />
                
                <Card content={view! {
                    <div class={css}>
                        {#for x in ACTIVITEITEN}
                            <a href={state_dependent_link((&self.architectuurlaag, &self.activiteit), (None, Some(&x)))} aria-label={x.to_text()}>
                                <span class="material-symbols-outlined" style="font-size: 48px;">{x.to_icon()}</span>
                                <span>{x.to_text()}</span>
                            </a>
                        {/for}
                    </div>
                }} />
            </header>
        }
    }
}

fn state_dependent_link((state_laag, state_activiteit): (&Option<Architectuurlaag>, &Option<Activiteit>), (value_laag, value_activiteit): (Option<&Architectuurlaag>, Option<&Activiteit>)) -> String {
    match (state_laag, state_activiteit, value_laag, value_activiteit) {
        // nothing is selected
        (None, None, Some(value_laag), _) => { format!("/beroepstaken?architectuurlaag={value_laag:#?}") }
        (None, None, _, Some(value_activiteit)) => { format!("/beroepstaken?activiteit={value_activiteit:#?}") }
        
        // one of them is already selected
        (None, Some(state_activiteit), _, Some(value_activiteit)) => {if !state_activiteit.eq(&value_activiteit) { format!("/beroepstaken?activiteit={value_activiteit:#?}") } else { format!("/beroepstaken") } }
        (Some(state_laag), None, Some(value_laag), _) => {if !state_laag.eq(&value_laag) { format!("/beroepstaken?architectuurlaag={value_laag:#?}") } else { format!("/beroepstaken") } }
        
        // the other one is selected
        (None, Some(state_activiteit), Some(value_laag), _) => { format!("/beroepstaken?architectuurlaag={value_laag:#?}&activiteit={state_activiteit:#?}") }
        (Some(state_laag), None, _, Some(value_activiteit)) => { format!("/beroepstaken?architectuurlaag={state_laag:#?}&activiteit={value_activiteit:#?}") }
        
        // Both are already selected
        (Some(state_laag), Some(state_activiteit), Some(value_laag), _) => {
            if !state_laag.eq(&value_laag) {
                format!("/beroepstaken?architectuurlaag={value_laag:#?}&activiteit={state_activiteit:#?}")
            } else {
                format!("/beroepstaken?activiteit={state_activiteit:#?}")
            } 
        }
        (Some(state_laag), Some(state_activiteit), _, Some(value_activiteit)) => {
            if !state_activiteit.eq(&value_activiteit) {
                format!("/beroepstaken?architectuurlaag={state_laag:#?}&activiteit={value_activiteit:#?}")
            } else {
                format!("/beroepstaken?architectuurlaag={state_laag:#?}")
            }
        }
        _ => {format!("/beroepstaken")}
    }
}