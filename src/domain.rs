use std::collections::{BTreeMap, HashMap};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone, Eq, PartialEq, Hash, PartialOrd)]
#[derive(Ord, FromFormField)]
pub enum Skill {
    #[serde(rename(serialize = "Overzicht creëren", deserialize = "Overzicht creëren"))]
    #[field(value = "Overzicht creëren")]
    Overzicht_creëren,

    #[serde(rename(serialize = "Kritisch oordelen", deserialize = "Kritisch oordelen"))]
    #[field(value = "Kritisch oordelen")]
    Kritisch_oordelen,

    #[serde(rename(
        serialize = "Juiste kennis ontwikkelen",
        deserialize = "Juiste kennis ontwikkelen"
    ))]
    Juiste_kennis_ontwikkelen,

    #[serde(rename(
        serialize = "Kwalitatief product maken",
        deserialize = "Kwalitatief product maken"
    ))]
    #[field(value = "Kwalitatief product maken")]
    Kwalitatief_product_maken,

    #[serde(rename(serialize = "Plannen", deserialize = "Plannen"))]
    #[field(value = "Plannen")]
    Plannen,

    #[serde(rename(serialize = "Boodschap delen", deserialize = "Boodschap delen"))]
    #[field(value = "Boodschap delen")]
    Boodschap_delen,

    #[serde(rename(serialize = "Samenwerken", deserialize = "Samenwerken"))]
    #[field(value = "Samenwerken")]
    Samenwerken,

    #[serde(rename(serialize = "Flexibel opstellen", deserialize = "Flexibel opstellen"))]
    #[field(value = "Flexibel opstellen")]
    Flexibel_opstellen,

    #[serde(rename(serialize = "Pro-actief handelen", deserialize = "Pro-actief handelen"))]
    #[field(value = "Pro-actief handelen")]
    #[field(value = "Proactief handelen")]
    Proactief_handelen,

    #[serde(rename(serialize = "Reflecteren", deserialize = "Reflecteren"))]
    #[field(value = "Reflecteren")]
    Reflecteren,
}

impl Skill {
    pub fn to_icon(&self) -> &str {
        match &self {
            Skill::Overzicht_creëren => { "explore" }
            Skill::Kritisch_oordelen => {"announcement"}
            Skill::Juiste_kennis_ontwikkelen => {"menu_book"}
            Skill::Kwalitatief_product_maken => {"biotech"}
            Skill::Plannen => {"calendar_month"}
            Skill::Boodschap_delen => {"co_present"}
            Skill::Samenwerken => {"handshake"}
            Skill::Flexibel_opstellen => {"accessibility_new"}
            Skill::Proactief_handelen => {"directions_run"}
            Skill::Reflecteren => {"psychology"}
        }
    }

    pub fn to_text(&self) -> &str {
        match &self {
            Skill::Overzicht_creëren => {"Overzicht creëren"}
            Skill::Kritisch_oordelen => {"Kritisch oordelen"}
            Skill::Juiste_kennis_ontwikkelen => {"Juiste kennisontwikkelen"}
            Skill::Kwalitatief_product_maken => {"Kwalitatief product maken"}
            Skill::Plannen => {"Plannen"}
            Skill::Boodschap_delen => {"Boodschap delen"}
            Skill::Samenwerken => {"Samenwerken"}
            Skill::Flexibel_opstellen => {"Flexibel opstellen"}
            Skill::Proactief_handelen => {"Pro-actief handelen"}
            Skill::Reflecteren => {"Reflecteren"}
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, Eq, PartialEq, Hash, PartialOrd)]
#[derive(Ord)]
pub enum Level {
    #[serde(rename(serialize = "1", deserialize = "1"))]
    Level1,

    #[serde(rename(serialize = "2", deserialize = "2"))]
    Level2,

    #[serde(rename(serialize = "3", deserialize = "3"))]
    Level3,

    #[serde(rename(serialize = "4", deserialize = "4"))]
    Level4,
}

#[derive(Debug, Serialize, Deserialize, Clone, Eq, PartialEq, Hash, FromFormField)]
pub enum Activiteit {
    #[serde(rename(serialize = "Analyseren", deserialize = "Analyseren"))]
    Analyseren,

