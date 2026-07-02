use bevy::prelude::*;
use std::collections::HashMap;

#[derive(Clone, PartialEq, Eq, Hash, Debug, Event)]
pub enum SoundEffect {
    BallBounce,
    MenuHover,
}

impl SoundEffect {
    pub fn get_path(&self) -> &str {
        match self {
            SoundEffect::BallBounce => "sound_effects/bounce.ogg",
            SoundEffect::MenuHover => "sound_effects/pop.ogg",
        }
    }

    const EFFECTS: [SoundEffect; 2] = [SoundEffect::BallBounce, SoundEffect::MenuHover];

    pub fn on_trigger(
        event: On<Self>,
        mut commands: Commands,
        loaded_sound_effects: Res<SoundEffects>,
    ) {
        let sound_handle = loaded_sound_effects.0.get(&event.event()).unwrap();

        commands.spawn((AudioPlayer(sound_handle.clone()), PlaybackSettings::DESPAWN));
    }
}

#[derive(Resource, Clone)]
pub struct SoundEffects(pub HashMap<SoundEffect, Handle<AudioSource>>);

impl FromWorld for SoundEffects {
    fn from_world(world: &mut World) -> Self {
        let mut sound_effects: HashMap<SoundEffect, Handle<AudioSource>> = HashMap::new();

        for effect in SoundEffect::EFFECTS.iter() {
            sound_effects.insert(effect.clone(), world.load_asset(effect.get_path()));
        }

        SoundEffects(sound_effects)
    }
}
