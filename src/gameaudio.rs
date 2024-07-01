use kira::{
    manager::{backend::DefaultBackend, AudioManager, AudioManagerSettings},
    sound::{self, static_sound::StaticSoundData, PlaybackState},
    tween::Tween,
};
pub struct GameAudio {
    pub active: bool,
    manager: AudioManager<DefaultBackend>,
    exhaust: StaticSoundData,
    won: StaticSoundData,
    lost: StaticSoundData,
    current_sound: Option<kira::sound::static_sound::StaticSoundHandle>,
    game_over_playing: bool,
}
impl GameAudio {
    pub fn new() -> Self {
        let manager = AudioManager::<DefaultBackend>::new(AudioManagerSettings::default()).unwrap();
        let exhaust = StaticSoundData::from_file("src/sounds/rocket_exhaust.wav").unwrap();
        let won = StaticSoundData::from_file("src/sounds/won.wav").unwrap();
        let lost = StaticSoundData::from_file("src/sounds/lost.wav").unwrap();

        GameAudio {
            manager,
            exhaust: exhaust,
            won: won,
            lost: lost,
            current_sound: None,
            game_over_playing: false,
            active: true,
        }
    }

    pub fn exhaust(&mut self) {
        if (!self.active) {
            return;
        }
        match &self.current_sound {
            Some(sound) => {
                if sound.state() != PlaybackState::Playing {
                    self.current_sound = Some(self.play_sound(self.exhaust.clone()));
                }
            }
            None => self.current_sound = Some(self.play_sound(self.exhaust.clone())),
        }
    }

    pub fn won(&mut self) {
        if (!self.active) {
            return;
        }
        if self.game_over_playing {
            return;
        }

        self.stop_current();
        self.game_over_playing = true;
        self.current_sound = Some(self.play_sound(self.won.clone()))
    }

    pub fn lost(&mut self) {
        if (!self.active) {
            return;
        }

        if self.game_over_playing {
            return;
        }

        self.stop_current();
        self.game_over_playing = true;
        self.current_sound = Some(self.play_sound(self.lost.clone()))
    }

    fn stop_current(&mut self) {
        if self.current_sound.is_some() {
            let sound = self.current_sound.as_mut().unwrap();
            if sound.state() == PlaybackState::Playing {
                sound.stop(Tween::default());
            }
        }
    }
    fn play_sound(
        &mut self,
        sound: StaticSoundData,
    ) -> kira::sound::static_sound::StaticSoundHandle {
        self.manager.play(sound).unwrap()
    }

    pub fn reset(&mut self) {
        self.game_over_playing = false;
    }
}
