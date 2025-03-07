use std::vec;

type Point = (usize, usize);

const DIR: [(isize, isize); 4] = [(0, -1), (1, 0), (1, 0), (-1, 0)];

fn walk(
    maze: &Vec<Vec<char>>,
    wall: char,
    curr: Point,
    end: Point,
    seen: &mut Vec<Vec<bool>>,
    path: &mut Vec<Point>,
) -> bool {
    if curr.0 >= maze[0].len() || curr.1 >= maze.len() {
        return false;
    } else if maze[curr.1][curr.0] == wall {
        return false;
    } else if curr == end {
        path.push(end);
        return true;
    } else if seen[curr.1][curr.0] {
        return false;
    }

    seen[curr.1][curr.0] = true;
    path.push(curr);

    for (x, y) in DIR {
        if walk(
            maze,
            wall,
            ((curr.0 as isize + x) as usize, (curr.1 as isize + y) as usize),
            end,
            seen,
            path,
        ) {
            return true;
        }
    }

    path.pop();

    false
}

fn solve(maze: Vec<Vec<char>>, wall: char, start: Point, end: Point) -> bool {
    let mut seen: Vec<Vec<bool>> = vec![vec![false; maze[0].len()]; maze.len()];
    let mut path = vec![];

    walk(&maze, wall, start, end, &mut seen, &mut path)
}

#[cfg(test)]
mod tests {
    use crate::maze::solve;

    #[test]
    fn test_maze() {
        let maze = vec![
            vec!['#', '#', '#', '#', '#', 'E', '#'],
            vec!['#', ' ', ' ', ' ', ' ', ' ', '#'],
            vec!['#', 'S', '#', '#', '#', '#', '#'],
        ];

        assert!(
            solve(maze, '#', (1, 2), (5, 0))
        );
    }
}
