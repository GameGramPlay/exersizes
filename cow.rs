fn main() {
    let tbl_breed = [
        "Red Chittagong", "Sussex", "Dexter", "Abondance",
        "Sahiwal", "Vorderwald", "Ayrshire", "Jersey",
        "Randall", "Alderney", "Carora", "Gloucester"
    ];

    let tbl_rating = [1, 2, 3, 2, 3, 1, 2, 1, 2, 1, 3, 2];
    let tbl_count = [6, 3, 8, 7, 6, 4, 3, 7, 3, 3, 4, 7];
    let tbl_volume = [7.5, 5.7, 11.4, 11.4, 22.0, 15.2, 21.0, 18.3, 19.0, 9.0, 23.1, 16.0];
    let mut tbl_dailyVolume: Vec<f64> = Vec::new();

    println!("Fields are Breed, Rating, Volume (cow), Count, Volume (day)");
    let mut counter = 0;
    let mut total_volume = 0;

    while counter < tbl_breed.len() {
        let daily_volume = tbl_volume[counter] * tbl_count[counter] as f64;
        print!("{}", tbl_breed[counter]);
        print!(" {}", tbl_rating[counter]);
        print!(" {}", tbl_volume[counter]);
        print!(" {}", tbl_count[counter]);
        print!(" {:.1}", tbl_volume[counter]);
        println!();
        counter += 1;
    };

    // ----------------------------------------------
    // Write your code below this line
}
