use histogram::Histogram;
use rspotify::spotify::client::Spotify;
use rspotify::spotify::oauth2::SpotifyClientCredentials;
use std::collections::HashMap;
use std::error::Error;

/// Authenticates a `Spotify` client with the spotify api.
/// `client_id` and `client_secret` can be gathered from spotify's developer workspace by registering an app.
pub fn auth_spotify(client_id: &str, client_secret: &str) -> Spotify {
    let client_credential = SpotifyClientCredentials::default()
        .client_id(client_id)
        .client_secret(client_secret)
        .build();

    Spotify::default()
        .client_credentials_manager(client_credential)
        .build()
}

/// Calculates a histogram of the genres in the playlist with id `playlist`.
/// Returns a vector of pairs (genre_name: `String`, amount: `u64`).
/// Needs a `Spotify` client.
pub fn get_genres_for_playlist(
    client: &Spotify,
    playlist: &str,
) -> Result<Vec<(String, u64)>, Box<dyn Error>> {
    let playlist = client.playlist(playlist, None, None)?;
    let track_page = playlist.tracks;
    let tracks = track_page.items;

    let mut histogram = Histogram::new();
    let mut number_genre: HashMap<String, u64> = HashMap::new();

    for track in tracks {
        for art in track.track.artists {
            if let Some(artist_id) = art.id {
                let full = client.artist(artist_id.as_str())?;
                for g in full.genres {
                    let genre_len = number_genre.len();
                    let number = match number_genre.get(&g) {
                        Some(num) => *num,
                        None => {
                            let len: u64 = (genre_len + 1) as u64;
                            number_genre.insert(g, len);
                            len
                        }
                    };
                    histogram.increment(number)?;
                }
            }
        }
    }

    let mut res: Vec<(String, u64)> = vec![];
    for i in number_genre {
        let genre = i.0;
        let amount = histogram.get(i.1);
        if let Some(a) = amount {
            res.push((genre, a));
        }
    }

    res.sort_by(|v1, v2| v2.1.cmp(&v1.1));

    Ok(res)
}
