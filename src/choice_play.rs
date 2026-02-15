use std::collections::HashMap;

use slint::ModelRc;

#[derive(Default)]
pub struct ChoicePlay {
    pub prev_guesses: HashMap<usize, ModelRc<bool>>,
    pub index_guesses: HashMap<usize, [usize; 4]>,
    pub info_type: usize,
    pub guess_type: usize,
    pub guess_type_is_txt: bool,
}
