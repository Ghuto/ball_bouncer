use bevy::prelude::*;
use std::collections::HashMap;
use std::slice::Iter;

#[derive(Clone, PartialEq, Eq, Hash, Debug)]
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

    fn iterator() -> Iter<'static, SoundEffect> {
        static EFFECTS: [SoundEffect; 2] = [SoundEffect::BallBounce, SoundEffect::MenuHover];
        return EFFECTS.iter();
    }
}

#[derive(Resource, Clone)]
pub struct SoundEffects(pub HashMap<SoundEffect, Handle<AudioSource>>);

pub fn load_sound_effects(mut commands: Commands, asset_server: Res<AssetServer>) {
    let mut sound_effects: HashMap<SoundEffect, Handle<AudioSource>> = HashMap::new();

    for effect in SoundEffect::iterator() {
        sound_effects.insert(effect.clone(), asset_server.load(effect.get_path()));
    }

    commands.insert_resource(SoundEffects(sound_effects));
}

#[derive(Event)]
pub struct PlaySoundEffect(pub SoundEffect);

pub fn on_play_sound(
    event: On<PlaySoundEffect>,
    mut commands: Commands,
    loaded_sound_effects: Res<SoundEffects>,
) {
    let sound_handle = loaded_sound_effects.0.get(&event.0).unwrap();

    commands.spawn((AudioPlayer(sound_handle.clone()), PlaybackSettings::DESPAWN));
}
