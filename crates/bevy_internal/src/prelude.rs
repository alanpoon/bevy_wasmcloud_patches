#[doc(hidden)]
pub use crate::{
    app::prelude::*, core::prelude::*, ecs::prelude::*, hierarchy::prelude::*, input::prelude::*,
    log::prelude::*,math::prelude::*, reflect::prelude::*, time::prelude::*,
    transform::prelude::*, utils::prelude::*, window::prelude::*, DefaultPlugins, MinimalPlugins,
};

pub use bevy_derive::{bevy_main, Deref, DerefMut};

#[doc(hidden)]
#[cfg(feature = "bevy_asset")]
pub use crate::asset::prelude::*;

#[doc(hidden)]
#[cfg(feature = "bevy_core_pipeline")]
pub use crate::core_pipeline::prelude::*;

#[doc(hidden)]
#[cfg(feature = "bevy_render")]
pub use crate::render::prelude::*;

#[doc(hidden)]
#[cfg(feature = "bevy_scene")]
pub use crate::scene::prelude::*;

#[doc(hidden)]
#[cfg(feature = "bevy_sprite")]
pub use crate::sprite::prelude::*;
