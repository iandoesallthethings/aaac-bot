use chrono::prelude::*;
use icalendar::parser::read_calendar;
use reqwest::blocking::get;

pub fn get_events(calendar_url: String) -> Vec<IcalEvent> {
    let body = get(calendar_url).unwrap().text().unwrap();

    read_calendar(&body).unwrap().events()
}

#[derive(Debug)]
pub struct IcalEvent {
    pub uid: String,
    pub summary: String,
    pub description: String,
    pub location: String,
    pub categories: Vec<String>,
    pub url: String,
    pub start: chrono::DateTime<Local>,
    pub end: chrono::DateTime<Local>,
}

impl IcalEvent {
    fn from_component(component: &icalendar::parser::Component) -> Self {
        Self {
            uid: component.prop("UID"),
            summary: component.prop("SUMMARY"),
            description: component.prop("DESCRIPTION"),
            location: component.prop("LOCATION"),
            url: component.prop("URL"),
            start: component.date_prop("DTSTART"),
            end: component.date_prop("DTEND"),
            categories: component.vec_prop("CATEGORIES"),
        }
    }
}

pub trait CalendarExtension {
    fn events(&self) -> Vec<IcalEvent>;
}

impl<'a> CalendarExtension for icalendar::parser::Calendar<'a> {
    fn events(&self) -> Vec<IcalEvent> {
        self.components
            .iter()
            .filter(|component| component.name == "VEVENT")
            .map(IcalEvent::from_component)
            .collect()
    }
}

trait IcalComponentExtension {
    fn prop(&self, name: &str) -> String;
    fn date_prop(&self, name: &str) -> chrono::DateTime<Local>;
    fn vec_prop(&self, name: &str) -> Vec<String>;
}

impl<'a> IcalComponentExtension for icalendar::parser::Component<'a> {
    fn prop(&self, name: &str) -> String {
        self.find_prop(name).unwrap().val.to_string()
    }

    fn date_prop(&self, name: &str) -> chrono::DateTime<Local> {
        let value = self.prop(name);

        NaiveDateTime::parse_from_str(&value, "%Y%m%dT%H%M%S")
            .unwrap()
            .and_local_timezone(Local)
            .unwrap()
    }

    fn vec_prop(&self, name: &str) -> Vec<String> {
        self.properties
            .iter()
            .filter(|property| property.name == name)
            .map(|property| property.val.to_string())
            .collect()
    }
}
