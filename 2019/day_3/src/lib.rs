type PointList = Vec<(i32, i32)>;

pub fn str_to_coor(input: &str) -> PointList {
    let mut output = Vec::new();
    // Starting off at the middle
    output.push((0, 0));

    let mut cur = (0, 0);
    for section in input.split(",") {
        let dir = &section[0..1];
        let amount: i32 = section[1..].parse().expect("Invalid input");
        match dir {
            "R" => cur.0 += amount,
            "U" => cur.1 += amount,
            "L" => cur.0 -= amount,
            "D" => cur.1 -= amount,
            _ => panic!("Invalid input"),
        }
        // Since cur is only composed of primitives this shouldn't take ownership and should just copy
        output.push(cur);
    }

    output
}

pub fn find_collisions(a: &PointList, b: &PointList) -> Vec<(i32, i32, u32, u32)> {
    let mut output = Vec::new();

    // For every line in A, checking if any line in b overlaps
    let mut line_dist: u32 = 0;
    for line in a.windows(2) {
        // If both x values are equal, the line must be vertical, with y differing
        // 0 is x, 1 is y.
        let line_hor = line[0].0 != line[1].0;
        let mut cross_line_dist: u32 = 0;
        for cross_line in b.windows(2) {
            let cross_hor = cross_line[0].0 != cross_line[1].0;
            if (line_hor != cross_hor) && (line[0].0 != 0 && line[0].1 != 0 && cross_line[0].0 != 0 && cross_line[0].1 != 0)  {
                // Making sure both lines don't point in the same direction, or they don't both begin at 0, 0
                let ((x_min, x_max), other_x, (y_min, y_max), other_y) =
                    if line_hor {
                        (minmax(line[0].0, line[1].0),
                        cross_line[0].0,
                        minmax(cross_line[0].1, cross_line[1].1),
                        line[0].1)
                    } else {
                        (minmax(cross_line[0].0, cross_line[1].0),
                        line[0].0,
                        minmax(line[0].1, line[1].1),
                        cross_line[0].1)
                };
                if other_x >= x_min && other_x <= x_max && other_y >= y_min && other_y <= y_max {
                    // Found a match
                    output.push(
                        if line_hor {
                            (
                                cross_line[0].0, line[0].1,
                                //line_dist + cross_line_dist + (other_x - x_min + other_y - y_min) as u32
                                line_dist + (other_x - line[0].0).abs() as u32,
                                cross_line_dist + (other_y - cross_line[0].1).abs() as u32 
                            )
                        }
                        else {
                            (
                                line[0].0, cross_line[0].1,
                                //line_dist + cross_line_dist + (other_x - x_min + other_y - y_min) as u32
                                line_dist + (other_y - line[0].1).abs() as u32,
                                cross_line_dist + (other_x - cross_line[0].0).abs() as u32
                            )
                        }
                    )
                }
            }
            
            if cross_hor {
                cross_line_dist += (cross_line[0].0 - cross_line[1].0).abs() as u32;
            }
            else {
                cross_line_dist += (cross_line[0].1 - cross_line[1].1).abs() as u32;
            }
        }

        if line_hor {
            line_dist += (line[0].0 - line[1].0).abs() as u32;
        }
        else {
            line_dist += (line[0].1 - line[1].1).abs() as u32;
        }
    }

    output
}

fn minmax<T: PartialOrd + Copy>(a: T, b: T) -> (T, T) {
    if a < b {
        (a, b)
    }
    else {
        (b, a)
    }
}