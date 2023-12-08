use crate::ratio::Ratio;
use std::collections::VecDeque;
use std::io::stdin;
use itertools::Itertools;

mod ratio;

fn main() {
    let mut lines: VecDeque<String> = stdin().lines().filter_map(|l| l.ok()).collect();

    // Get the seeds
    let seeds= lines
        .pop_front()
        .unwrap();
    let seeds = seeds
        .split(':')
        .last()
        .unwrap()
        .split(' ')
        .filter(|s| s.len() > 0)
        .map(|s| {
            s.trim()
                .to_string()
                .parse::<isize>()
                .expect("Found an empty string")
        }).batching(|it| {
           match it.next() {
               None => None,
               Some(x) => match it.next() {
                   None => None,
                   Some(y) => Some((x, y)),
               }
           }
       }).flat_map(|(a,b)| a..(a+b));


    // Pop empty line
    lines.pop_front();

    // Get the seeds-to-soil ranges
    let seeds_soil = make_ratios(&mut lines);
    println!("Loaded {} seed-to-soil ratios", seeds_soil.len());

    // Get the soil-to-fertiliser ranges
    let soil_fertiliser = make_ratios(&mut lines);
    println!("Loaded {} soil-to-fertiliser ratios", soil_fertiliser.len());

    // Get the fertiliser-to-water ranges
    let fertiliser_water = make_ratios(&mut lines);
    println!(
        "Loaded {} fertiliser-to-water ratios",
        fertiliser_water.len()
    );

    // Get the water-to-light ranges
    let water_light = make_ratios(&mut lines);
    println!("Loaded {} water-to-light ratios", water_light.len());

    // Get the light-to-temp ranges
    let light_temp = make_ratios(&mut lines);
    println!("Loaded {} light-to-temp ratios", light_temp.len());

    // Get the temp-to-humidity ranges
    let temp_humidity = make_ratios(&mut lines);
    println!("Loaded {} temp-to-humidity ratios", temp_humidity.len());

    // Get the humidity-to-location ranges
    let humidity_location = make_ratios(&mut lines);
    println!(
        "Loaded {} humidity-to-location ratios",
        humidity_location.len()
    );

    let result: isize = seeds
        .map(|s| {
            seeds_soil
                .iter()
                .map(|r| r.map_value(&s))
                .filter_map(|v| v.ok())
                .last()
                .unwrap_or(s)
        })
        .map(|s| {
            soil_fertiliser
                .iter()
                .map(|r| r.map_value(&s))
                .filter_map(|v| v.ok())
                .last()
                .unwrap_or(s)
        })
        .map(|s| {
            fertiliser_water
                .iter()
                .map(|r| r.map_value(&s))
                .filter_map(|v| v.ok())
                .last()
                .unwrap_or(s)
        })
        .map(|s| {
            water_light
                .iter()
                .map(|r| r.map_value(&s))
                .filter_map(|v| v.ok())
                .last()
                .unwrap_or(s)
        })
        .map(|s| {
            light_temp
                .iter()
                .map(|r| r.map_value(&s))
                .filter_map(|v| v.ok())
                .last()
                .unwrap_or(s)
        })
        .map(|s| {
            temp_humidity
                .iter()
                .map(|r| r.map_value(&s))
                .filter_map(|v| v.ok())
                .last()
                .unwrap_or(s)
        })
        .map(|s| {
            humidity_location
                .iter()
                .map(|r| r.map_value(&s))
                .filter_map(|v| v.ok())
                .last()
                .unwrap_or(s)
        })
        .min()
        .unwrap();
    println!("{:?}", result)
}

fn make_ratios(lines: &mut VecDeque<String>) -> Vec<Ratio> {
    let mut seeds_soil: Vec<Ratio> = Vec::new();
    loop {
        let line = lines.pop_front().expect("Didn't get a line");
        if (&line).trim().len() == 0 {
            break;
        }
        let ratio = Ratio::from_string(line);
        match ratio {
            Ok(ratio) => {
                seeds_soil.push(ratio);
            }
            Err(_) => {
                continue;
            }
        }
    }
    seeds_soil
}
