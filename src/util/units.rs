use std::collections::HashMap;

pub fn get_squares() -> Vec<String> {
    let rows = "ABCDEFGHI".chars().collect::<Vec<_>>();
    let cols = "123456789".chars().collect::<Vec<_>>();

    cross(&rows, &cols)
}

pub fn get_units() -> HashMap<String, Vec<Vec<String>>> {
    let rows = "ABCDEFGHI".chars().collect::<Vec<_>>();
    let cols = "123456789".chars().collect::<Vec<_>>();
    let squares = get_squares();

    let mut units = HashMap::new();
    for s in &squares {
        let (row, col) = s.split_at(1);
        let row_units = cross(&[row.chars().next().unwrap()], &cols);
        let col_units = cross(&rows, &[col.chars().next().unwrap()]);

        let box_units = cross(
            match row {
                "A" | "B" | "C" => &['A', 'B', 'C'],
                "D" | "E" | "F" => &['D', 'E', 'F'],
                "G" | "H" | "I" => &['G', 'H', 'I'],
                _ => &['A', 'B', 'C'],
            },
            match col {
                "1" | "2" | "3" => &['1', '2', '3'],
                "4" | "5" | "6" => &['4', '5', '6'],
                "7" | "8" | "9" => &['7', '8', '9'],
                _ => &['1', '2', '3'],
            },
        );

        units.insert(
            s.clone(),
            vec![row_units, col_units, box_units],
        );
    }
    units
}

pub fn cross(a: &[char], b: &[char]) -> Vec<String> {
    let mut result = Vec::new();
    for &i in a {
        for &j in b {
            result.push(format!("{}{}", i, j));
        }
    }
    result
}