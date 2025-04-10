use crate::util::song;

use super::song::Song;
use super::basetype::Timebase;
extern crate wasm_bindgen;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct songWrapper {
    song: Song,
}

#[wasm_bindgen]
impl songWrapper {
    pub fn new(name: &str) -> Self {
        songWrapper {
            song: Song::new(name)
        }
    } // fn new

    pub fn new_channel(
        &mut self,
        name: &str,
        preset: &str,
        volume: f32,
        n_poly: usize,
        pan: i8,
        be_modulated: bool,
    ) {
        self.song.new_channel(name, preset, volume, n_poly, pan, be_modulated);
    } // fn new_channel

    pub fn clear(&mut self) {
        self.song.clear();
    }

    pub fn new_pattern(
        &mut self,
        channel_id: usize,
        start_time: Timebase,
        pattern_name: &str,
    ) {
        self.song.new_pattern(channel_id, start_time, pattern_name).unwrap();
    }

    pub fn move_pattern_time(
        &mut self,
        channel_id: usize,
        pattern_id: usize,
        new_start_time: Timebase,
    ) {
        self.song.move_pattern_time(channel_id, pattern_id, new_start_time).unwrap();
    }

    pub fn move_pattern_channel(
        &mut self,
        channel_id: usize,
        pattern_id: usize,
        new_channel_id: usize,
    ) {
        self.song.move_pattern_channel(channel_id, pattern_id, new_channel_id).unwrap();
    }

    pub fn edit_pattern(
        &mut self,
        channel_id: usize,
        pattern_id: usize,
        mode: &str,
        note_idx: u8,
        start_time: Timebase,
        end_time: Timebase,
    ) {
        self.song.edit_pattern(channel_id, pattern_id, mode, note_idx, start_time, end_time).unwrap();
    }

    pub fn save_to_file(&self, file_path: &str) {
        self.song.save_to_file(file_path);
    }

    pub fn read_from_file(&mut self, file_path: &str) {
        self.song.read_from_file(file_path).unwrap();
    }
} // impl songWrapper