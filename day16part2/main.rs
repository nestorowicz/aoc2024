use std::{collections::{HashMap, HashSet}, io::{read_to_string, stdin}};
use aoc::{Point, Map, Direction, Ray, debug, is_debug};

const START: char = 'S';
const END: char = 'E';
const WALL: char = '#';
const INITIAL_DIRECTION: Direction = Direction::Right;
const COST_MOVE: u64 = 1;
const COST_ROTATE: u64 = 1000;

struct Input {
    map: Map<char>,
    start: Point,
    end: Point
}

fn main() {
    let input = parse_input();
    if is_debug() { aoc::print_char_map(&input.map); }

    let initial_ray = Ray::new(input.start, INITIAL_DIRECTION);
    let graph = find_shortest_paths(&input, initial_ray);
    debug!("graph \n{:?}\n", graph);

    let points = points_in_path(&graph, input.end);
    debug!("best path\n {:?}\n\n", points);

    if is_debug() {
        let mut map = input.map.clone();
        for tile in  points.iter() {
            map.state.get_mut(tile.y).unwrap()[tile.x] = 'O';
        }
        aoc::print_char_map(&map);
    }
    println!("tiles in cheapest paths {}", points.len() + 1);
}

fn parse_input() -> Input {
    let map = Map{state:
        read_to_string(stdin()).unwrap()
            .lines()
            .map(|s| s.chars().collect())
            .collect()
    };
    let start = map.find_position(&START).unwrap();
    let end = map.find_position(&END).unwrap();
    return Input{map, start, end};
}

fn pop_shortest_from_queue(queue: &mut HashSet<Ray>, distances: &HashMap<Ray, u64>) -> Option<(Ray, u64)> {
    let (shortest, distance) = queue.iter()
        .map(|ray| (ray, distances.get(&ray)))
        .filter_map(|(ray, distance)| Some((ray, distance?)))
        .min_by(|(_, distance), (_, distance2)| distance.cmp(distance2))
        .map(|(ray, distance)| (ray.to_owned(), distance.to_owned()))?;
    queue.remove(&shortest);
    return Some((shortest, distance));
}

fn find_shortest_paths(input: &Input, initial_ray: Ray) -> HashMap<Point, HashMap<Ray, Vec<Ray>>> {
    let map = &input.map;
    // prev is a map of points to the rays to point to them and their previous nodes.
    let mut prev: HashMap<Point, HashMap<Ray, Vec<Ray>>> = HashMap::new();
    let mut distances: HashMap<Ray, u64> = HashMap::from([(initial_ray, 0)]);
    let mut queue: HashSet<Ray> = HashSet::from([initial_ray]);
    let mut min_cost: Option<u64> = None;
    
    loop {
        let Some((current_ray, current_distance)) = pop_shortest_from_queue(&mut queue, &distances) else { break };
        if min_cost.is_some() && min_cost.unwrap() < current_distance {
            break;
        }
        if current_ray.point == input.end {
            debug!("Found min cost {:?}", current_distance);
            min_cost = Some(current_distance);
        }
        let neighbors = vec![
            (current_ray.move_by_one(), COST_MOVE),
            (current_ray.rotate_90_clockwise().move_by_one(), COST_MOVE + COST_ROTATE),
            (current_ray.rotate_90_counter_clockwise().move_by_one(), COST_MOVE + COST_ROTATE)
        ];
        for (neighbor, cost) in neighbors.into_iter() {
            let Some(neighbor) = neighbor else { continue };
            let cost = cost + current_distance;
            // if not a valid point (hit a wall), continue
            if map.peek(&neighbor.point).filter(|c| **c != WALL).is_none() {
                continue
            }
            // if we haven't seen that neighbor yet, add it to the queue
            if !queue.contains(&neighbor) && !distances.contains_key(&neighbor) {
                debug!("adding to queue {:?}", neighbor);
                queue.insert(neighbor.clone());
            }
            
            let prev_cost = distances.get(&neighbor);
            debug!("neighbor {:?} cost {:?} prev_cost {:?}", neighbor, cost, prev_cost);

            if prev.get(&neighbor.point).is_none() {
                prev.insert(neighbor.point, HashMap::new());
            }

            // if we found cheaper way to the neighbor, replace the prev node with the current node
            if prev_cost.is_none() {
                debug!("first path {:?} cost {:?} prev_cost {:?}", neighbor, cost, prev_cost);
                prev.get_mut(&neighbor.point).unwrap().insert(neighbor, vec![current_ray]);
                distances.insert(neighbor, cost);
                continue;
            }

            // if the cost is equal, add to the prev array
            if prev_cost.is_some() && cost < *prev_cost.unwrap(){
                debug!("Overriding prev neighbor {:?} cost {:?} prev_cost {:?}", neighbor, cost, prev_cost);
                prev.insert(neighbor.point, HashMap::from([(neighbor, vec![current_ray])]));
                distances.insert(neighbor, cost);
                continue;
            }

            // if the cost is equal, add to the prev array
            if prev_cost.is_some() && *prev_cost.unwrap() == cost {
                debug!("Pushing alternative path to neighbor {:?}", neighbor);
                prev.get_mut(&neighbor.point).unwrap().get_mut(&neighbor).unwrap().push(current_ray);
            }
        }

        debug!("Visited {:?} distance {:?} prev {:?} \n", current_ray, current_distance, prev.get(&current_ray.point));
    }

    let paths = prev.get_mut(&input.end).unwrap();
    let expensive_paths: Vec<Ray> = paths.keys()
        .filter(|k| distances[k] > min_cost.unwrap())
        .map(|k| *k)
        .collect();

    expensive_paths.iter().for_each(|r| _ = paths.remove(r));
    return prev;
}

fn points_in_path(graph: &HashMap<Point, HashMap<Ray, Vec<Ray>>>, tile: Point) -> HashSet<Point> {
    let rays: HashSet<Ray> = graph
        .get(&tile)
        .map(|m| m.keys().map(|r| *r).collect())
        .unwrap_or(HashSet::new());
    let prev: HashSet<Point> = rays.iter().flat_map(|ray| get_prev_points(graph, *ray)).collect();
    return HashSet::from([tile]).union(&prev).map(|p| *p).collect();
}

fn get_prev_points(graph: &HashMap<Point, HashMap<Ray, Vec<Ray>>>, ray: Ray) -> HashSet<Point> {
    let initial = HashSet::from([ray.point]);
    let Some(edges) = graph.get(&ray.point) else { return initial };
    let Some(prev_rays) = edges.get(&ray) else { return initial };
    let prev_points: HashSet<Point> = prev_rays.iter().flat_map(|ray| get_prev_points(graph, *ray)).collect();
    return initial.union(&prev_points).map(|p| *p).collect();
}
