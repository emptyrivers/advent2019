
struct Point(i32, i32);

enum Segment {
    Horizontal {
        left: i32,
        right: i32,
        y: i32
    },
    Vertical {
        top: i32,
        bot: i32,
        x: i32
    }
}

fn create_segment(dat: &str, p: &mut Point) -> Segment {
    let direction = &dat[0..1];
    let len: i32 = dat[1..].parse().unwrap();
    match direction {
        "D" => {
            let new_segment = Segment::Vertical {
                top: p.1,
                bot: p.1 - len,
                x: p.0,
            };
            p.1 -= len;
            new_segment
        },
        "U" => {
            let new_segment = Segment::Vertical{
                top: p.1 + len,
                bot: p.1,
                x: p.0,
            };
            p.1 += len;
            new_segment
        },
        "R" => {
            let new_segment = Segment::Horizontal{
                left: p.0,
                right: p.0 + len,
                y: p.1,
            };
            p.0 += len;
            new_segment
        },
        "L" => {
            let new_segment = Segment::Horizontal{
                left: p.0 - len,
                right: p.0,
                y: p.1,
            };
            p.0 -= len;
            new_segment
        },
        _ => panic!("invalid segment found: {}, direction: {}", dat, direction)
    }
}

fn manhattan_dist(p: &Point) -> i32 {
    p.0.abs() + p.1.abs()
}

fn closest_approach(s: &Segment) -> i32 {
    match s {
        Segment::Vertical{top, bot, x,} => {
            if top.signum() != bot.signum() {
                x.abs()
            } else{
                x.abs() + std::cmp::min(top.abs(), bot.abs())
            }
        },
        Segment::Horizontal{left, right, y,} => {
            if left.signum() != right.signum() {
                y.abs()
            } else{
                y.abs() + std::cmp::min(left.abs(), right.abs())
            }
        }
    }
}


fn find_intersect(a: &Segment, b: &Segment) -> Result<Point, ()> {
    match a {
        Segment::Horizontal{left, right, y,} => match b {
            Segment::Horizontal{..} => Err(()),
            Segment::Vertical{top, bot, x,} => {
                if bot < y && y < top && left < x && x < right {
                    Ok(Point(*x, *y))
                } else {
                    Err(())
                }
            }
        },
        Segment::Vertical{top, bot, x,} => match b{
            Segment::Vertical{..} => Err(()),
            Segment::Horizontal{left, right, y,} => {
                if bot < y && y < top && left < x && x < right {
                    Ok(Point(*x, *y))
                } else {
                    Err(())
                }
            }
        }
    }
}

pub fn solve() {
    let data = super::get_data("03_wires.txt");
    let mut end = Point(0, 0);
    let (mut vert, mut horiz): (Vec<Segment>, Vec<Segment>) = data[0]
        .split(",")
        .map(|dat| create_segment(&dat, &mut end))
        .partition(|seg| match seg {
            Segment::Vertical{..} => true,
            Segment::Horizontal{..} => false,
        });
    println!("Vertical segments: {}, and horizontal: {}", vert.len(), horiz.len());
    end = Point(0, 0);
    let mut best_distance = 0;
    let mut best_inter = Point(0, 0);
    for seg in data[1].split(",") {
        let segment = create_segment(seg, &mut end);
        if best_distance == 0 || closest_approach(&segment) < best_distance {
            let mut update: bool = false;
            match segment {
                Segment::Horizontal{..} => {
                    for blue in &vert {
                        match find_intersect(&blue, &segment) {
                            Ok(p) => {
                                let dist = manhattan_dist(&p);
                                if best_distance == 0 || dist < best_distance {
                                    best_distance = dist;
                                    best_inter = p;
                                    update = true;
                                }
                            },
                            Err(_) => ()
                        }
                    }
                },
               Segment::Vertical{..} => {
                    for blue in &horiz {
                        match find_intersect(&blue, &segment) {
                            Ok(p) => {
                                let dist = manhattan_dist(&p);
                                if best_distance == 0 || dist < best_distance {
                                    best_distance = dist;
                                    best_inter = p;
                                    update = true;
                                }
                            },
                            Err(_) => ()
                        }
                    }
                }
            }
            if update {
                horiz = horiz.into_iter().filter(|seg| closest_approach(&seg) < best_distance).collect();
                vert = vert.into_iter().filter(|seg| closest_approach(&seg) < best_distance).collect();
            }
        }
    }
    println!("Best possible intersection is at ({},{}), dist = {}", best_inter.0, best_inter.1, best_distance); 

}