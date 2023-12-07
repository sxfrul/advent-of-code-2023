use std::fs;

fn main() {
    // --snip--
    let input = fs::read_to_string("input.txt")
        .expect("Should have been able to read the file");

    let line_split: Vec<&str> = input.lines().collect();
    let mut sum_validity = 0;
    for line in line_split {

        let mut validity = 1;
        //println!("{}", line);
        let game_split: Vec<&str> = line.split(": ").collect();
        let game_overview = format!("{}", game_split[0]);

        let game_overview2: Vec<&str> = game_overview.split(" ").collect();
        let game_ID = format!("{}", game_overview2[1]).parse::<i32>().unwrap();


        let set_overview = format!("{}", game_split[1]);
        println!("Playing: {} || ID: {} || Set Overview: {}", game_overview, game_ID, set_overview);

        let set_split: Vec<&str> = set_overview.split("; ").collect();
        for set in set_split{
            //println!("SET: {}", set); //PRINTS WHAT IS INSIDE A SET

            let ball_forset: Vec<&str> = set.split(", ").collect();
            for ball in ball_forset{
                let balls = format!("{}", ball);
                //println!("{}", balls);
    
                if balls.contains("blue") {
                    let number_color_split: Vec<&str> = balls.split(" ").collect();
                    let value_key = format!("{}", number_color_split[0]);
                    let value = value_key.parse::<i32>().unwrap();
                    //println!("This set contains {} blue balls!", value);
                    if value > 14{
                        //println!{"INVALID!"};
                        validity = 0;
                    }
                }
                if balls.contains("red") {
                    let number_color_split: Vec<&str> = balls.split(" ").collect();
                    let value_key = format!("{}", number_color_split[0]);
                    let value = value_key.parse::<i32>().unwrap();
                    //println!("This set contains {} red balls!", value);
                    if value > 12{
                        //println!{"INVALID!"};
                        validity = 0;
                    }
                }
                if balls.contains("green") {
                    let number_color_split: Vec<&str> = balls.split(" ").collect();
                    let value_key = format!("{}", number_color_split[0]);
                    let value = value_key.parse::<i32>().unwrap();
                    //println!("This set contains {} green balls!", value);
                    if value > 13{
                        //println!{"INVALID!"};
                        validity = 0;
                    }
                }
            }
        }
        println!("Validity: {}", validity);
        if validity > 0 {
            sum_validity += game_ID;
        }
    }
    println!("Result: {}", sum_validity)
}