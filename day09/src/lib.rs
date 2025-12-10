use std::{array, iter::Peekable, slice, vec};

type Point = [i64; 2];
type PointF = [f64; 2];

fn parse_input(input_str: &String) -> Vec<Point> {
    input_str
        .lines()
        .map(|line| {
            let mut iter = line.split(',')
                .map(|n| n.parse().unwrap());
            array::from_fn(|_| iter.next().unwrap())
        })
        .collect()
}

fn part1(input: Vec<Point>) -> i64 {
    input.iter().enumerate().flat_map(|(i, p)| {
        input.iter().skip(i + 1).map(|q| {
            ((p[0] - q[0]).abs() + 1) * ((p[1] - q[1]).abs() + 1)
        })
    }).max().unwrap()
}

type Edge<'a> = [&'a Point; 2];
type EdgeF = [PointF; 2];

struct EdgeIter<'a> {
    head: &'a Point,
    i: std::iter::Peekable<std::slice::Iter<'a, Point>>
}

fn new_edge_iter(points: &Vec<Point>) -> EdgeIter {
    EdgeIter { head: &points[0], i: points.iter().peekable() }
}

impl<'a> Iterator for EdgeIter<'a> {
    type Item = Edge<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        self.i.next().map(|p| {
            match self.i.peek() {
                Some(q) => [p, *q],
                None => [p, self.head]
            }
        })
    }
}

fn is_vertical(e: &Edge) -> bool {
    e[0][0] == e[1][0]
}

fn _crossing(vertical: &Edge, horizontal: &Edge) -> bool {
    let y_axis = (vertical[0][1] - horizontal[0][1]) * (vertical[1][1] - horizontal[0][1]) < 0;
    let x_axis = (horizontal[0][0] - vertical[0][0]) * (horizontal[1][0] - vertical[0][0]) < 0;
    x_axis && y_axis
}

fn crossing(a: &Edge, b: &Edge) -> bool {
    match (is_vertical(a), is_vertical(b)) {
        (true, true) => false,
        (false, false) => false,
        (true, false) => _crossing(a, b),
        (false, true) => _crossing(b, a),
    }
}

fn is_vertical_f(e: &EdgeF) -> bool {
    (e[0][0] - e[1][0]).abs() < 0.0001
}

fn _crossing_f(vertical: &EdgeF, horizontal: &EdgeF) -> bool {
    let y_axis = (vertical[0][1] - horizontal[0][1]) * (vertical[1][1] - horizontal[0][1]) < -0.001;
    let x_axis = (horizontal[0][0] - vertical[0][0]) * (horizontal[1][0] - vertical[0][0]) < -0.001;
    x_axis && y_axis
}

fn point_f_ify(p: &Point) -> PointF {
    return [p[0] as f64, p[1] as f64]
}

fn edge_f_ify(e: &Edge) -> EdgeF {
    return [point_f_ify(e[0]), point_f_ify(e[1])]
}

fn crossing_f(a: &EdgeF, b: &Edge) -> bool {
    match (is_vertical_f(a), is_vertical(b)) {
        (true, true) => false,
        (false, false) => false,
        (true, false) => _crossing_f(a, &edge_f_ify(b)),
        (false, true) => _crossing_f(&edge_f_ify(b), a),
    }
}

fn part2(input: Vec<Point>) -> i64 {
    input.iter().enumerate().flat_map(|(i, p)| {
        input.iter().skip(i + 1).flat_map(|q| {
            let top = p[0].min(q[0]) as f64 + 0.5;
            let left = p[1].min(q[1]) as f64 + 0.5;
            let bottom = p[0].max(q[0]) as f64 - 0.5;
            let right = p[1].max(q[1]) as f64 - 0.5;

            let corners: &[EdgeF] = &[
                [[top, left], [top, 0.0]], 
                [[top, right], [top, 0.0]], 
                [[bottom, left], [bottom, 0.0]], 
                [[bottom, right], [bottom, 0.0]], 
            ];

            let outside = corners.iter().any(|c| {
                new_edge_iter(&input).filter(|e| crossing_f(c, e)).count() % 2 == 0
            });
            if outside {
                return None;
            }

            let edges: &[Edge] = &[
                [&[p[0], p[1]], &[p[0], q[1]]], 
                [&[p[0], p[1]], &[q[0], p[1]]], 
                [&[p[0], q[1]], &[q[0], q[1]]], 
                [&[q[0], p[1]], &[q[0], q[1]]], 
            ];
            
            let crossed = new_edge_iter(&input).flat_map(|b| edges.iter().map(move |e| (b, e)))
                .any(|(b, e)| crossing(&b, e));
            if crossed {
                None
            } else {
                Some(((p[0] - q[0]).abs() + 1) * ((p[1] - q[1]).abs() + 1))
            }
        })
    }).max().unwrap()
}

#[cfg(test)]
mod tests {
    use std::fs;
    use test_case::test_case;

    use crate::parse_input;

    #[test_case("input/sample_input.txt", 50)]
    #[test_case("input/input.txt", 4739623064)]
    fn test_part1(input_file: &str, epxected: i64) {
        let input_str = fs::read_to_string(input_file).unwrap();
        let input = parse_input(&input_str);
        let result = crate::part1(input);

        assert_eq!(result, epxected);
    }

    #[test_case("input/sample_input.txt", 24)]
    #[test_case("input/input.txt", 0)]
    fn test_part2(input_file: &str, epxected: i64) {
        let input_str = fs::read_to_string(input_file).unwrap();
        let input = parse_input(&input_str);
        let result = crate::part2(input);

        assert_eq!(result, epxected);
    }
}
