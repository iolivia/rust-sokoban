use audio::SoundSource;
use ggez::{audio, Context};
use specs::{World, WorldExt};
use std::collections::HashMap;

#[derive(Default)]
pub struct AudioStore {
    pub sounds: HashMap<String, audio::Source>,
}

impl AudioStore {
    pub fn play_sound(&mut self, context: &mut Context, sound: &String) {
        let _ = self
            .sounds
            .get_mut(sound)
            .expect("expected sound")
            .play_detached(context);
    }
}

pub fn initialize_sounds(world: &mut World, context: &mut Context) {
    let mut audio_store = world.write_resource::<AudioStore>();
    let sounds = ["correct", "incorrect", "wall"];

    for sound in sounds.iter() {
        let sound_name = sound.to_string();
        let sound_path = format!("/sounds/{}.wav", sound_name);
        let sound_source = audio::Source::new(context, sound_path).expect("expected sound loaded");

        audio_store.sounds.insert(sound_name, sound_source);
    }
}
