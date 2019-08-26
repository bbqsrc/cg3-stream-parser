use lazy_static::lazy_static;
use regex::Regex;
use serde_derive::Serialize;

lazy_static! {
    static ref RE_WORD_FORM: Regex = Regex::new(r#"^"<(.*?)>"(?:\s(.*)?)?$"#).unwrap();
    static ref RE_BASE_FORM: Regex = Regex::new(r#"^\s+"(.*?)"(?:\s(.*)?)?$"#).unwrap();
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

    pub fn to_string(&self) -> String {
        let mut s = String::new();
        s.push_str("\"<");
        s.push_str(self.word_form);
        s.push_str(">\"");
        self.tags.iter().for_each(|t| {
            s.push_str(" ");
            s.push_str(t);
        });
        s.push_str("\n");
        self.readings.iter().for_each(|r| {
            s.push_str("    \"");
            s.push_str(r.base_form);
            s.push_str("\"");
            r.tags.iter().for_each(|t| {
                s.push_str(" ");
                s.push_str(t);
            });
            s.push_str("\n");
        });
        s
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

pub fn to_cg3_string<'s>(input: &[Cohort<'s>]) -> String {
    input.iter().map(Cohort::to_string).collect::<Vec<_>>().join("")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic() {
        let stream = r#"
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
        "#;

        let foo = from_string(stream);
        println!("{}", serde_json::to_string_pretty(&foo).unwrap());
    }

    #[test]
    fn basic2() {
        let stream = r#"
"<same>"
    "sáve" N <NomGenSg> Sem/Dummytag Sg Nom <W:21.3018> <WA:15.3018> <spelled> "<sáve>" @SUBJ> &SUGGESTWF &typo
    "sále" N <NomGenSg> Sem/Build-part Sg Nom <W:21.3018> <WA:15.3018> <spelled> "<sále>" @SUBJ> &SUGGESTWF &typo
"<">"
    """ PUNCT <W:0.0>
"<>>"
    ">" PUNCT LEFT <W:0.0>
:
"<hello>"
    "heallat" Ex/V Ex/IV Der/PassS <mv> V <0> IV Ind Prs Sg3 <W:0.0> @+FMAINV
"<.>"
    "." CLB <W:0.0> <NoSpaceAfterPunctMark>
        "#;

        let foo = from_string(stream);
        println!("{}", serde_json::to_string_pretty(&foo).unwrap());
    }


    #[test]
    fn pathological() {
        let stream = r#"
"<">"
    """ PUNCT <W:0.0>
"<">"
    """ PUNCT <W:0.0>
"<>>"
    ">" PUNCT LEFT <W:0.0>
"<">"
    """ PUNCT <W:0.0>
"<<>"
    "<" PUNCT LEFT <W:0.0>
"<>>"
    ">" PUNCT LEFT <W:0.0>
:
"<<>"
    "<" PUNCT LEFT <W:0.0>
"<">"
    """ PUNCT <W:0.0>
"<<>"
    "<" PUNCT LEFT <W:0.0> <spaceAfterParenBeg> &space-after-paren-beg &space-before-paren-end &LINK ID:9 R:RIGHT:10
    "<" PUNCT LEFT <W:0.0> <spaceAfterParenBeg> "<<>>" &space-after-paren-beg &SUGGESTWF ID:9 R:RIGHT:10
:
"<>>"
    ">" PUNCT LEFT <W:0.0> <spaceBeforeParenEnd> &LINK &space-after-paren-beg &space-before-paren-end ID:10 R:LEFT:9
"<">"
    """ PUNCT <W:0.0>
"<>>"
    ">" PUNCT LEFT <W:0.0>
:
"<<>"
    "<" PUNCT LEFT <W:0.0>
"<">"
    """ PUNCT <W:0.0>
"<>>"
    ">" PUNCT LEFT <W:0.0>
        "#;

        let foo = from_string(stream);
        println!("{}", serde_json::to_string_pretty(&foo).unwrap());
    }

    #[test]
    fn idempotent() {
        let stream = r#""<same>"
    "sáve" N <NomGenSg> Sem/Dummytag Sg Nom <W:21.3018> <WA:15.3018> <spelled> "<sáve>" @SUBJ> &SUGGESTWF &typo
    "sále" N <NomGenSg> Sem/Build-part Sg Nom <W:21.3018> <WA:15.3018> <spelled> "<sále>" @SUBJ> &SUGGESTWF &typo
"<">"
    """ PUNCT <W:0.0>
"<>>"
    ">" PUNCT LEFT <W:0.0>
"<hello>"
    "heallat" Ex/V Ex/IV Der/PassS <mv> V <0> IV Ind Prs Sg3 <W:0.0> @+FMAINV
"<.>"
    "." CLB <W:0.0> <NoSpaceAfterPunctMark>
"#;
        let foo = from_string(stream);
        println!("{}", serde_json::to_string_pretty(&foo).unwrap());
        let s = to_cg3_string(&foo);
        println!("{}", s);
        assert_eq!(stream, &s);
    }
}