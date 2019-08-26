use lazy_static::lazy_static;
use regex::Regex;
use serde_derive::Serialize;

lazy_static! {
    static ref RE_WORD_FORM: Regex = Regex::new(r#"^"<(.*)>"\s*(.*)?$"#).unwrap();
    static ref RE_BASE_FORM: Regex = Regex::new(r#"^\s+"(.*)"\s*(.*)$"#).unwrap();
}

#[derive(Debug, Clone, Serialize)]
pub struct Cohort<'s> {
    word_form: &'s str,
    tags: Vec<&'s str>,
    readings: Vec<Reading<'s>>,
}

impl<'s> Cohort<'s> {
    fn from_captures(captures: regex::Captures<'s>) -> Option<Cohort<'s>> {
        let word_form = match captures.get(1) {
            Some(v) => v.as_str(),
            None => return None
        };

        let tags = captures.get(2)
            .filter(|x| x.as_str() != "")
            .map(|x| x.as_str().split(" ").collect::<Vec<_>>())
            .unwrap_or(vec![]);

        Some(Cohort {
            word_form,
            tags,
            readings: vec![]
        })
    }
}

#[derive(Debug, Clone, Serialize)]
pub struct Reading<'s> {
    base_form: &'s str,
    tags: Vec<&'s str>,
}

impl<'s> Reading<'s> {
    fn from_captures(captures: regex::Captures<'s>) -> Option<Reading<'s>> {
        let base_form = match captures.get(1) {
            Some(v) => v.as_str(),
            None => return None
        };

        let tags = captures.get(2)
            .filter(|x| x.as_str() != "")
            .map(|x| x.as_str().split(" ").collect::<Vec<_>>())
            .unwrap_or(vec![]);

        Some(Reading {
            base_form,
            tags
        })
    }
}

pub fn from_string<'s>(input: &'s str) -> Vec<Cohort<'s>> {
    input.lines().fold(vec![], |mut state, line| {
        if let Some(captures) = RE_WORD_FORM.captures(line) {
            if let Some(cohort) = Cohort::from_captures(captures) {
                state.push(cohort);
            }
            
            return state;
        }

        let last_cohort = match state.last_mut() {
            Some(v) => v,
            None => return state
        };

        if let Some(captures) = RE_BASE_FORM.captures(line) {
            if let Some(reading) = Reading::from_captures(captures) {
                last_cohort.readings.push(reading);
            }
            
            return state;
        }

        state
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic() {
        let stream = r#"""
some garbage
"<They>" TAG1 TAG2
    "they" <*> PRON PERS NOM PL3 SUBJ
garbage
"<went>"
    "go" V PAST VFIN
"<to>"
    "to" PREP
"<the>"
    "the" DET CENTRAL ART SG/PL
: other garbage
"<zoo>"
    "zoo" N NOM SG
    : almost a thing
"<to>"
    "to" INFMARK>
"<look>"
    "look" V INF
"<at>"
    "at" PREP
"<the>"
    "the" DET CENTRAL ART SG/PL
"<bear>"
    "bear" N NOM SG
"<.>"
        """#;

        let foo = from_string(stream);
        println!("{}", serde_json::to_string(&foo).unwrap());
    }
}