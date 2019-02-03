webmask is identified by 4 bytes = `MASK`.

```
MASK        <> VERSION     <> kI <> vU          <> Ly
4D 41 53 4B <> 00 00 00 01 <> 02 <> 02 00 00 00 <> 00 00 00 0B
```

MASK must equal "MASK".
kI must equal 2.

```
version = 1
kI = 2
vU = 33554432
Ly = 11 (I think these may be timing segments? perhaps, keyframes?)
```

version = int32 at offset 4
kI = int8 at offset 8
vU = int32 at offset 8
Ly = int32 at offset 12

c == "header length"
c = 16

b == "timing info length?"
b = 16 * Ly (11) = 176

> Looks like the file has 3 parts.
> 1: Header
> 2: Timing Data
> 3: SVG Data

---

MK function

frames = []
l = 0
loop through 0..`Ly`

if 0 == int32(timing_data[l]) && 0 == int32(timing_data[l + 8]) {
  time = int32(l + 4)
  offset = int32(l + 12)
  l = l + 16

  frames.push({
    time: time,
    offset: offset
  })
}

number_of_frames = frames.length
l = 1000 * duration # not sure where duration comes from.
l > frames[number_of_frames - 1].time && frames.push({
  time: l,
  offset: -1
})

if (last_frame.time < total_length) {
  frames.push({
    time: l,
    offset: -1
  })
}

# This is probably a data saving method, if last frame specified is not last frame in the video: use first frame as last frame?

---

Frame Data

frame:
data = [];
data_len = int32
time (g) = int32 at offset 8

frame_length = 12 + data_len

frameData.push({
  start: b,
  end: c,
  data: data
})


