use std::fs::File;
use std::io::{self, prelude::*, BufReader};
use crate::Node;

pub fn process_data() -> io::Result<()> {
    let file = File::open("us-latest.osm")?;
    let reader = BufReader::new(file);

    let mut nodes = Vec::new();
    let mut index = 0;

    for line in reader.lines() {
        let mut line = line?;
        line = line.trim().to_string();

        if line.starts_with("<node") && line.ends_with("\">") {
            let lat = line.split("lat=\"").nth(1).unwrap().split("\"").next().unwrap().parse::<f64>().unwrap();
            let lon = line.split("lon=\"").nth(1).unwrap().split("\"").next().unwrap().parse::<f64>().unwrap();

            nodes.push(Node {
                name: String::new(),
                place: String::new(),
                province: String::new(),
                lat,
                lon,
            });
        }

        if line.starts_with("<tag k=\"name\"") && index < nodes.len() {
            let name = line.split("v=\"").nth(1).unwrap().split("\"").next().unwrap();
            nodes[index].name = name.to_string();
        }

        if line.starts_with("<tag k=\"place\"") && index < nodes.len() {
            let place = line.split("v=\"").nth(1).unwrap().split("\"").next().unwrap();
            nodes[index].place = place.to_string();
        }

        if line.starts_with("<tag k=\"is_in:province\"") && index < nodes.len() {
            let province = line.split("v=\"").nth(1).unwrap().split("\"").next().unwrap();
            nodes[index].province = province.trim().to_string();
        }

        if line.contains("</node>") {
            index += 1;
        }
    }

    // remove all nodes without place
    nodes.retain(|node| node.place != "");
    nodes.retain(|node| node.place == "town" || node.place == "village" || node.place == "city");

    // save nodes to file
    let mut file = File::create("nodes.json")?;
    file.write_all(serde_json::to_string_pretty(&nodes).unwrap().as_bytes())?;

    Ok(())
}

pub fn to_static_file() -> io::Result<()> {
    let file = File::open("nodes.json")?;
    let reader = BufReader::new(file);

    let country = "USA";
    let lines = String::from_utf8(reader.bytes().map(|b| b.unwrap()).collect()).unwrap();
    let nodes: Vec<Node> = serde_json::from_str(&lines).unwrap();
    let mut places = String::from("pub const PLACES: [Place; 1] = [");

    for node in nodes {
        let name: String = if node.province == "" {
            format!("{}, {}", node.name, country)
        } else {
            format!("{} ({}), {}", node.name, node.province, country)
        };

        places.push_str(&format!("Place {{ name: \"{}\", lat: {:.6}, lon: {:.6} }},\n", name, node.lat, node.lon));
    }

    places.push_str("];");

    let mut file = File::create("places.rs")?;
    file.write_all(places.as_bytes())?;

    Ok(())
}