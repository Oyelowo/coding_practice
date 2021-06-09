use std::fmt::{self, Display, Formatter};

struct City {
    name: &'static str,
    lat: f32,
    lon: f32,
}

impl Display for City {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        let lat_c = if self.lat > 0.0 { "N" } else { "S" };
        let lon_c = if self.lon > 0.0 { "E" } else { "W" };
        write!(
            f,
            "{}: {:.3}°{} {:.3}°{}",
            self.name,
            self.lat.abs(),
            lat_c,
            self.lon.abs(),
            lon_c
        )
    }
}

#[derive(Debug)]
struct Color {
    red: u8,
    green: u8,
    blue: u8,
}

impl Display for Color {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "RGB({r}, {g}, {b}) Ox{rhex:02X}{ghex:02X}{bhex:02X}",
            r = self.red,
            g = self.green,
            b = self.blue,
            rhex = self.red,
            ghex = self.green,
            bhex = self.blue
        )
    }
}

fn main() {
    let cities = [
        City {
            name: "Dublin",
            lat: 53.347778,
            lon: -6.259722,
        },
        City {
            name: "Oslo",
            lat: 59.95,
            lon: 10.75,
        },
        City {
            name: "Vancouver",
            lat: 49.25,
            lon: -123.1,
        },
    ];
    cities.iter().for_each(|city| println!("{}", *city));

    let colors = [
        Color {
            red: 128,
            green: 255,
            blue: 90,
        },
        Color {
            red: 0,
            green: 3,
            blue: 254,
        },
        Color {
            red: 0,
            green: 0,
            blue: 0,
        },
    ];

    colors.iter().for_each(|color| println!("{:#?}", color));

    // Switch this to use {} once you've added an implementation
    // for fmt::Display
    colors.iter().for_each(|color| println!("{}", color));
}
