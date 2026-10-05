# Audio mix recipes

## Goals

- Integrated loudness around **-16 LUFS** (YouTube/Spotitube target)
- True peak ≤ **-1 dBTP** (room for transcoder overshoot)
- Zero clipped samples
- No scene with peak above -10 dBFS (sounds compressed if some scene dominates)

## Quick start (BGM + 2-3 sfx tracks)

```rust
fn audio(&self) -> AudioMap<'_> {
    AudioMap::from([
        AudioTrack::new("bgm60.wav", Second(0.)..Eof).gain_db(-5.0).fade_in(1.5).fade_out(2.5),
        AudioTrack::new("whoosh-sfx.wav", Second(6.85)..Eof).volume(1.4),   // big transition
        AudioTrack::new("whoosh-sfx.wav", Second(12.85)..Eof).volume(1.4),  // another big transition
        AudioTrack::new("pop-sfx.wav",  Second(9.3)..Eof).volume(1.4),     // plaza click
    ])
}
```

The BGM gain is in dB and stacks with the file's natural loudness. Start with `gain_db(-5.0)`, run `cargo run --release -- audio analyze`, and adjust up/down in 2-dB steps.

## Synthesizing sfx (ffmpeg one-liners)

These live in `promo/media/` flat — no subdirectories. `include_media_dir!` doesn't recurse.

```sh
cd promo/media

# whoosh — brown noise with lowpass sweep + envelope (~0.7s)
ffmpeg -y -f lavfi -i "anoisesrc=colour=brown:d=0.7:amplitude=0.8" \
  -af "lowpass=f=900:poles=2,highpass=f=120,afade=t=in:d=0.18,afade=t=out:st=0.3:d=0.4,volume=1.4" \
  -ar 44100 -ac 2 whoosh-sfx.wav

# pop — short sine with echo decay (~0.2s)
ffmpeg -y -f lavfi -i "sine=frequency=620:d=0.14" \
  -af "volume=0.55,afade=t=out:st=0.02:d=0.12,aecho=0.6:0.3:60:0.25" \
  -ar 44100 -ac 2 pop-sfx.wav
```

Adjust `frequency=620` upward (900-1200) for "tap" feel, downward (300-500) for "thud". Adjust `d=0.14` for length.

## Placement rules

- Place whooshes 0.05-0.1s **before** the visual transition so the eye and ear sync.
- Place pops 0.0-0.05s **after** the visual beat (e.g. 50ms after the click ripple starts expanding).
- Never more than 6 sfx tracks total — they overlap and muddy the mix.
- Sfx volumes: whoosh 1.0-1.4, pop 1.0-1.4. Never above BGM by more than ~3 dB.

## Debugging with `audio analyze`

```sh
cargo run --release -- audio analyze            # summary
cargo run --release -- audio analyze --json     # machine-readable
cargo run --release -- audio analyze --waveform frames/waveform.png
```

Per-scene report tells you where the mix is uneven:

```
IntroScene                         0.00s..3.50s  -30.7 LUFS / peak -19.6 dBTP
PlazaScene                        7.00s..13.00s -30.0 LUFS / peak -10.4 dBTP
ScenarioGridScene                13.00s..17.50s -26.4 LUFS / peak -10.8 dBTP
```

If `PlazaScene` jumps 4 dB above neighbors, the click pop is too loud or lands wrong. If `IntroScene` is much quieter than the rest, the fade-in is too aggressive or starts from zero instead of partial level.

## Loudness cheat sheet

| Integrated LUFS | Use for |
|---|---|
| -14 to -16 | YouTube/social videos (most common target) |
| -18 to -20 | Apple Music/Spotify streaming |
| -23 | EBU broadcast (rare for promos) |

Adjust `gain_db` on the BGM track to reach target. The clip rule: integrated LUFS increases by ~1 for every 1 dB of gain boost, peaks rise faster than that.

## AudioMap API quick reference

```rust
AudioTrack::new(file: &str, range: AudioDuration).gain_db(d).fade_in(s).fade_out(s)
                                         .volume(v).pan(p).offset(s).voice()
                                         .duck_under_voice()
```

- `volume(N)` is linear (1.0 = unity, 0.5 ≈ -6 dB). Use for sfx.
- `gain_db(N)` is dB. Use for BGM that needs precise loudness.
- `Second(n)..Eof` means "play from second n to end of file".
- `Second(n)..Second(m)` for clipped ranges.
- `range: Second(0.)..Eof` plays the entire file.
