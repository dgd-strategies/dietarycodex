use super::{
    acs2020::Acs2020Scorer, ahei::Ahei, amed::AMedScorer, dash::DashScorer,
    dashi::DashiScorer, dii::DiiScorer, hei::HeiScorer, mind::MindScorer,
    phdi::PhdiScorer, DietScore,
};

pub fn all_scorers() -> Vec<Box<dyn DietScore>> {
    vec![
        Box::new(Ahei),
        Box::new(HeiScorer),
        Box::new(DashScorer),
        Box::new(DashiScorer),
        Box::new(AMedScorer),
        Box::new(DiiScorer),
        Box::new(PhdiScorer),
        Box::new(Acs2020Scorer),
        Box::new(MindScorer),
    ]
}
