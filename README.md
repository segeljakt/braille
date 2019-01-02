# braille

A library for generating braille and box-drawing characters.

![Img1](https://cdn.discordapp.com/attachments/423260820760428564/450404172626657283/expand.gif)

![Img2](https://media.giphy.com/media/2xPYeK6vTvxO9al6WT/giphy.gif)

# API

This library exposes four arrays:

* `BRAILLE` - Print a braille character.
* `BRAILLE_SINGLE` - Print a single-dot braille character.
* `BOX` - Print a box-drawing character.
* `BOX_SINGLE` - Print a single-dot box-drawing character.

How these work is best explained with examples.

# Examples (Braille)

```rust
println!("{}", BRAILLE[1][0]
                      [0][1]
                      [1][0]
                      [1][1]);
```
Output: `⣕`

```rust
println!("{}", BRAILLE_SINGLE[2][1]);
```
Output: `⠠`

```rust
for a in 0..2 {
    for b in 0..2 {
        for c in 0..2 {
            for d in 0..2 {
                for e in 0..2 {
                    for f in 0..2 {
                        for g in 0..2 {
                            for h in 0..2 {
                                print!("{}", BRAILLE[a][b]
                                                    [c][d]
                                                    [e][f]
                                                    [g][h]);
                            }
                        }
                    }
                }
            }
        }
    }
}
println!("");
```
Output: ` ⢀⡀⣀⠠⢠⡠⣠⠄⢄⡄⣄⠤⢤⡤⣤⠐⢐⡐⣐⠰⢰⡰⣰⠔⢔⡔⣔⠴⢴⡴⣴⠂⢂⡂⣂⠢⢢⡢⣢⠆⢆⡆⣆⠦⢦⡦⣦⠒⢒⡒⣒⠲⢲⡲⣲⠖⢖⡖⣖⠶⢶⡶⣶⠈⢈⡈⣈⠨⢨⡨⣨⠌⢌⡌⣌⠬⢬⡬⣬⠘⢘⡘⣘⠸⢸⡸⣸⠜⢜⡜⣜⠼⢼⡼⣼⠊⢊⡊⣊⠪⢪⡪⣪⠎⢎⡎⣎⠮⢮⡮⣮⠚⢚⡚⣚⠺⢺⡺⣺⠞⢞⡞⣞⠾⢾⡾⣾⠁⢁⡁⣁⠡⢡⡡⣡⠅⢅⡅⣅⠥⢥⡥⣥⠑⢑⡑⣑⠱⢱⡱⣱⠕⢕⡕⣕⠵⢵⡵⣵⠃⢃⡃⣃⠣⢣⡣⣣⠇⢇⡇⣇⠧⢧⡧⣧⠓⢓⡓⣓⠳⢳⡳⣳⠗⢗⡗⣗⠷⢷⡷⣷⠉⢉⡉⣉⠩⢩⡩⣩⠍⢍⡍⣍⠭⢭⡭⣭⠙⢙⡙⣙⠹⢹⡹⣹⠝⢝⡝⣝⠽⢽⡽⣽⠋⢋⡋⣋⠫⢫⡫⣫⠏⢏⡏⣏⠯⢯⡯⣯⠛⢛⡛⣛⠻⢻⡻⣻⠟⢟⡟⣟⠿⢿⡿⣿`

# Examples - (Box-drawing characters)

```rust
println!("{}", BOX[1][1]
                  [1][0]);
```
Output: `▛`

```rust
println!("{}", BOX_SINGLE[1][1]);
```
Output: `▗`

```rust
for a in 0..2 {
    for b in 0..2 {
        for c in 0..2 {
            for d in 0..2 {
                print!("{}", BOX[a][b]
                                [c][d]);
            }
        }
    }
}
println!("");
```
Output: ` ▗▖▄▝▐▞▟▝▚▌▙▀▜▛▋`

