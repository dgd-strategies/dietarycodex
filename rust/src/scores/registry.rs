use super::{ahei::Ahei, amed::AMedScorer, dash::DashScorer, hei::HeiScorer, DietScore};

pub fn all_scorers() -> Vec<Box<dyn DietScore>> {
    vec![
        Box::new(Ahei),
        Box::new(HeiScorer),
        Box::new(DashScorer),
        Box::new(AMedScorer),
    ]
}
