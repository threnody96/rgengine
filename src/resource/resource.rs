use std::rc::Rc;
use ::resource::audio::AudioPlayer;
use ::resource::controller::Input;
use ::resource::material::Material;
use ::resource::state::Variable;

pub struct Resource {
    pub audio: Rc<AudioPlayer>,
    pub input: Rc<Input>,
    pub material: Rc<Material>,
    pub variable: Rc<Variable>,
}

impl Resource {

    pub fn new(material: Material, input: Input) -> Self {
        let m = Rc::new(material);
        Self {
            audio: Rc::new(AudioPlayer::new(m.clone())),
            input: Rc::new(input),
            material: m.clone(),
            variable: Rc::new(Variable::new()),
        }
    }

}
