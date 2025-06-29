use super::DietScore;

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
