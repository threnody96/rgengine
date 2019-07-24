use std::rc::Rc;
use std::borrow::Borrow;
use ::resource::audio::AudioPlayer;
use ::resource::controller::Input;
use ::resource::material::Material;
use ::resource::state::Variable;

pub struct Resource {
    audio: Rc<AudioPlayer>,
    input: Rc<Input>,
    material: Rc<Material>,
    variable: Rc<Variable>,
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

    pub fn audio(&self) -> &AudioPlayer { self.audio.borrow() }

    pub fn input(&self) -> &Input { self.input.borrow() }

    pub fn material(&self) -> &Material { self.material.borrow() }

    pub fn variable(&self) -> &Variable { self.variable.borrow() }

}
