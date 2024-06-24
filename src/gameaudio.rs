use kira::{
    manager::{backend::DefaultBackend, AudioManager, AudioManagerSettings},
    sound::{static_sound::StaticSoundData, PlaybackState},
};
pub struct GameAudio {
    manager: AudioManager<DefaultBackend>,
    sound_data: StaticSoundData,
    current_sound: Option<kira::sound::static_sound::StaticSoundHandle>,
}
impl GameAudio {
    pub fn new() -> Self {
        let manager = AudioManager::<DefaultBackend>::new(AudioManagerSettings::default()).unwrap();
        let sound_data = StaticSoundData::from_file("src/rocket_exhaust.wav").unwrap();
        GameAudio {
            manager,
            sound_data,
            current_sound: None,
        }
    }

    pub fn exhaust(&mut self) {
        match &self.current_sound {
            Some(sound) => {
                if sound.state() != PlaybackState::Playing {
                    self.current_sound = Some(self.play_sound());
                }
            }
            None => self.current_sound = Some(self.play_sound()),
        }
    }

    fn play_sound(&mut self) -> kira::sound::static_sound::StaticSoundHandle {
        self.manager.play(self.sound_data.clone()).unwrap()
    }
}
