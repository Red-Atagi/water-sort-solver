pub mod vial;
pub mod liquid;
pub mod solver;

use std::{collections::HashSet};
use itertools::Itertools;

use crate::liquid::Liquid;

fn main() {
    let orange = "orange".to_string();
    let mint = "mint".to_string();
    let maroon = "maroon".to_string();
    let green = "green".to_string();
    let brown = "brown".to_string();
    let pink = "pink".to_string();
    let magenta = "magenta".to_string();
    let purple = "purple".to_string();
    let red = "red".to_string();
    let light_blue = "light_blue".to_string();
    let lavender = "lavender".to_string();

    let orange_liquid = liquid::Liquid::new(orange, 1);
    let mint_liquid = liquid::Liquid::new(mint, 1);
    let maroon_liquid = liquid::Liquid::new(maroon, 1);
    let green_liquid = liquid::Liquid::new(green, 1);
    let brown_liquid = liquid::Liquid::new(brown, 1);
    let pink_liquid = liquid::Liquid::new(pink, 1);
    let magenta_liquid = liquid::Liquid::new(magenta, 1);
    let purple_liquid = liquid::Liquid::new(purple, 1);
    let red_liquid = liquid::Liquid::new(red, 1);
    let light_blue_liquid = liquid::Liquid::new(light_blue, 1);
    let lavender_liquid = liquid::Liquid::new(lavender, 1);

    let vial0 = vial::Vial::from_liquids(4, vec![red_liquid.clone(), mint_liquid.clone(), maroon_liquid.clone(), green_liquid.clone()].into_iter().rev().collect());
    let vial1 = vial::Vial::from_liquids(4, vec![brown_liquid.clone(), purple_liquid.clone(), magenta_liquid.clone(), orange_liquid.clone()].into_iter().rev().collect());
    let vial2 = vial::Vial::from_liquids(4, vec![orange_liquid.clone(), mint_liquid.clone(), green_liquid.clone(), orange_liquid.clone()].into_iter().rev().collect());
    let vial3 = vial::Vial::from_liquids(4, vec![orange_liquid.clone(), purple_liquid.clone(), light_blue_liquid.clone(),brown_liquid.clone()].into_iter().rev().collect());
    let vial4 = vial::Vial::from_liquids(4, vec![lavender_liquid.clone(), pink_liquid.clone(), green_liquid.clone(), maroon_liquid.clone()].into_iter().rev().collect());
    let vial5 = vial::Vial::from_liquids(4, vec![purple_liquid.clone(), magenta_liquid.clone(), lavender_liquid.clone(), light_blue_liquid.clone()].into_iter().rev().collect());
    let vial6 = vial::Vial::from_liquids(4, vec![red_liquid.clone(), magenta_liquid.clone(), pink_liquid.clone(), light_blue_liquid.clone()].into_iter().rev().collect());
    let vial7 = vial::Vial::from_liquids(4, vec![maroon_liquid.clone(), brown_liquid.clone(), green_liquid.clone(), purple_liquid].into_iter().rev().collect());
    let vial8 = vial::Vial::from_liquids(4, vec![lavender_liquid.clone(), red_liquid.clone(), light_blue_liquid.clone(), pink_liquid.clone()].into_iter().rev().collect());
    let vial9 = vial::Vial::from_liquids(4, vec![magenta_liquid.clone(), maroon_liquid.clone(), lavender_liquid.clone(), pink_liquid.clone()].into_iter().rev().collect());
    let vial10 = vial::Vial::from_liquids(4, vec![brown_liquid.clone(), mint_liquid.clone(), red_liquid.clone(), mint_liquid.clone()].into_iter().rev().collect());

    let mut solver = solver::Solver::new(vec![vial0, vial1, vial2, vial3, vial4, vial5, vial6, vial7, vial8, vial9, vial10, vial::Vial::new(4), vial::Vial::new(4)]);
    println!("{:?}", solver.solution_steps(&mut HashSet::new()).unwrap());

}
