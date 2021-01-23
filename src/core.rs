

#[derive(Clone, Debug)]
pub struct Dj {
	size: usize,
	data: Vec<Vec<i32>>
}

impl Dj {
	pub fn new(node: i32) -> Dj {
		Dj {size: node as usize, data: vec![vec![i32::MAX; node as usize]; node as usize]}
	}

	pub fn distance_set(&mut self, snode: i32, dnode: i32, dist: i32) {
		self.data[snode as usize][dnode as usize] = dist;
		self.data[dnode as usize][snode as usize] = dist;
	}

	pub fn run(&self, start: i32, goal: i32) -> Vec<i32> {
		let mut target = 0 as usize;

		let mut cost = vec![i32::MAX; self.size];
		let mut via = vec![0; self.size];

		let mut used = vec![false; self.size];

		cost[start as usize] = 0;

		for _ in 0..self.size {
			let mut min = i32::MAX;

			for i in 0..self.size {
				if !used[i] && min > cost[i] {
					min = cost[i];
					target = i;
				}
			}

			if target == goal as usize {
				break;
			}

			for neighbor in 0..self.size {
				if cost[neighbor] > self.data[target][neighbor].saturating_add(cost[target]) {
					cost[neighbor] = self.data[target][neighbor] + cost[target];
					via[neighbor] = target;
				}
			}

			used[target] = true;
		}

		let mut count = goal as usize;
		let mut res = vec![goal; 1];
		for _ in 0..self.size {
			count = via[count];
			res.push(count as i32);
    		//printf(" -> %d", count);
    		if count == start as usize {
				break
			};
		}

		res
	}
}