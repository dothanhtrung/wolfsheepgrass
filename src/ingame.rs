use std::ops::Deref;

use bevy::prelude::*;

use crate::resource::*;

pub fn moving(
    time: Res<Time>,
    mut object: Query<(&Speed, &MoveDirection, &mut Transform)>,
    mut alive_text: Query<&mut Text, With<AliveText>>,
) {
    let mut text = alive_text.single_mut();
    text.sections[1].value = (object.iter().count() - 1).to_string();

    for (speed, direction, mut transform) in object.iter_mut() {
        if direction.x == 0. && direction.y == 0. {
            return;
        }

        let powx = direction.x.powi(2);
        let powy = direction.y.powi(2);
        let x = transform.translation.x
            + speed.deref() * time.delta().as_millis() as f32 * direction.x / (powx + powy).sqrt();
        let z = transform.translation.z
            + speed.deref() * time.delta().as_millis() as f32 * direction.y / (powx + powy).sqrt();

        let boundary = CHUNK_SIZE / 2.;
        if x <= boundary && x >= -boundary {
            transform.translation.x = x;
        }
        if z <= boundary && z >= -boundary {
            transform.translation.z = z;
        }
    }
}

pub fn change_direction(
    mut wolfs: Query<
        (&mut MoveDirection, &Transform),
        (With<Wolf>, Without<Sheep>, Without<Grass>),
    >,
    mut sheeps: Query<
        (&mut MoveDirection, &Transform),
        (With<Sheep>, Without<Wolf>, Without<Grass>),
    >,
    grasses: Query<&Transform, (With<Grass>, Without<Sheep>, Without<Wolf>)>,
) {
    let (mut wolf_direction, wolf_transform) = wolfs.single_mut();

    let mut closest_sheep = 2. * CHUNK_SIZE.powi(2);
    for (mut sheep_direction, sheep_transform) in sheeps.iter_mut() {
        let mut closest_grass = CHUNK_SIZE.powi(2);
        for grass_transform in grasses.iter() {
            let x = grass_transform.translation.x - sheep_transform.translation.x;
            let z = grass_transform.translation.z - sheep_transform.translation.z;
            let grass_distance = x.powi(2) + z.powi(2);
            if grass_distance < closest_grass {
                closest_grass = grass_distance;
                sheep_direction.x = x;
                sheep_direction.y = z;
            }
        }

        let x = sheep_transform.translation.x - wolf_transform.translation.x;
        let z = sheep_transform.translation.z - wolf_transform.translation.z;
        let wolf_distance = x.powi(2) + z.powi(2);
        if wolf_distance < closest_sheep {
            closest_sheep = wolf_distance;
            wolf_direction.x = x;
            wolf_direction.y = z;
        }

        if (x.powi(2) + z.powi(2)).sqrt() <= SHEEP_VISION {
            sheep_direction.x = x;
            sheep_direction.y = z;
        }
    }
}

fn close_enough(pos1: Vec3, pos2: Vec3) -> bool {
    (pos2.x - pos1.x).abs() <= CELL_SIZE && (pos2.z - pos1.z).abs() <= CELL_SIZE
}

pub fn eat(
    mut commands: Commands,
    mut wolf: Query<&Transform, (With<Wolf>, Without<Sheep>, Without<Grass>)>,
    mut sheeps: Query<
        (Entity, &Transform, &mut Sheep),
        (With<Sheep>, Without<Wolf>, Without<Grass>),
    >,
    mut grasses: Query<(Entity, &Transform), (With<Grass>, Without<Sheep>, Without<Wolf>)>,
    mut wolf_text: Query<(&mut Text, &mut WolfEatCount)>,
) {
    let wolf_pos = wolf.single_mut().translation;
    let (mut text, mut wolf_eat_count) = wolf_text.single_mut();

    for (se, sheep_transform, mut sheep) in sheeps.iter_mut() {
        if close_enough(wolf_pos, sheep_transform.translation) {
            if commands.get_entity(se).is_some() {
                commands.entity(se).despawn_recursive();
                wolf_eat_count.0 += 1;
            }
            continue;
        }

        for (ge, grass_transform) in grasses.iter_mut() {
            if commands.get_entity(ge).is_some() {
                if close_enough(sheep_transform.translation, grass_transform.translation) {
                    commands.entity(ge).despawn_recursive();
                    sheep.hungry = 0;
                }
            }
        }
    }

    text.sections[1].value = wolf_eat_count.to_string();
}

pub fn hungry(
    mut commands: Commands,
    mut sheeps: Query<(Entity, &mut Sheep)>,
    mut hungry_text: Query<(&mut Text, &mut HungryCount)>,
    time: Res<Time>,
) {
    let (mut text, mut hungry_text) = hungry_text.single_mut();

    for (e, mut sheep) in sheeps.iter_mut() {
        sheep.hungry += time.delta().as_millis();

        if sheep.hungry > SHEEP_HUNGRY_TIME {
            if commands.get_entity(e).is_some() {
                commands.entity(e).despawn_recursive();
                hungry_text.0 += 1;
            }
        }
    }

    text.sections[1].value = hungry_text.0.to_string();
}
