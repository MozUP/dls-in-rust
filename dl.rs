use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
#[allow(unused_variables)]

fn dls(x : f32, score_1: i32, score_2 : i32) -> i32 {

let score_1 = score_1 as f32;
let score_2  = score_2 as f32;
let mut to_win = (((100.0-x)*score_1)/100.0) as i32;

to_win = to_win+1;

return to_win;


}


fn main() {
    // Create a path to the desired file
    let path = Path::new("dataset.txt");
    let display = path.display();

    // Open the path in read-only mode, returns `io::Result<File>`
    let mut file = match File::open(&path) {
        // The `description` method of `io::Error` returns a string that
        // describes the error
        Err(why) => panic!("couldn't open {}: {}", display,
                                                   why.description()),
        Ok(file) => file,
    };

    // Read the file contents into a string, returns `io::Result<usize>`
    let mut s = String::new();
    match file.read_to_string(&mut s) {
        Err(why) => panic!("couldn't read {}: {}", display,
                                                   why.description()),
        Ok(_) => println!(""),
    }
    let outer = s.split("\n"); // split the string based on new line

    let mut stats = vec![]; // create an empty vector to store values

    for over in outer{

    let inner = over.split(","); // split the inner lines by commas

    let mut vec = inner.collect::<Vec<&str>>(); // collect the ratio values in a vector

    vec.reverse();
    stats.push(vec); // push that vector to a parent vector

    }

   stats.reverse(); // reverse the vector to make more sense while applying logic

   let india_score = 300;  // indian team's score in 50 overs
   let pakistan_score = 200; // pakistan's score when rain stopped play
   let pakistan_wickets = 7; // pakistan's wickets
   let over_bowled = 30; // over's bowled
   let resource_p : f32 = (stats[50-over_bowled][9-pakistan_wickets]).parse().unwrap();
   let score = dls(resource_p,india_score,pakistan_score);

   println!("Pakistan need {} runs to win after rain.",score);
    // `file` goes out of scope, and the "hello.txt" file gets closed
}
