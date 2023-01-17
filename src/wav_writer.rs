use std::f32::consts::PI;
use hound::WavWriter;
use std::io::{Write, Seek};

const SAMPLE_RATE: f32 = 44100.0;

// ノート番号と長さを表す構造体
#[derive(Debug)]
pub struct Note {
  pub no: i32,
  pub len: i32,
}

// Vec<Note>をWAVファイルへ書き出す関数
pub fn write(filename: &str, notes: Vec<Note>, bpm: f32) {
  // WAVファイルのフォーマットを指定
  let spec = hound::WavSpec {
    channels: 1,
    sample_rate: SAMPLE_RATE as u32,
    bits_per_sample: 16,
    sample_format: hound::SampleFormat::Int,
  };
  let mut fw = WavWriter::create(filename, spec).unwrap();
  // 繰り返しノートを書き込み
  for note in notes.into_iter() {
    // 音の長さを計算
    let len = (4.0 / note.len as f32 * (60.0 / bpm) * SAMPLE_RATE) as u32;
    // 周波数を計算
    let tone = if note.no >= 0 {
      440,0 * 2.0f32.powf((note.no -69) as f32 / 12.0)
    } else { 0.0 };
    write_tone(&mut fw, tone, len);
  }
}

// サイン波をファイルに書き込む

