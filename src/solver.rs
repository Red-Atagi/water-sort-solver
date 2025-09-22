use crate::vial::Vial;
use std::collections::HashSet;

#[derive(Clone)]
pub struct Solver {
    vials: Vec<Vial>,
}

impl Solver {
    pub fn new(vials: Vec<Vial>) -> Self {
        Solver { vials }
    }

    pub fn move_liquid(&mut self, from: usize, to: usize) -> Result<(), String> {
        if from >= self.vials.len() || to >= self.vials.len() {
            return Err("Invalid vial index".to_string());
        }
        let liquid = self.vials[from].pour_out()?;
        self.vials[to].pour_in(liquid)?;
        Ok(())
    }

    pub fn is_solved(&self) -> bool {
        self.vials.iter().all(|vial| {
            vial.volume() == 0
                || (vial.top_color().is_some()
                    && vial.volume() == vial.capacity()
                    && vial.top_color_count() == vial.volume())
        })
    }

    pub fn solution_steps(&mut self, visted: &mut HashSet<Vec<Vial>>) -> Option<Vec<(usize, usize)>> {
        // Placeholder for actual solution logic
        // base case if state is solved
        if self.is_solved() {
            return Some(vec![]);
        }

        // find a source vial
        for i in 0..self.vials.len() {
            let source_vial = &self.vials[i];

            //skip if empty or full of one color
            if source_vial.volume() == 0 || source_vial.top_color_count() == source_vial.capacity() {
                continue;
            }

            // find source color and amount
            let source_color = source_vial.top_color().unwrap();
            let source_amount = source_vial.top_color_count();
            let source_volume = source_vial.volume();

            // find a target vial
            for j in 0..self.vials.len() {
                if i == j {
                    continue;
                }
                let target_vial = &self.vials[j];
                // skip if not enough space
                if target_vial.space_left() < source_amount {
                    continue;
                }
                // skip if target is empty and source amount is the source volume aka skip useless move
                if target_vial.volume() == 0 && source_amount == source_volume {
                    continue;
                }
                // skip if top color is different
                if let Some(target_color) = target_vial.top_color() {
                    if target_color != source_color {
                        continue;
                    }
                }

                // pour from source to target
                let mut new_solver = self.clone();
                if new_solver.move_liquid(i, j).is_err() {
                    panic!("Unexpected error during move_liquid");
                }

                // recurse
                if visted.insert(new_solver.vials.clone()){
                    if let Some(mut steps) = new_solver.solution_steps(visted) {
                        // println!("source: {:?}, target: {:?}", source_vial, target_vial);
                        // println!("steps: {:?}\n", steps);
                        steps.insert(0, (i, j));
                        return Some(steps);
                    }
                }   

            }
        }
        //if we get here there is no solution
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::liquid::Liquid;

    #[test]
    fn test_move_liquid() {
        let mut solver = Solver::new(vec![
            Vial::from_liquids(4, vec![Liquid::new("red".to_string(), 2)]),
            Vial::from_liquids(4, vec![]),
        ]);
        assert!(solver.move_liquid(0, 1).is_ok());
        assert_eq!(solver.vials[0].volume(), 0);
        assert_eq!(solver.vials[1].volume(), 2);
    }

    #[test]
    fn test_is_solved() {
        let solver = Solver::new(vec![
            Vial::from_liquids(4, vec![Liquid::new("red".to_string(), 4)]),
            Vial::from_liquids(4, vec![]),
        ]);
        assert!(solver.is_solved());

        let solver = Solver::new(vec![
            Vial::from_liquids(4, vec![Liquid::new("red".to_string(), 2)]),
            Vial::from_liquids(4, vec![]),
        ]);
        assert!(!solver.is_solved());
    }

    #[test]
    fn test_solution_steps() {
        let mut solver = Solver::new(vec![
            Vial::from_liquids(4, vec![Liquid::new("red".to_string(), 2), Liquid::new("blue".to_string(), 2)]),
            Vial::from_liquids(4, vec![Liquid::new("blue".to_string(), 2), Liquid::new("red".to_string(), 2)]),
            Vial::from_liquids(4, vec![]),
            Vial::from_liquids(4, vec![]),
        ]);
        let steps = solver.solution_steps(&mut HashSet::new());
        assert!(steps.is_some());
        let steps = steps.unwrap();
        // println!("Steps: {:?}\n", steps);
        for (from, to) in steps {
            assert!(solver.move_liquid(from, to).is_ok());
            // println!("state: {:?}\n", solver.vials);
        }
        assert!(solver.is_solved());
    }

    #[test]
    fn test_unsolvable() {
        let mut solver = Solver::new(vec![
            Vial::from_liquids(4, vec![Liquid::new("red".to_string(), 2), Liquid::new("blue".to_string(), 2)]),
            Vial::from_liquids(4, vec![Liquid::new("blue".to_string(), 2), Liquid::new("red".to_string(), 1)]),
            Vial::from_liquids(4, vec![]),
            Vial::from_liquids(4, vec![]),
        ]);
        let steps = solver.solution_steps(&mut HashSet::new());
        assert!(steps.is_none());
    }
}