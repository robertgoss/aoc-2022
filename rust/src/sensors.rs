use std::collections::BTreeSet;
use std::cmp::{min, max};

struct Ranges {
    spans : BTreeSet<(i64, i64)>
}

struct Box2 {
    xmin : i64,
    ymin : i64,
    xmax : i64,
    ymax : i64
}

#[derive(Debug)]
struct Sensor {
    pos : (i64, i64),
    dist : i64
}

pub struct Sensors {
    sensors : Vec<Sensor>
}

fn parse_pos(string : &str) -> Option<(i64, i64)> {
    let (x_str, y_str) = string.split_once(", ")?;
    let x_s = x_str.strip_prefix("x=")?;
    let x = x_s.parse::<i64>().ok()?;
    let y_s = y_str.strip_prefix("y=")?;
    let y = y_s.parse::<i64>().ok()?;
    Some((x,y))
}

impl Sensor {
    fn from_line(line : &str) -> Option<Sensor> {
        let (sensor_s, beacon_s) = line.split_once(":")?;
        let sensor_p_s = sensor_s.strip_prefix("Sensor at ")?;
        let sensor_p = parse_pos(sensor_p_s)?;
        let beacon_p_s = beacon_s.strip_prefix(" closest beacon is at ")?;
        let beacon_p = parse_pos(beacon_p_s)?;
        let dist = (sensor_p.0 - beacon_p.0).abs() + (sensor_p.1 - beacon_p.1).abs();
        Some(Sensor{
            pos : sensor_p,
            dist : dist
        })
    }

    fn span_at_row(&self, row_num : i64) -> Option<(i64, i64)> {
        let x_diff =  self.dist - (self.pos.1 - row_num).abs();
        if x_diff >= 0 {
            Some((self.pos.0 - x_diff, self.pos.0 + x_diff))
        } else {
            None
        }
    }

    fn contains(&self, b : &Box2) -> bool {
        self.contains_point(b.furthest(&self.pos))
    }

    fn overlap(&self, b : &Box2) -> bool {
        self.contains_point(b.nearest(&self.pos))
    }

    fn contains_point(&self, pt : (i64, i64)) ->bool {
        let dist =  (pt.0 - self.pos.0).abs() + (pt.1 - self.pos.1).abs();
        dist <= self.dist
    }

}


fn find_beacon_in_box(box2 : &Box2, sensors : &[&Sensor]) -> Option<(i64,i64)> {
    if box2.unit() {
        if sensors.is_empty() {
          return Some((box2.xmin,box2.ymin));
        } else {
          return None;
        }
    }
    for sensor in sensors {
        if sensor.contains(box2) {
            return None;
        }
    }
    for sub_box in box2.split() {
        let sub_sensors :Vec<&Sensor> = sensors.iter().filter(
            |sensor| sensor.overlap(&sub_box)
        ).cloned().collect();
        if let Some(pos) = find_beacon_in_box(&sub_box, &sub_sensors) {
            return Some(pos);
        }
    }
    None

}



impl Sensors {
    pub fn from_lines(lines : &Vec<String>) -> Sensors {
        let sensors = lines.iter().filter_map(
            |line| Sensor::from_line(line)
        ).collect();
        Sensors { sensors: sensors }
    }

    pub fn no_becons_row(&self, row_num : i64) -> usize {
        self.ranges_at_row(row_num).spans.iter().map(
            |(min, max)| (max-min) as usize
        ).sum()
    }

    fn ranges_at_row(&self, row_num : i64) -> Ranges {
        let mut range = Ranges { spans : BTreeSet::new() };
        for sensor in self.sensors.iter() {
            if let Some(span) = sensor.span_at_row(row_num) {
                range.merge(span);
            }
        }
        range
    }

    pub fn find_beacon(&self, xmin : i64, xmax : i64, ymin : i64, ymax : i64) -> Option<(i64,i64)> {
        let b = Box2{xmin : xmin, xmax : xmax, ymin: ymin, ymax : ymax};
        let sen_ref : Vec<&Sensor> = self.sensors.iter().collect();
        find_beacon_in_box(&b, &sen_ref)
    }
}

fn overlap((a_min, a_max) : &(i64, i64), (b_min, b_max) : &(i64, i64)) -> bool {
    a_min <= b_max && b_min <= a_max
}

impl Ranges {
    fn merge(&mut self, new_span : (i64, i64)) {
        let (mut span_min, mut span_max) = new_span;
        self.spans.drain_filter(
            |span| {
                if overlap(span, &new_span) {
                    span_min = min(span.0, span_min);
                    span_max = max(span.1, span_max);
                    true
                } else {
                    false
                }
            }
        );
        self.spans.insert((span_min, span_max));
    }
}

impl Box2 {
    fn unit(&self) -> bool {
        (self.xmin == self.xmax) && (self.ymin == self.ymax)
    }

    fn split(&self) -> [Box2; 2] {
        if (self.xmin - self.xmax).abs() > (self.ymin - self.ymax).abs() {
            self.split_x()
        } else {
            self.split_y()
        }
    }

    fn split_x(&self) -> [Box2; 2] {
        let xmid = (self.xmin + self.xmax) / 2;
        [
            Box2 { xmin : self.xmin, xmax : xmid, ymin : self.ymin, ymax : self.ymax},
            Box2 { xmin : xmid+1, xmax : self.xmax, ymin : self.ymin, ymax : self.ymax}
        ]
    }

    fn split_y(&self) -> [Box2; 2] {
        let ymid = (self.ymin + self.ymax) / 2;
        [
            Box2 { xmin : self.xmin, xmax : self.xmax, ymin : self.ymin, ymax : ymid},
            Box2 { xmin : self.xmin, xmax : self.xmax, ymin : ymid+1, ymax : self.ymax}
        ]
    }

    fn nearest(&self, pt : &(i64, i64)) -> (i64, i64) {
        let x = max(self.xmin, min(self.xmax, pt.0));
        let y = max(self.ymin, min(self.ymax, pt.1));
        (x, y)
    }

    fn furthest(&self, pt : &(i64, i64)) -> (i64, i64) {
        let x = if (pt.0 - self.xmin).abs() < (pt.0 - self.xmax).abs() {
            self.xmax
        } else {
            self.xmin
        };
        let y = if (pt.1 - self.ymin).abs() < (pt.1 - self.ymax).abs() {
            self.ymax
        } else {
            self.ymin
        };
        (x, y)
    }
}