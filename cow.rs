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
    let mut total_volume: f64 = 0.0;
    let mut best_counter = 0;
    let mut best_volume = tbl_volume[counter];
    let mut best_rating = tbl_rating[counter];

    for (counter, breed) in tbl_breed.iter().enumerate() {
        let daily_volume = tbl_volume[counter] * tbl_count[counter] as f64;
        total_volume += daily_volume;
        println!(
            "{} {:.1} {:.1} {:.1} {:.1}", 
            tbl_breed[counter], tbl_rating[counter], tbl_volume[counter], tbl_count[counter], daily_volume
        );
        if tbl_rating[counter] <= best_rating && tbl_volume[counter] > best_volume {
            best_counter = counter;
            best_volume = tbl_volume[counter];
            best_rating = tbl_rating[counter];
        }

    };
    println!("Total: {} litres", total_volume);
    println!(
        "Recommended breed: {} rating: {} volume: {}",
        tbl_breed[best_counter], best_rating, best_volume
    );
    // ----------------------------------------------
    // Write your code below this line
}
