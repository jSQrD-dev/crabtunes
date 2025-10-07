use std::path::{Path, PathBuf};

#[derive(Debug, Clone)]
pub struct Track {
    pub path: PathBuf,
    pub title: String,                   // Vorbis: TITLE, ID3: TIT2
    pub artist: String,                  // Vorbis: ARTIST, ID3: TPE1
    pub album: String,                   // Vorbis: ALBUM, ID3: TALB
    pub album_artist: Option<String>,    // Vorbis: ALBUMARTIST, ID3: TPE2
    pub genre: Option<String>,           // Vorbis: GENRE, ID3: TCON
    pub year: Option<u32>,               // Vorbis: DATE, ID3: TDRC (ID3v2.4) or TYER (ID3v2.3)
    pub comments: Vec<String>,           // Vorbis: DESCRIPTION/COMMENT, ID3: COMM
    pub composer: Option<String>,        // Vorbis: COMPOSER, ID3: TCOM
    pub duration: Option<u32>,           // Stored in audio properties, not in tags
    pub track_position: Option<u32>,     // Vorbis: TRACKNUMBER, ID3: TRCK (the 'N' in "N/T")
    pub track_total: Option<u32>,        // Vorbis: TRACKTOTAL, ID3: TRCK (the 'T' in "N/T")
    pub disc_position: Option<u32>,      // Vorbis: DISCNUMBER, ID3: TPOS (the 'N' in "N/T")
    pub disc_total: Option<u32>,         // Vorbis: DISCTOTAL, ID3: TPOS (the 'T' in "N/T")
    pub bpm: Option<u32>,                // Vorbis: BPM, ID3: TBPM
    pub lyrics: Option<String>,          // Vorbis: LYRICS, ID3: USLT (Unsynchronised lyrics/text)
    pub album_art_data: Option<Vec<u8>>, // Vorbis: METADATA_BLOCK_PICTURE, ID3: APIC (Attached picture)
    // Replay-Gain fields
    pub replaygain_track_gain: Option<f32>, // Vorbis: REPLAYGAIN_TRACK_GAIN, ID3: RVA2 (track)
    pub replaygain_track_peak: Option<f32>, // Vorbis: REPLAYGAIN_TRACK_PEAK, ID3: RVA2 (track)
    pub replaygain_album_gain: Option<f32>, // Vorbis: REPLAYGAIN_ALBUM_GAIN, ID3: RVA2 (album)
    pub replaygain_album_peak: Option<f32>, // Vorbis: REPLAYGAIN_ALBUM_PEAK, ID3: RVA2 (album)
    // Additional metadata fields
    pub musicbrainz_track_id: Option<String>, // Vorbis: MUSICBRAINZ_TRACKID, ID3: UFID/TXXX
    pub musicbrainz_artist_id: Option<String>, // Vorbis: MUSICBRAINZ_ARTISTID, ID3: TXXX
    pub musicbrainz_album_id: Option<String>, // Vorbis: MUSICBRAINZ_ALBUMID, ID3: TXXX
    pub musicbrainz_release_group_id: Option<String>, // Vorbis: MUSICBRAINZ_RELEASEGROUPID, ID3: TXXX
    pub discogs_release_id: Option<String>,           // Vorbis: DISCOGS_RELEASE_ID, ID3: TXXX
    pub discogs_master_release_id: Option<String>, // Vorbis: DISCOGS_MASTER_RELEASE_ID, ID3: TXXX
    pub discogs_artist_id: Option<String>,         // Vorbis: DISCOGS_ARTIST_ID, ID3: TXXX
    pub discogs_label: Option<String>,             // Vorbis: LABEL, ID3: TPUB
    pub catalog_number: Option<String>,            // Vorbis: CATALOGNUMBER, ID3: TXXX
    pub grouping: Option<String>,                  // Vorbis: GROUPING, ID3: TIT1 (Content group)
}

/// Helper to parse ReplayGain strings like "-7.23 dB" into a float.
fn parse_replaygain(value: &str) -> Option<f32> {
    value.trim_end_matches(" dB").parse::<f32>().ok()
}
