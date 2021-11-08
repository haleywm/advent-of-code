use itertools::Itertools;
use regex::Regex;
use std::cmp::max;

type Tool = (i32, i32, i32);
type ToolList = Vec<Tool>;
fn main() {
    let stats_re = Regex::new(r"^.+ (\d+)$").unwrap();

    let mut lines = advent::line_iter("input/day_21.txt")
        .expect("Unable to open input file")
        .map(|l| l.unwrap());

    let weapons: ToolList = lines
        .by_ref()
        .skip(1)
        .take_while(|l| l.len() > 0)
        .map(parse_tool)
        .collect();

    let mut armor: ToolList = lines
        .by_ref()
        .skip(1)
        .take_while(|l| l.len() > 0)
        .map(parse_tool)
        .collect();

    let mut rings: ToolList = lines
        .by_ref()
        .skip(1)
        .take_while(|l| l.len() > 0)
        .map(parse_tool)
        .collect();
    
    // Boss
    let boss_stats: Vec<i32> = lines
        .map(|l| {
            let cap = stats_re.captures(&l).unwrap();
            cap.get(1).unwrap().as_str().parse().unwrap()
        })
        .collect();
    
    assert!(boss_stats.len() == 3, "Invalid boss stat amount");
    let boss: Tool = (boss_stats[0], boss_stats[1], boss_stats[2]);
    armor.push((0, 0, 0));
    rings.push((0, 0, 0));
    rings.push((0, 0, 0));
    // Generate every possible player:
    // Player gets 1 weapon, 0-1 armor, 0-2 rings

    let loadouts = weapons
        .iter()
        .map(|weapons| {
            armor
                .iter()
                .map(|armor| {
                    rings
                        .iter()
                        .combinations(2)
                        .map(|rings| (weapons.clone(), armor.clone(), rings[0].clone(), rings[1].clone()))
                })
        })
        .flatten()
        .flatten()
        .map(|(weapon, armor, ring_one, ring_two)| {
            (
                weapon.0 + armor.0 + ring_one.0 + ring_two.0,
                weapon.1 + armor.1 + ring_one.1 + ring_two.1,
                weapon.2 + armor.2 + ring_one.2 + ring_two.2,
            )
        });

    let best: i32 = loadouts
        .clone()
        .filter_map(|(cost, damage, armor)| {
            // Doing simulation here, returning the cost of all winning games
            let mut player_stats = (100, damage, armor);
            let mut boss_stats = boss.clone();

            loop {
                // Player attack
                boss_stats.0 -= max(1, player_stats.1 - boss_stats.2);
                if boss_stats.0 <= 0 {
                    // Boss died
                    return Some(cost);
                }
                // Boss attack
                player_stats.0 -= max(1, boss_stats.1 - player_stats.2);
                if player_stats.0 <= 0 {
                    // Player died
                    return None;
                }
            }
        })
        .min()
        .unwrap();

    println!("{}", best);

    let worst: i32 = loadouts
        .filter_map(|(cost, damage, armor)| {
            // Doing simulation here, returning the cost of all winning games
            let mut player_stats = (100, damage, armor);
            let mut boss_stats = boss.clone();

            loop {
                // Player attack
                boss_stats.0 -= max(1, player_stats.1 - boss_stats.2);
                if boss_stats.0 <= 0 {
                    // Boss died
                    return None;
                }
                // Boss attack
                player_stats.0 -= max(1, boss_stats.1 - player_stats.2);
                if player_stats.0 <= 0 {
                    // Player died
                    return Some(cost);
                }
            }
        })
        .max()
        .unwrap();
        
    println!("{}", worst);
}

fn parse_tool(s: String) -> Tool {
    let tool_re = Regex::new(r"^.+\s+(\d+)\s+(\d+)\s+(\d+)$").unwrap();
    let cap = tool_re.captures(&s).unwrap();
    let a = cap.get(1).unwrap().as_str().parse().unwrap();
    let b = cap.get(2).unwrap().as_str().parse().unwrap();
    let c = cap.get(3).unwrap().as_str().parse().unwrap();
    (a, b, c)
}
