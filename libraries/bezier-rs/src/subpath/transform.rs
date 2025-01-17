use super::*;
use crate::ComputeType;

/// Functionality that transforms Subpaths, such as split, reduce, offset, etc.
impl Subpath {
	/// Returns either one or two Subpaths that result from splitting the original Subpath at the point corresponding to `t`.
	/// If the original Subpath was closed, a single open Subpath will be returned.
	/// If the original Subpath was open, two open Subpaths will be returned.
	pub fn split(&self, t: ComputeType) -> (Subpath, Option<Subpath>) {
		match t {
			ComputeType::Parametric(t) => {
				assert!((0.0..=1.).contains(&t));

				let number_of_curves = self.len_segments() as f64;
				let scaled_t = t * number_of_curves;

				let target_curve_index = scaled_t.floor() as i32;
				let target_curve_t = scaled_t % 1.;
				let num_manipulator_groups = self.manipulator_groups.len();

				// The only case where `curve` would be `None` is if the provided argument was 1
				let optional_curve = self.iter().nth(target_curve_index as usize);
				let curve = optional_curve.unwrap_or_else(|| self.iter().last().unwrap());

				let [first_bezier, second_bezier] = curve.split(if t == 1. { t } else { target_curve_t });

				let mut clone = self.manipulator_groups.clone();
				let (mut first_split, mut second_split) = if t > 0. {
					let clone2 = clone.split_off(num_manipulator_groups.min((target_curve_index as usize) + 1));
					(clone, clone2)
				} else {
					(vec![], clone)
				};

				if self.closed && (t == 0. || t == 1.) {
					// The entire vector of manipulator groups will be in the second_split because target_curve_index == 0.
					// Add a new manipulator group with the same anchor as the first node to represent the end of the now opened subpath
					let last_curve = self.iter().last().unwrap();
					first_split.push(ManipulatorGroup {
						anchor: first_bezier.end(),
						in_handle: last_curve.handle_end(),
						out_handle: None,
					});
				} else {
					if !first_split.is_empty() {
						let num_elements = first_split.len();
						first_split[num_elements - 1].out_handle = first_bezier.handle_start();
					}

					if !second_split.is_empty() {
						second_split[0].in_handle = second_bezier.handle_end();
					}

					// Push new manipulator groups to represent the location of the split at the end of the first group and at the start of the second
					// If the split was at a manipulator group's anchor, add only one manipulator group
					// Add it to the first list when the split location is on the first manipulator group, otherwise add to the second list
					if target_curve_t != 0. || t == 0. {
						first_split.push(ManipulatorGroup {
							anchor: first_bezier.end(),
							in_handle: first_bezier.handle_end(),
							out_handle: None,
						});
					}

					if t != 0. {
						second_split.insert(
							0,
							ManipulatorGroup {
								anchor: second_bezier.start(),
								in_handle: None,
								out_handle: second_bezier.handle_start(),
							},
						);
					}
				}

				if self.closed {
					// "Rotate" the manipulator groups list so that the split point becomes the start and end of the open subpath
					second_split.append(&mut first_split);
					(Subpath::new(second_split, false), None)
				} else {
					(Subpath::new(first_split, false), Some(Subpath::new(second_split, false)))
				}
			}
			// TODO: change this implementation to Euclidean compute
			ComputeType::Euclidean(_t) => todo!(),
			ComputeType::EuclideanWithinError { t: _, epsilon: _ } => todo!(),
		}
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use glam::DVec2;

	fn set_up_open_subpath() -> Subpath {
		let start = DVec2::new(20., 30.);
		let middle1 = DVec2::new(80., 90.);
		let middle2 = DVec2::new(100., 100.);
		let end = DVec2::new(60., 45.);

		let handle1 = DVec2::new(75., 85.);
		let handle2 = DVec2::new(40., 30.);
		let handle3 = DVec2::new(10., 10.);

		Subpath::new(
			vec![
				ManipulatorGroup {
					anchor: start,
					in_handle: None,
					out_handle: Some(handle1),
				},
				ManipulatorGroup {
					anchor: middle1,
					in_handle: None,
					out_handle: Some(handle2),
				},
				ManipulatorGroup {
					anchor: middle2,
					in_handle: None,
					out_handle: None,
				},
				ManipulatorGroup {
					anchor: end,
					in_handle: None,
					out_handle: Some(handle3),
				},
			],
			false,
		)
	}

	fn set_up_closed_subpath() -> Subpath {
		let mut subpath = set_up_open_subpath();
		subpath.closed = true;
		subpath
	}

	#[test]
	fn split_an_open_subpath() {
		let subpath = set_up_open_subpath();
		let location = subpath.evaluate(ComputeType::Parametric(0.2));
		let split_pair = subpath.iter().next().unwrap().split((0.2 * 3.) % 1.);
		let (first, second) = subpath.split(ComputeType::Parametric(0.2));
		assert!(second.is_some());
		let second = second.unwrap();
		assert_eq!(first.manipulator_groups[1].anchor, location);
		assert_eq!(second.manipulator_groups[0].anchor, location);
		assert_eq!(split_pair[0], first.iter().last().unwrap());
		assert_eq!(split_pair[1], second.iter().next().unwrap());
	}

	#[test]
	fn split_at_start_of_an_open_subpath() {
		let subpath = set_up_open_subpath();
		let location = subpath.evaluate(ComputeType::Parametric(0.));
		let split_pair = subpath.iter().next().unwrap().split(0.);
		let (first, second) = subpath.split(ComputeType::Parametric(0.));
		assert!(second.is_some());
		let second = second.unwrap();
		assert_eq!(
			first.manipulator_groups[0],
			ManipulatorGroup {
				anchor: location,
				in_handle: None,
				out_handle: None
			}
		);
		assert_eq!(first.manipulator_groups.len(), 1);
		assert_eq!(second.manipulator_groups[0].anchor, location);
		assert_eq!(split_pair[1], second.iter().next().unwrap());
	}

	#[test]
	fn split_at_end_of_an_open_subpath() {
		let subpath = set_up_open_subpath();
		let location = subpath.evaluate(ComputeType::Parametric(1.));
		let split_pair = subpath.iter().last().unwrap().split(1.);
		let (first, second) = subpath.split(ComputeType::Parametric(1.));
		assert!(second.is_some());
		let second = second.unwrap();
		assert_eq!(first.manipulator_groups[3].anchor, location);
		assert_eq!(split_pair[0], first.iter().last().unwrap());
		assert_eq!(
			second.manipulator_groups[0],
			ManipulatorGroup {
				anchor: location,
				in_handle: None,
				out_handle: None
			}
		);
		assert_eq!(second.manipulator_groups.len(), 1);
	}

	#[test]
	fn split_a_closed_subpath() {
		let subpath = set_up_closed_subpath();
		let location = subpath.evaluate(ComputeType::Parametric(0.2));
		let split_pair = subpath.iter().next().unwrap().split((0.2 * 4.) % 1.);
		let (first, second) = subpath.split(ComputeType::Parametric(0.2));
		assert!(second.is_none());
		assert_eq!(first.manipulator_groups[0].anchor, location);
		assert_eq!(first.manipulator_groups[5].anchor, location);
		assert_eq!(first.manipulator_groups.len(), 6);
		assert_eq!(split_pair[0], first.iter().last().unwrap());
		assert_eq!(split_pair[1], first.iter().next().unwrap());
	}

	#[test]
	fn split_at_start_of_a_closed_subpath() {
		let subpath = set_up_closed_subpath();
		let location = subpath.evaluate(ComputeType::Parametric(0.));
		let (first, second) = subpath.split(ComputeType::Parametric(0.));
		assert!(second.is_none());
		assert_eq!(first.manipulator_groups[0].anchor, location);
		assert_eq!(first.manipulator_groups[4].anchor, location);
		assert_eq!(subpath.manipulator_groups[0..], first.manipulator_groups[..4]);
		assert!(!first.closed);
		assert_eq!(first.iter().last().unwrap(), subpath.iter().last().unwrap());
		assert_eq!(first.iter().next().unwrap(), subpath.iter().next().unwrap());
	}

	#[test]
	fn split_at_end_of_a_closed_subpath() {
		let subpath = set_up_closed_subpath();
		let location = subpath.evaluate(ComputeType::Parametric(1.));
		let (first, second) = subpath.split(ComputeType::Parametric(1.));
		assert!(second.is_none());
		assert_eq!(first.manipulator_groups[0].anchor, location);
		assert_eq!(first.manipulator_groups[4].anchor, location);
		assert_eq!(subpath.manipulator_groups[0..], first.manipulator_groups[..4]);
		assert!(!first.closed);
		assert_eq!(first.iter().last().unwrap(), subpath.iter().last().unwrap());
		assert_eq!(first.iter().next().unwrap(), subpath.iter().next().unwrap());
	}
}
