use super::{ahei::Ahei, hei::HeiScorer, DietScore};

pub fn all_scorers() -> Vec<Box<dyn DietScore>> {
    vec![Box::new(Ahei), Box::new(HeiScorer)]
}
