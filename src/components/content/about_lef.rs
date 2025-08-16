use tidos::{view, Component, Page};
use crate::components::card::Card;

pub struct AboutLef;

impl Component for AboutLef {
    fn to_render(&self, page: &mut Page) -> String {
        view! {
            <Card content={
                view! {
                    <img src="/thankyou.png"/>
                }
            }/>
        }
    }
}

pub struct PersonSection<'a> {
    pub name: &'a str,
    pub image: Option<&'a str>,
    pub linkedIn: Option<&'a str>,
}

impl<'a> Component for PersonSection<'a> {
    fn to_render(&self, page: &mut Page) -> String {
        view! {
            <Card content={
                view! {
                    <img src="/thankyou.png"/>
                }
            }/>
        }
    }
}