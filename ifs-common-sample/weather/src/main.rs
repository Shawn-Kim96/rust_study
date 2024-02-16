use std::f64::consts::PI;

struct GridConverter {
    olon: f64,
    xo: f64,
    yo: f64,
    sn: f64,
    sf: f64,
    ro: f64,
    re: f64,
    degrad: f64,
}

impl GridConverter {
    fn new() -> GridConverter {
        let nx = 149.0; // X축 격자점 수
        let ny = 253.0; // Y축 격자점 수

        let re = 6371.00877; // 지도반경
        let grid = 5.0; // 격자간격 (km)
        let mut slat1 = 30.0; // 표준위도 1
        let mut slat2 = 60.0; // 표준위도 2
        let mut olon = 126.0; // 기준점 경도
        let mut olat = 38.0; // 기준점 위도
        let xo = 210.0 / grid; // 기준점 X좌표
        let yo = 675.0 / grid; // 기준점 Y좌표

        let degrad = PI / 180.0;

        let re = re / grid;
        slat1 *= degrad;
        slat2 *= degrad;
        olon *= degrad;
        olat *= degrad;

        let sn = (PI * 0.25 + slat2 * 0.5).tan() / (PI * 0.25 + slat1 * 0.5).tan();
        let sn = (slat1.cos() / slat2.cos()).ln() / sn.ln();
        let sf = (PI * 0.25 + slat1 * 0.5).tan().powf(sn) * slat1.cos() / sn;
        let ro = re * sf / (PI * 0.25 + olat * 0.5).tan().powf(sn);

        GridConverter {
            olon,
            xo,
            yo,
            sn,
            sf,
            ro,
            re,
            degrad,
        }
    }

    fn map_to_grid(&self, lat: f64, lon: f64, _code: i32) -> (i32, i32) {
        let mut ra = (PI * 0.25 + lat * self.degrad * 0.5).tan();
        ra = self.re * self.sf / ra.powf(self.sn);
        println!("{}, {}", ra, self.sn);


        let mut theta = lon * self.degrad - self.olon;
        if theta > PI {
            theta -= 2.0 * PI;
        }
        if theta < -PI {
            theta += 2.0 * PI;
        }
        theta *= self.sn;
        println!("{}", self.ro);
        let x = (ra * theta.sin()) + self.xo;
        let y = (self.ro - ra * theta.cos()) + self.yo;
        let x = (x + 1.5).round() as i32;
        let y = (y + 1.5).round() as i32;
        (x, y)
    }
}

fn main() {
    let converter = GridConverter::new();
    let (x, y) = converter.map_to_grid(37.566, 126.9784, 0);
    println!("Grid: ({}, {})", x, y);
}
