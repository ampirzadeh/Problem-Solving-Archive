use std::{
    fs::File,
    io::{BufRead, BufReader},
    path::Path,
};

struct RangeInfo {
    range_destination_start: u64,
    range_source_start: u64,
    range_length: u64,
}

impl RangeInfo {
    fn new(l: &str) -> Self {
        let info = l
            .trim()
            .split(" ")
            .map(|f| f.parse::<u64>().expect("Failed to parse to int"))
            .collect::<Vec<u64>>();

        RangeInfo {
            range_destination_start: info[0],
            range_source_start: info[1],
            range_length: info[2],
        }
    }
}

fn get_value(k: u64, range_info: &Vec<String>) -> u64 {
    for l in range_info {
        let ri = RangeInfo::new(l);

        if ri.range_source_start <= k && k <= ri.range_source_start + ri.range_length {
            return ri.range_destination_start + (k - ri.range_source_start);
        }
    }

    return k;
}

#[derive(Debug)]
struct SeedResult(u64, u64);

fn main() {
    setupper::logger();

    let p = Path::new("src/data/day5.txt");
    let file = File::open(&p).unwrap();
    let reader = BufReader::new(&file);

    let mut seeds_and_results: Vec<SeedResult> = vec![];
    let mut info: Vec<String> = vec![];

    for line in reader.lines() {
        let linee = line.expect("Failed to read line");

        // if reading data about seeds
        if let Some(l) = linee.strip_prefix("seeds: ") {
            seeds_and_results = l
                .trim()
                .split(" ")
                .map(|f| f.parse::<u64>().expect("Failed to parse to int"))
                .map(|f| SeedResult(f, f))
                .collect::<Vec<SeedResult>>();
        }
        // if the current "map" information is over map the old info to the new ones
        else if linee.trim().is_empty() {
            for sr in seeds_and_results.iter_mut() {
                sr.1 = get_value(sr.1, &info)
            }

            log::debug!("{:?}", info);
            log::debug!("{:?}", seeds_and_results);

            info.clear();
        }
        // if reading a new "map"
        else if linee.contains("map:") {
        } else {
            info.push(linee.to_string());
        }
    }

    // map the final information to the destination
    for sr in seeds_and_results.iter_mut() {
        sr.1 = get_value(sr.1, &info)
    }

    log::debug!("{:?}", info);
    log::debug!("{:?}", seeds_and_results);


    log::info!(
        "{:?}",
        seeds_and_results
            .iter()
            .min_by(|x, y| x.1.cmp(&y.1))
            .expect("Couldn't get minimum")
    )
}
