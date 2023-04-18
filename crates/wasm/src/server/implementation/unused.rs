//! Used to stub out all the unused host functions on the serverside.
use crate::shared::{implementation::unsupported, wit};

use super::Bindings;

impl wit::client_message::Host for Bindings {
    fn send(
        &mut self,
        _: wit::client_message::Target,
        _: String,
        _: Vec<u8>,
    ) -> anyhow::Result<()> {
        unsupported()
    }
}
impl wit::client_player::Host for Bindings {
    fn get_local(&mut self) -> anyhow::Result<wit::types::EntityId> {
        unsupported()
    }
}
impl wit::client_input::Host for Bindings {
    fn get(&mut self) -> anyhow::Result<wit::client_input::Input> {
        unsupported()
    }

    fn get_previous(&mut self) -> anyhow::Result<wit::client_input::Input> {
        unsupported()
    }
}
impl wit::client_camera::Host for Bindings {
    fn screen_ray(
        &mut self,
        _camera: wit::types::EntityId,
        _clip_space_pos: wit::types::Vec2,
    ) -> anyhow::Result<wit::types::Ray> {
        unsupported()
    }
}
impl wit::client_audio::Host for Bindings {
    fn load(&mut self, _url: String) -> anyhow::Result<()> {
        unsupported()
    }

    fn play(&mut self, _name: String, _looping: bool, _amp: f32) -> anyhow::Result<()> {
        unsupported()
    }
}