    #[serde(rename(serialize = "Adviseren", deserialize = "Adviseren"))]
    Adviseren,

    #[serde(rename(serialize = "Ontwerpen", deserialize = "Ontwerpen"))]
    Ontwerpen,

    #[serde(rename(serialize = "Realiseren", deserialize = "Realiseren"))]
    Realiseren,

    #[serde(rename(serialize = "Manage & Control", deserialize = "Manage & Control"))]
    #[field(value = "Manage & Control")]
    #[field(value = "Manage %26 Control")]
    #[field(value = "Manage_and_Control")]
    Manage_and_Control,
}

impl Activiteit {
    pub fn to_icon(&self) -> &str {
        match &self {
            Activiteit::Analyseren => {"biotech"}
            Activiteit::Adviseren => {"chat"}
            Activiteit::Ontwerpen => {"design_services"}
            Activiteit::Realiseren => {"build"}
            Activiteit::Manage_and_Control => {"tune"}
        }
    }
    
    pub fn to_text(&self) -> &str {
        match &self {
            Activiteit::Manage_and_Control => {"Manage & Control"}
            Activiteit::Analyseren => {"Analyseren"}
            Activiteit::Adviseren => {"Adviseren"}
            Activiteit::Ontwerpen => {"Ontwerpen"}
            Activiteit::Realiseren => {"Realiseren"}
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, Eq, PartialEq, Hash, FromFormField)]

pub enum Architectuurlaag {
    #[serde(rename(
        serialize = "Gebruikersinteractie",
        deserialize = "Gebruikersinteractie"
    ))]
    Gebruikersinteractie,

    #[serde(rename(
        serialize = "Organisatieprocessen",
        deserialize = "Organisatieprocessen"
    ))]
    Organisatieprocessen,

    #[serde(rename(serialize = "Infrastructuur", deserialize = "Infrastructuur"))]
    Infrastructuur,

    #[serde(rename(serialize = "Software", deserialize = "Software"))]
    Software,

    #[serde(rename(serialize = "Hardwareinterfacing", deserialize = "Hardwareinterfacing"))]

    Hardwareinterfacing,
}

impl Architectuurlaag {
    pub fn to_icon(&self) -> &str {
        match &self {
            Architectuurlaag::Gebruikersinteractie => {"smart_button"}
            Architectuurlaag::Organisatieprocessen => {"domain"}
            Architectuurlaag::Infrastructuur => {"storage"}
            Architectuurlaag::Software => {"laptop"}
            Architectuurlaag::Hardwareinterfacing => {"memory"}
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, Eq, PartialEq, Hash)]

pub enum Guild {
    #[serde(rename(serialize = "AI", deserialize = "AI"))]

    ArtificialIntelligence,

    #[serde(rename(serialize = "BE", deserialize = "BE"))]
    Backend,

    #[serde(rename(serialize = "BIM", deserialize = "BIM"))]
   
    BusinessItManagement,

    #[serde(rename(serialize = "CSC", deserialize = "CSC"))]
    
    CyberSecurityAndCloud,

    #[serde(rename(serialize = "FE", deserialize = "FE"))]
    
    Frontend,

    #[serde(rename(serialize = "UI/UX", deserialize = "UI/UX"))]
    
    UIUX,

    #[serde(rename(serialize = "TI", deserialize = "TI"))]
    Embedded,

    #[serde(rename(serialize = "GD", deserialize = "GD"))]
    GameDevelopment,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct LevelDescription {
    pub title: String,
    pub info: Option<String>,
}

#[derive(Debug, serde::Deserialize, serde::Serialize, Clone)]
pub struct HBOIExampleResponse {
    pub id: String,
    pub architectureLayerId: Architectuurlaag,
    pub activityId: Activiteit,
    pub guild: Guild,
    pub title: String,
}

pub type VaardighedenResponseBody = BTreeMap<Skill, BTreeMap<Level, LevelDescription>>;
pub type HBOIResponseBody = BTreeMap<String, BTreeMap<Level, LevelDescription>>;