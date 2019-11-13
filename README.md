[![Latest Version]][crates.io]
# spotify-genres-rs
Analysing spotify genres in a playlist

# Building a histogram of used genres
Things you need to get started: 
* Register a new application at [Spotify's dashboard](https://developer.spotify.com/dashboard)
* Get the `client_id` and `client_secret`
* Get the playlist's `id`. You can find it it its uri: spotify:playlist:`37i9dQZEVXbMDoHDwVN2tF`

## Example code
````rust
use spotify_genres::{auth_spotify, get_genres_for_playlist};

fn main() {
    let top_50 = vec![
        "37i9dQZEVXbMDoHDwVN2tF", // GLOBAL
        "37i9dQZEVXbJiZcmkrIHGU", // GERMANY
        "6VZ7JY80Iy1wy7GF076AMo", // NORWAY
    ];
    let spotify = auth_spotify("id", "secret");
    for playlist in top_50 {
        if let Ok(res) = get_genres_for_playlist(&spotify, playlist) {
            let str: Vec<String> = res.iter().map(|p| format!("{}: {}", p.0, p.1)).collect();
            println!("{} -> {:#?}", playlist, str);
        } else {
            eprintln!("error in analysing: {}", playlist);
        }
    }
}
````

[crates.io]: https://crates.io/crates/spotify-genres
[Latest Version]: https://img.shields.io/crates/v/spotify-genres.svg


