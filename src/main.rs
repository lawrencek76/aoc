use std::fs;

fn main() {
    let input = read_input().unwrap();
    //loop over each line in the input and print the first and last number
    let mut sum = 0;
    let mut powersum = 0;
    for line in input.lines() {
        let game_result = parse_game_result(&line).unwrap();
        match is_possible(&game_result, Draw{red: 12, green: 13, blue: 14}) {
            true => {
                println!("Possible: {:?}", game_result);
                sum = sum + game_result.id;
            }
            false => {
                println!("Not Possible: {:?}", game_result);
            }
        }
        powersum = powersum + (game_result.min_cubes.red * game_result.min_cubes.blue * game_result.min_cubes.green);
    }
    println!("Sum: {}, PowerSum {}", sum, powersum);
}

fn is_possible(game_result: &GameResult, max_draw: Draw) -> bool {
    // foreach reveal check if maxDraw is less than or equal to the reveal
    for reveal in &game_result.reveal {
        if max_draw.red < reveal.red || max_draw.blue < reveal.blue || max_draw.green < reveal.green {
            return false;
        }
    }
    true
}

fn read_input() -> std::io::Result<String> {
    fs::read_to_string("./src/input-2")
}

fn parse_game_result(line: &str) -> Option<GameResult> {
    //split the line into a vector of strings on :
    let parts: Vec<&str> = line.split(":").collect();
    if parts.len() != 2 {
        return None;
    }
    let id = parts[0].replace("Game ", "").parse::<i32>().unwrap();
    let mut game_result = GameResult{id: id, reveal: vec![], min_cubes: Draw{red: 0, blue: 0, green: 0}};
    //split the second part into a vector of strings on ;
    let draws: Vec<&str> = parts[1].split(";").collect();
    for draw in draws {
        let cubes: Vec<&str> = draw.split(",").collect();
        let mut draw = Draw{red: 0, blue: 0, green: 0};
        for cube in cubes {
             // split on space to get the color and the number
            let data: Vec<&str> = cube.split_whitespace().collect();
            let value = data[0].parse::<i32>().unwrap();
            let name = data[1];
            // add to reveal
            if name == "red" {
                draw.red = value;
            } else if name == "blue" {
                draw.blue = value;
            } else if name == "green" {
                draw.green = value;
            }
        }
        if draw.red > game_result.min_cubes.red {
            game_result.min_cubes.red = draw.red;
        }
        if draw.blue > game_result.min_cubes.blue {
            game_result.min_cubes.blue = draw.blue;
        }
        if draw.green > game_result.min_cubes.green {
            game_result.min_cubes.green = draw.green;
        }
        game_result.reveal.push(draw);
    }
    
    Some(game_result)
}

#[derive(Debug)]
struct GameResult {
    id: i32,
    reveal: Vec<Draw>,
    min_cubes: Draw,
}

#[derive(Debug)]
struct Draw {
    red: i32,
    blue: i32,
    green: i32,
}