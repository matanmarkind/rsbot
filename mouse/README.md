This crate is dedicated to mouse movements for the RS bot. In it there are
2 portions: prep work needed to "seed" the mouse controller, the controller
used during play.

The design of the mouse is based around trying to look human. This means
avoiding issues like moving in a perfectly straight line, teleporting directly
to the target, always clicking on the exact same pixel. The strategy to avoid
these pitfalls is to have the mouse follow actual recorded movements and then
play them back.

# Recording

Step one is to record mouse movements. This is done by running the record
binary. Every ~10ms, it will record the mouse's location and store this in
a CSV. 'device_query' was the library needed to get the mouse's location on
linux, check it on crates for install info.

```
rsbot$ cargo run -p mouse --bin record -- --out-fpath /path/to/mouse_raw.csv --batch-period-s 60 --active-time-s 1200
```

# Parsing

Once mouse positions have been recorded, we need to convert this into paths
that can be replayed. The goal is to create a data structure that maps from
{path_distance : path}. 'path_distance' is the net distance from the mouse's
starting location to the finishing location, it is not the total distance
covered by following the path. This is is needed to find the relevant path on
replay. We also calculate the angle the path follows (again net from initial to
final position), which will be used on replay.

We serialize the output in bincode format.

```
rsbot$ cargo run -p mouse --bin parse -- --in-fpath /path/to/mouse_raw.csv --out-fpath /path/to/mouse_paths.bincode
```

# Replay

This binary allows for user testing to see how the mouse actually behaves. It
makes use of the exported controller library. The way this works is:
- A destination, 'dst', is given (x, y)
- The controller calculates the distance from the current location, 'cur_loc', to
  'dst'.
- The database of recorded paths is searched for a path of this length.
- The path is rotated so that the mouse will move towards 'dst'.

Move movement uses the inputbot crate, which require sudo to move the mouse.

```
rsbot$ cargo build -p mouse --bin replay && sudo ./target/debug/replay --in-fpath /path/to/mouse_paths.bincode
```
