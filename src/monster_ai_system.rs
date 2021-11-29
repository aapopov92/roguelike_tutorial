use specs::prelude::*;
use super::{Viewshed, Monster, Name, Map, Position};
use rltk::{Point, console};

pub struct MonsterAI {}

impl<'a> System<'a> for MonsterAI {
	type SystemData = (WriteStorage<'a, Viewshed>,
						ReadExpect<'a, Point>,
						ReadStorage<'a, Monster>,
						ReadStorage<'a, Name>,
						WriteExpect<'a, Map>,
						WriteStorage<'a, Position>);

	fn run(&mut self, data: Self::SystemData) {
		let (mut viewshed, player_pos, monster, name, mut map, mut position) = data;

		for (mut viewshed, _monster, name, mut pos) in (&mut viewshed, &monster, &name, &mut position).join() {
			let distance = rltk::DistanceAlg::Pythagoras.distance2d(Point::new(pos.x, pos.y), *player_pos);
			if distance < 1.5 {
				console::log(format!("{} shouts at you!", &name.name));
				return;
			}
			if viewshed.visible_tiles.contains(&*player_pos) {
				let path = rltk::a_star_search (
					map.xy_idx(pos.x, pos.y) as i32, 
					map.xy_idx(player_pos.x, player_pos.y) as i32,
					&mut *map
				);
				if path.success && path.steps.len() > 1 {
					pos.x = path.steps[1] as i32 % map.width;
					pos.y = path.steps[1] as i32 / map.width;
					viewshed.dirty = true;
				}
			}
		}
	}
}