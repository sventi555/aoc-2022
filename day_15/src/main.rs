use utils::read_lines;

struct SensorBeaconPair {
    sensor: (i32, i32),
    beacon: (i32, i32),
}

impl From<String> for SensorBeaconPair {
    fn from(s: String) -> SensorBeaconPair {
        // [['Sensor at x=2662540', 'y=1992627'], ['closest beacon is at x=1562171', 'y=2000000']]
        let coords: Vec<Vec<i32>> = s
            .split(':')
            .map(|coord| {
                coord
                    .split(',')
                    .map(|val| {
                        val.split('=')
                            .collect::<Vec<&str>>()
                            .last()
                            .unwrap()
                            .parse()
                            .unwrap()
                    })
                    .collect()
            })
            .collect();

        SensorBeaconPair {
            sensor: (coords[0][0], coords[0][1]),
            beacon: (coords[1][0], coords[1][1]),
        }
    }
}

fn part_a() {
    let sensor_beacon_pairs: Vec<SensorBeaconPair> = read_lines("./day_15/inpute.txt")
        .map(|line| line.into())
        .collect();
}

fn main() {
    part_a();
}
