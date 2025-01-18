#[cfg(not(target_arch = "wasm32"))]
use bevy::{
    prelude::*,
    sprite::{MaterialMesh2dBundle, Mesh2dHandle},
    render::mesh::*,
    render::render_asset::RenderAssetUsages,
};
use bevy::window::PrimaryWindow;
use rand::Rng;

use crate::settings::*;
use crate::tetris_pieces::*;
use crate::tetris_board::*;
use crate::functions::*;

pub struct ParticlePlugin;

impl Plugin for ParticlePlugin{
    fn build(&self, app: &mut App){
        app
        .insert_resource(SpawnParticleBuffer{spawn_particles: Vec::new()})
        .add_systems(Update, (spawn_particles, update_particles, remove_particles));
    }
}

// Buffer to keep count of the particles 
// that we want to spawn

#[derive(Resource)]
pub struct SpawnParticleBuffer{
    pub spawn_particles: Vec<TetrisParticle>,
}

#[derive(Component, Clone)]
pub struct TetrisParticle{
    pos: Vec2,
    vel: Vec2,
    size: f32,
    color: Color,
}

impl TetrisParticle{
    pub fn rand(pos: Vec2, color: Color) -> Self{
        let mut rng = rand::thread_rng();
        let random_angle:f32 = rng.gen::<f32>() * TAU;
        let random_speed:f32= rng.gen::<f32>() * 5.0 + 5.0;
        let random_size:f32 = rng.gen::<f32>() * 10.0 + 10.0;
        
        return TetrisParticle{
            pos: pos,
            vel: Vec2::new(random_angle.cos(), random_angle.sin()) * random_speed,
            size: random_size,
            color: color,
        }
    }

    pub fn update(&mut self){
        self.pos += self.vel;
        self.vel *= 0.95;
        self.size *= 0.95;
    }
}

fn interact(
    mut commands: Commands,
    input: Res<ButtonInput<KeyCode>>,
    mut tetris_pieces_info: ResMut <SpawnParticleBuffer>,
){
    if !input.just_pressed(KeyCode::Space){
        return;
    }

    for i in 0..20{
        tetris_pieces_info.spawn_particles.push(TetrisParticle::rand(Vec2::new(0.0, 0.0), Color::srgb(1.0, 1.0, 1.0)));
    }
}

fn spawn_particles(
    mut particle_buffer: ResMut<SpawnParticleBuffer>,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
){

    for particle in &particle_buffer.spawn_particles{
        commands.spawn((
            SpriteBundle {
                sprite: Sprite {
                    color: particle.color,           // Line color
                    custom_size: Some(Vec2::new(1.0, 1.0)),
                    ..Default::default()
                },
                transform: Transform {
                    translation: Vec3::ZERO,
                    ..Default::default()
                },
                ..Default::default()
            },
            particle.clone(),
        ));
    }

    // clear the spawn buffer
    particle_buffer.spawn_particles.clear();
}

fn remove_particles(
    mut commands: Commands,
    particles: Query<(Entity, &TetrisParticle)>
){
    for (particle_entity, particle) in &particles{
        if particle.size < 1.0{
            commands.entity(particle_entity).despawn();
        }
    }
}

fn update_particles(
    mut particles: Query<(&mut Transform, &mut TetrisParticle)> 
){
    for (mut transform, mut particle) in &mut particles{
        particle.update();

        transform.translation = particle.pos.extend(10.0);
        transform.scale = Vec3::new(particle.size, particle.size, 1.0);
    }
}
