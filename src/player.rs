use rodio::{Decoder, DeviceSinkBuilder, MixerDeviceSink, Player, Source};
use std::fs::File;
use std::io::BufReader;
use std::path::Path;

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LoopMode {
    None,
    One,
}

pub struct AudioPlayer {
    _handle: MixerDeviceSink,
    player: Player,
    loop_mode: LoopMode,
}

impl AudioPlayer {
    pub fn new() -> Self {
        let handle = DeviceSinkBuilder::open_default_sink()
            .expect("无法初始化音频输出");
        let player = Player::connect_new(&handle.mixer());
        Self {
            _handle: handle,
            player,
            loop_mode: LoopMode::None,
        }
    }

    pub fn play(&mut self, path: &Path) -> Result<(), String> {
        self.player.stop();

        let file = File::open(path).map_err(|e| format!("无法打开文件: {}", e))?;
        let source = Decoder::try_from(BufReader::new(file))
            .map_err(|e| format!("解码失败: {}", e))?;

        match self.loop_mode {
            LoopMode::One => self.player.append(source.repeat_infinite()),
            LoopMode::None => self.player.append(source),
        }
        self.player.play();
        Ok(())
    }

    pub fn set_loop_mode(&mut self, mode: LoopMode) {
        self.loop_mode = mode;
    }

    pub fn loop_mode(&self) -> LoopMode {
        self.loop_mode
    }

    pub fn toggle_pause(&self) {
        if self.player.is_paused() {
            self.player.play();
        } else {
            self.player.pause();
        }
    }
}