This crate is dedicated to handling the visual aspects of the RS bot. That means taking screenshots and finding objects of interest.

# Screenshot

The basic input to the bot is a screenshot of OSRS. This is done using the scrap library, which gives us a screenshot as a GBRA rowwise vector. Example usage in the screenshot binary.

```
$ cargo run -p screen --bin screenshot -- --out-dir /path/to/dir/
```

# Get pixel value

Now that we can capture screenshots, we need to decide how to find our target within it. The search is done by finding relevant pixels (as opposed to full object detection). This is based on an assumption that OSRS graphics are simple enough for this to work. For this we have 2 options.

Move the mouse to an area of interest and get it's location and the pixel it is covering.
```
$ cargo run -p screen --bin print_mouse_pixel
```

Or you can take a saved image and use https://yangcha.github.io/iview/iview.html.

If you want to translate from pixel values to a visual - https://www.rapidtables.com/web/color/RGB_Color.html

# Find an object

Now that we have decided what to look for, we need to find it. We search for relevant pixels by giving a range of values that each channel can fall within (0-255 for each of blue, green, and red). We also give the region to look in.

```
$ cargo run -p screen --bin find_pixel_fuzzy -- --top-left 960,40 --past-bottom-right 1920,1040
```

# Libraries with dependencies
scrap

I am dependent on runelight popping up icons such as shrimp on fishing spots. TBH if I was a regular player I'd really want this since seeing certain things is quite difficult.

# Explain how to extend
- Explain adding new letters/words with bin/action_words.rs
- Explain adding new inventory pixels with bin/inventory.rs & uncommenting in frame.rs
- What the player looks like can cause an issue since those are also just pixels on the screen.