# TUI Music Player written in rust

A very basic TUI music player written in rust that is compatible with Subsonic API

Currently under development

## TODOs:

### Done
- [x] Build a very basic frontend (for now just login)
- [x] Create Action enum to pass data between music core library and the frontend
- [x] Create App struct in the music_core library for the frontend to call
- [x] Generic Subsonic response envelope + `call` helper
- [x] `getPlaylists`
- [x] `getArtists`
- [x] `getGenres`

### Fixes
- [x] `getGenres` parses the wrong key: the response is `{"genres": {"genre": [...]}}`, so `GenresList.genres` should be `genre` (currently always returns an empty list)
- [ ] `ArtistsFailed` writes to `state.playlists` instead of `state.artist_index`
- [ ] Rename `GenreFailed` -> `GenresFailed` to match the other actions

### music_core: drill-down (getting to actual songs)
- [ ] `Song` model (id, title, artist, album, duration, track, coverArt, ...)
- [ ] `Album` model
- [ ] `getArtist(id)` -> albums of an artist
- [ ] `getAlbum(id)` -> songs on an album
- [ ] `getPlaylist(id)` -> songs in a playlist
- [ ] `getSongsByGenre(genre)` -> songs in a genre
- [ ] `getAlbumList2` (recent / newest / random / frequent) for a home view

### music_core: playback
- [ ] Build the `stream` URL for a song (auth params as query string)
- [ ] Pick an audio backend (e.g. `rodio`) and run it on its own thread
- [ ] Play queue in `AppState` (queue, current index, playing/paused, position)
- [ ] Actions: Play, Pause/Resume, Next, Previous, Seek, Enqueue, ClearQueue
- [ ] `scrobble` when a track is played

### music_core: cleanup
- [ ] Factor the repeated "spawn thread, call subsonic, send result" block in `update.rs` into a helper
- [ ] Reuse one `reqwest::blocking::Client` instead of creating one per request
- [ ] Logout action (clear saved config)

### Later
- [ ] `search3`
- [ ] Starred / favourites (`getStarred2`, `star`, `unstar`)
- [ ] Cover art (`getCoverArt`)
