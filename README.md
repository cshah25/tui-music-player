# TUI Music Player written in rust

A very basic TUI music player written in rust that is compatible with Subsonic API

Currently under development

Link: [https://git.vtechno.ca/cshah25/tui-music-player](https://git.vtechno.ca/cshah25/tui-music-player)

## TODOs:

### Done
- [x] Build a very basic frontend (for now just login)
- [x] Create Action enum to pass data between music core library and the frontend
- [x] Create App struct in the music_core library for the frontend to call
- [x] Generic Subsonic response envelope + `call` helper
- [x] `getPlaylists`
- [x] `getArtists`
- [x] `getGenres`

### music_core: drill-down (getting to actual songs)
- [x] `Song` model (id, title, artist, album, duration, track, coverArt, ...)
- [x] `Album` model
- [x] `getArtist(id)` -> albums of an artist
- [x] `getAlbum(id)` -> songs on an album
- [x] `getPlaylist(id)` -> songs in a playlist
- [x] `getSongsByGenre(genre)` -> songs in a genre
- [x] `getAlbumList2` (recent / newest / random / frequent) for a home view

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
