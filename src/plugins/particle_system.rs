use std::borrow::Cow;

use bevy::{
    prelude::*,
    render::{extract_resource::{ExtractResource, ExtractResourcePlugin}, render_asset::{RenderAssetUsages, RenderAssets}, render_graph::{self, RenderGraph, RenderLabel}, render_resource::{AsBindGroup, BindGroup, BindGroupEntries, BindGroupLayout, CachedComputePipelineId, ComputePassDescriptor, ComputePipelineDescriptor, Extent3d, PipelineCache, TextureDimension, TextureFormat, TextureUsages}, renderer::{RenderContext, RenderDevice}, Render, RenderApp, RenderSet},
};

pub struct AwesomeParticlesPlugin;

#[derive(Debug, Hash, PartialEq, Eq, Clone, RenderLabel)]
struct AwesomeParticlesLabel;

const SIZE: (u32, u32) = (1280, 720); //Texture size
const WORKGROUP_SIZE: u32 = 8; //Compute shader workgroup size

impl Plugin for AwesomeParticlesPlugin {
    fn build(&self, app: &mut App) {
        app
        .add_plugins(ExtractResourcePlugin::<AwesomeParticlesImage>::default())
        .add_systems(Startup, setup);

        let render_app = app.sub_app_mut(RenderApp);
        render_app.add_systems(
            Render,
            prepare_bind_group.in_set(RenderSet::PrepareBindGroups),
        );

        let mut render_graph = render_app.world.resource_mut::<RenderGraph>();
        render_graph.add_node(AwesomeParticlesLabel, AwesomeParticlesNode::default());
        render_graph.add_node_edge(AwesomeParticlesLabel, bevy::render::graph::CameraDriverLabel);
    }

    fn finish(&self, app: &mut App) {
        let render_app = app.sub_app_mut(RenderApp);
        render_app.init_resource::<AwesomeParticlesPipeline>();
    }
}


fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut images: ResMut<Assets<Image>>,
){
    // Image that will store all the data mutations done in the compute shader (hence the STORAGE_BINDING usage)
    let mut image = Image::new_fill(
        Extent3d {
            width: SIZE.0,
            height: SIZE.1,
            depth_or_array_layers: 1,
        },
        TextureDimension::D2,
        &[0, 0, 0, 255],
        TextureFormat::Rgba8Unorm,
        RenderAssetUsages::RENDER_WORLD,
    );
    image.texture_descriptor.usage =
        TextureUsages::COPY_DST | TextureUsages::STORAGE_BINDING | TextureUsages::TEXTURE_BINDING;
    let image = images.add(image);

    commands.spawn(SpriteBundle {
        sprite: Sprite {
            custom_size: Some(Vec2::new(SIZE.0 as f32, SIZE.1 as f32)),
            ..default()
        },
        texture: image.clone(),
        ..default()
    });
    commands.spawn(Camera2dBundle::default());

    commands.insert_resource(AwesomeParticlesImage { texture: image });
}

//Data storage
#[derive(Resource, Clone, Deref, ExtractResource, AsBindGroup)]
struct AwesomeParticlesImage {
    #[storage_texture(0, image_format = Rgba8Unorm, access = ReadWrite)]
    texture: Handle<Image>,
}

//Data storage binding to GPU
#[derive(Resource)]
struct AwesomeParticlesImageBindGroup(BindGroup);

fn prepare_bind_group(
    mut commands: Commands,
    pipeline: Res<AwesomeParticlesPipeline>,
    gpu_images: Res<RenderAssets<Image>>,
    game_of_life_image: Res<AwesomeParticlesImage>,
    render_device: Res<RenderDevice>,
) {
    let view = gpu_images.get(&game_of_life_image.texture).unwrap();
    let bind_group = render_device.create_bind_group(
        None,
        &pipeline.texture_bind_group_layout,
        &BindGroupEntries::single(&view.texture_view),
    );
    commands.insert_resource(AwesomeParticlesImageBindGroup(bind_group));
}

//Pipeline for the data storage bind group
#[derive(Resource)]
struct AwesomeParticlesPipeline {
    texture_bind_group_layout: BindGroupLayout,
    main_pipeline: CachedComputePipelineId,
}

impl FromWorld for AwesomeParticlesPipeline {
    fn from_world(world: &mut World) -> Self {
        let render_device = world.resource::<RenderDevice>();
        let texture_bind_group_layout = AwesomeParticlesImage::bind_group_layout(render_device);
        let shader = world
            .resource::<AssetServer>()
            .load("shaders/game_of_life.wgsl"); //Compute shader goes here
        let pipeline_cache = world.resource::<PipelineCache>();
        let main_pipeline = pipeline_cache.queue_compute_pipeline(ComputePipelineDescriptor {
            label: None,
            layout: vec![texture_bind_group_layout.clone()],
            push_constant_ranges: Vec::new(),
            shader: shader.clone(),
            shader_defs: vec![],
            entry_point: Cow::from("init"),
        });

        AwesomeParticlesPipeline {
            texture_bind_group_layout,
            main_pipeline,
        }
    }
}

/* Begin compute shader specific logic */
struct AwesomeParticlesNode {
    color: Vec4,
}

impl Default for AwesomeParticlesNode {
    fn default() -> Self {
        Self {
            color: Vec4::new(255.0, 0.0, 0.0, 255.0), //Default to red color
        }
    }
}

impl render_graph::Node for AwesomeParticlesNode {

    fn update(&mut self, world: &mut World) {
        let pipeline = world.resource::<AwesomeParticlesPipeline>();
        let pipeline_cache = world.resource::<PipelineCache>();

        //TODO: implement
    }

    //This function actually runs the compute shader pass
    fn run(
        &self,
        _graph: &mut render_graph::RenderGraphContext,
        render_context: &mut RenderContext,
        world: &World,
    ) -> Result<(), render_graph::NodeRunError> {
        let texture_bind_group = &world.resource::<AwesomeParticlesImageBindGroup>().0;
        let pipeline_cache = world.resource::<PipelineCache>();
        let pipeline = world.resource::<AwesomeParticlesPipeline>();

        let mut pass = render_context
            .command_encoder()
            .begin_compute_pass(&ComputePassDescriptor::default());

        pass.set_bind_group(0, texture_bind_group, &[]);

        // run pipeline
        let main_pipeline = pipeline_cache.get_compute_pipeline(pipeline.main_pipeline).unwrap();
        pass.set_pipeline(main_pipeline);
        pass.dispatch_workgroups(SIZE.0 / WORKGROUP_SIZE, SIZE.1 / WORKGROUP_SIZE, 1);

        Ok(())
    }
}