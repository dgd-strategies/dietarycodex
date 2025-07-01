use super::{DietScore, FieldDeps};
use serde::Serialize;

#[macro_export]
macro_rules! register_scores {
    () => {{
        let scores: Vec<Box<dyn $crate::scores::DietScore>> = vec![
            Box::new($crate::scores::ahei::Ahei),
            Box::new($crate::scores::hei::HeiScorer),
            Box::new($crate::scores::dash::DashScorer),
            Box::new($crate::scores::dashi::DashiScorer),
            Box::new($crate::scores::amed::AMedScorer),
            Box::new($crate::scores::dii::DiiScorer),
            Box::new($crate::scores::phdi::PhdiScorer),
            Box::new($crate::scores::acs2020::Acs2020Scorer),
            Box::new($crate::scores::mind::MindScorer),
        ];
        scores
    }};
}

pub fn all_scorers() -> Vec<Box<dyn DietScore>> {
    register_scores!()
}

#[derive(Debug, Serialize, Clone)]
pub struct ScoreMeta {
    pub name: &'static str,
    pub required_fields: &'static [&'static str],
}

pub fn all_score_metadata() -> Vec<ScoreMeta> {
    let mut metas = vec![
        ScoreMeta {
            name: <crate::scores::ahei::Ahei as FieldDeps>::name(),
            required_fields: <crate::scores::ahei::Ahei as FieldDeps>::required_fields(),
        },
        ScoreMeta {
            name: <crate::scores::hei::HeiScorer as FieldDeps>::name(),
            required_fields: <crate::scores::hei::HeiScorer as FieldDeps>::required_fields(),
        },
        ScoreMeta {
            name: <crate::scores::dash::DashScorer as FieldDeps>::name(),
            required_fields: <crate::scores::dash::DashScorer as FieldDeps>::required_fields(),
        },
        ScoreMeta {
            name: <crate::scores::dashi::DashiScorer as FieldDeps>::name(),
            required_fields: <crate::scores::dashi::DashiScorer as FieldDeps>::required_fields(),
        },
        ScoreMeta {
            name: <crate::scores::amed::AMedScorer as FieldDeps>::name(),
            required_fields: <crate::scores::amed::AMedScorer as FieldDeps>::required_fields(),
        },
        ScoreMeta {
            name: <crate::scores::dii::DiiScorer as FieldDeps>::name(),
            required_fields: <crate::scores::dii::DiiScorer as FieldDeps>::required_fields(),
        },
        ScoreMeta {
            name: <crate::scores::phdi::PhdiScorer as FieldDeps>::name(),
            required_fields: <crate::scores::phdi::PhdiScorer as FieldDeps>::required_fields(),
        },
        ScoreMeta {
            name: <crate::scores::acs2020::Acs2020Scorer as FieldDeps>::name(),
            required_fields: <crate::scores::acs2020::Acs2020Scorer as FieldDeps>::required_fields(
            ),
        },
        ScoreMeta {
            name: <crate::scores::mind::MindScorer as FieldDeps>::name(),
            required_fields: <crate::scores::mind::MindScorer as FieldDeps>::required_fields(),
        },
    ];
    metas.sort_by(|a, b| a.name.cmp(b.name));
    metas
}
