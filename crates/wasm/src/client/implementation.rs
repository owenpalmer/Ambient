use ambient_input::{player_prev_raw_input, player_raw_input};
use ambient_network::client::game_client;
use anyhow::Context;

use super::Bindings;
use crate::shared::{
    conversion::{FromBindgen, IntoBindgen},
    implementation::message,
    wit,
};

use ambient_core::camera;

use ambient_std::shapes::Ray;
use glam::Vec3;

impl wit::client_message::Host for Bindings {
    fn send(
        &mut self,
        target: wit::client_message::Target,
        name: String,
        data: Vec<u8>,
    ) -> anyhow::Result<()> {
        use wit::client_message::Target;
        let module_id = self.id;
        let world = self.world_mut();

        match target {
            Target::ServerUnreliable | Target::ServerReliable => {
                let connection = world
                    .resource(game_client())
                    .as_ref()
                    .context("no game client")?
                    .connection
                    .clone();

                message::send_networked(
                    world,
                    connection,
                    module_id,
                    &name,
                    &data,
                    matches!(target, Target::ServerReliable),
                )
            }
            Target::LocalBroadcast => message::send_local(world, module_id, None, name, data),
            Target::Local(id) => {
                message::send_local(world, module_id, Some(id.from_bindgen()), name, data)
            }
        }
    }
}
impl wit::client_player::Host for Bindings {
    fn get_raw_input(&mut self) -> anyhow::Result<wit::client_player::RawInput> {
        Ok(self
            .world()
            .resource(player_raw_input())
            .clone()
            .into_bindgen())
    }

    fn get_prev_raw_input(&mut self) -> anyhow::Result<wit::client_player::RawInput> {
        Ok(self
            .world()
            .resource(player_prev_raw_input())
            .clone()
            .into_bindgen())
    }
}
impl wit::camera::Host for Bindings {
    fn screen_ray(&mut self) -> anyhow::Result<wit::types::Ray> {
        let test = Ray {
            origin: Vec3::ONE,
            dir: Vec3::ZERO,
        };
        Ok(test.into_bindgen())
    }
}