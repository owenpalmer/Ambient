use crate::{
    global::{Vec3, Ray},
    internal::{wit, conversion::FromBindgen},
};

#[cfg(feature = "client")]
#[allow(missing_docs)]
pub fn screen_ray() -> Ray {
    wit::camera::screen_ray().from_bindgen()
}
