extern crate core;

pub mod camera;
pub mod color;
pub mod extract_component;
mod extract_param;
pub mod extract_resource;
pub mod mesh;
pub mod primitives;
pub mod rangefinder;
pub mod render_asset;
pub mod render_graph;
pub mod render_phase;
pub mod render_resource;
pub mod renderer;
pub mod settings;
mod spatial_bundle;
pub mod texture;
pub mod view;

pub use extract_param::Extract;

pub mod prelude {
    #[doc(hidden)]
    pub use crate::{
        camera::{Camera, OrthographicProjection, PerspectiveProjection},
        color::Color,
        mesh::{shape, Mesh},
        render_resource::Shader,
        spatial_bundle::SpatialBundle,
        texture::Image,
        view::{ComputedVisibility, Msaa, Visibility, VisibilityBundle},
    };
}

pub use once_cell;

use crate::{
    camera::CameraPlugin,
    color::Color,
    mesh::MeshPlugin,
    primitives::{CubemapFrusta, Frustum},
    render_graph::RenderGraph,
    render_resource::{PipelineCache, Shader, ShaderLoader},
    renderer::render_system,
    texture::ImagePlugin,
    view::{ViewPlugin, WindowRenderPlugin},
};
use bevy_app::{App, AppLabel, Plugin};
use bevy_asset::{AddAsset, AssetServer};
use bevy_ecs::prelude::*;
use bevy_utils::tracing::debug;
use std::{
    any::TypeId,
    ops::{Deref, DerefMut},
};

/// Contains the default Bevy rendering backend based on wgpu.
#[derive(Default)]
pub struct RenderPlugin;

/// The labels of the default App rendering stages.
#[derive(Debug, Hash, PartialEq, Eq, Clone, StageLabel)]
pub enum RenderStage {
    /// Extract data from the "app world" and insert it into the "render world".
    /// This step should be kept as short as possible to increase the "pipelining potential" for
    /// running the next frame while rendering the current frame.
    Extract,

    /// Prepare render resources from the extracted data for the GPU.
    Prepare,

    /// Create [`BindGroups`](crate::render_resource::BindGroup) that depend on
    /// [`Prepare`](RenderStage::Prepare) data and queue up draw calls to run during the
    /// [`Render`](RenderStage::Render) stage.
    Queue,

    // TODO: This could probably be moved in favor of a system ordering abstraction in Render or Queue
    /// Sort the [`RenderPhases`](crate::render_phase::RenderPhase) here.
    PhaseSort,

    /// Actual rendering happens here.
    /// In most cases, only the render backend should insert resources here.
    Render,

    /// Cleanup render resources here.
    Cleanup,
}

/// The simulation [`World`] of the application, stored as a resource.
/// This resource is only available during [`RenderStage::Extract`] and not
/// during command application of that stage.
/// See [`Extract`] for more details.
#[derive(Default)]
pub struct MainWorld(World);

impl Deref for MainWorld {
    type Target = World;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for MainWorld {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

pub mod main_graph {
    pub mod node {
        pub const CAMERA_DRIVER: &str = "camera_driver";
    }
}

/// A Label for the rendering sub-app.
#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, AppLabel)]
pub struct RenderApp;

impl Plugin for RenderPlugin {
    /// Initializes the renderer, sets up the [`RenderStage`](RenderStage) and creates the rendering sub-app.
    fn build(&self, app: &mut App) {
        
    }
}

/// A "scratch" world used to avoid allocating new worlds every frame when
/// swapping out the [`MainWorld`] for [`RenderStage::Extract`].
#[derive(Default)]
struct ScratchMainWorld(World);

/// Executes the [`Extract`](RenderStage::Extract) stage of the renderer.
/// This updates the render world with the extracted ECS data of the current frame.
fn extract(app_world: &mut World, render_app: &mut App) {
    let extract = render_app
        .schedule
        .get_stage_mut::<SystemStage>(&RenderStage::Extract)
        .unwrap();

    // temporarily add the app world to the render world as a resource
    let scratch_world = app_world.remove_resource::<ScratchMainWorld>().unwrap();
    let inserted_world = std::mem::replace(app_world, scratch_world.0);
    let running_world = &mut render_app.world;
    running_world.insert_resource(MainWorld(inserted_world));

    extract.run(running_world);
    // move the app world back, as if nothing happened.
    let inserted_world = running_world.remove_resource::<MainWorld>().unwrap();
    let scratch_world = std::mem::replace(app_world, inserted_world.0);
    app_world.insert_resource(ScratchMainWorld(scratch_world));

    // Note: We apply buffers (read, Commands) after the `MainWorld` has been removed from the render app's world
    // so that in future, pipelining will be able to do this too without any code relying on it.
    // see <https://github.com/bevyengine/bevy/issues/5082>
    extract.apply_buffers(running_world);
}
