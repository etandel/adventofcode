use std::cell::RefCell;
use std::collections::{HashMap, HashSet};
use std::env;
use std::fs;
use std::path::Path;
use std::rc::Rc;

type Graph<'a> = HashMap<&'a str, HashSet<&'a str>>;
type Counts<'a> = Rc<RefCell<HashMap<&'a str, usize>>>;

fn read_input<P>(path: P) -> String
where
    P: AsRef<Path>,
{
    fs::read_to_string(path).unwrap()
}

fn build_graph(input: &str) -> Graph {
    let mut g = Graph::new();

    let pairs = input.lines().map(|l| {
        let mut s = l.split('-');
        (s.next().unwrap(), s.next().unwrap())
    });

    for (from, to) in pairs {
        g.entry(from).or_insert_with(|| HashSet::new()).insert(to);
        g.entry(to).or_insert_with(|| HashSet::new()).insert(from);
    }

    g
}

fn is_small(s: &str) -> bool {
    s.chars().all(char::is_lowercase)
}

fn search<'a, 'b, F>(
    graph: &'a Graph,
    counts: Counts<'a>,
    n: &'a str,
    should_visit_small: &'static F,
) -> usize
where
    F: Fn(Counts<'a>, &'a str) -> bool,
    'a: 'b,
{
    if n == "end" {
        1
    } else {
        let mut count = 0;

        for &neighbor in graph.get(n).iter().flat_map(|&v| v.iter()) {
            if neighbor == "start" {
                continue;
            }

            if is_small(neighbor) {
                *counts.borrow_mut().entry(neighbor).or_insert(0) += 1;

                let should = should_visit_small(counts.clone(), neighbor);
                if should {
                    count += search(graph, counts.clone(), neighbor, should_visit_small);
                }
                *counts.borrow_mut().get_mut(neighbor).unwrap() -= 1;
            } else {
                count += search(graph, counts.clone(), neighbor, should_visit_small);
            }
        }

        count
    }
}

fn should_visit_small_1(visited: Counts, n: &str) -> bool {
    !visited.borrow().get(&n).map(|&v| v == 2).unwrap_or(true)
}

fn should_visit_small_2<'a>(counts: Counts<'a>, small_cavern: &'a str) -> bool {
    counts.borrow()[&small_cavern] == 1
        || counts
            .borrow()
            .iter()
            .all(|(&k, &c)| c < 2 || (k == small_cavern && c == 2))
}

fn part1() {
    let input = read_input("input.txt");
    let graph = build_graph(&input);

    let visited = Rc::new(RefCell::new(HashMap::new()));
    visited.borrow_mut().insert("start", 1);
    let count = search(&graph, visited, "start", &should_visit_small_1);

    println!("{}", count);
}

fn part2() {
    let input = read_input("input.txt");
    let graph = build_graph(&input);

    let visited = Rc::new(RefCell::new(HashMap::new()));
    visited.borrow_mut().insert("start", 1);
    let count = search(&graph, visited, "start", &should_visit_small_2);

    println!("{}", count);
}

fn main() {
    let args: Vec<String> = env::args().collect();
    match &args[1][..] {
        "1" => part1(),
        "2" => part2(),
        _ => println!("Must pass either '1' or '2'."),
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_() {}
}
