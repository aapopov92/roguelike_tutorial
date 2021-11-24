use specs::prelude::*;
use super::{Viewshed, Monster, Name};
use rltk::{Point, console};

pub struct MonsterAI {}

impl<'a> System<'a> for MonsterAI {
	type SystemData = (ReadStorage<'a, Viewshed>,
						ReadExpect<'a, Point>,
						ReadStorage<'a, Monster>,
						ReadStorage<'a, Name>);

	fn run(&mut self, data: Self::SystemData) {
		let (viewshed, ppos, monster, name) = data;

		for (viewshed, _monster, name) in (&viewshed, &monster, &name).join() {
			if viewshed.visible_tiles.contains(&ppos) {
				console::log(format!("{} shouts at you!", &name.name))
			}
		}
	}
}