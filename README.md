### webmask-renderer

Rust project for rendering `.webmask` files.

Right now, it outputs a 1280x720 transparent png sequence into an `out/` directory.

A few more ffmpeg steps are necessary to construct a playable movie:

```
ffmpeg -framerate 24 -i out/%d.png -filter_complex "color=size=1280x720:color=white [white]; [white][0:v] overlay=shortest=1 [out]" -map "[out]" movie.mp4
```

Some more improvements to be made:

- Outputting animated format.
- Output slices of animation.
- Customize svg rendering (blur, color, etc.)
- Ability to mux audio data.
