use spotify_genres::{auth_spotify, get_genres_for_playlist};

fn main() {
    let top_50 = vec![
        "37i9dQZEVXbMDoHDwVN2tF", // GLOBAL
        "37i9dQZEVXbJiZcmkrIHGU", // GERMANY
        "6VZ7JY80Iy1wy7GF076AMo", // NORWAY
    ];
    let spotify = auth_spotify("ex", "example");
    for playlist in top_50 {
        if let Ok(res) = get_genres_for_playlist(&spotify, playlist) {
            let str: Vec<String> = res.iter().map(|p| format!("{}: {}", p.0, p.1)).collect();
            println!("{} -> {:#?}", playlist, str);
        } else {
            eprintln!("error in analysing: {}", playlist);
        }
    }
}
