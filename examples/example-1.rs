use braille::*;
fn main() {

    println!("{}", BRAILLE[1][0]
                          [0][1]
                          [1][0]
                          [1][1]);


    println!("{}", BOX[1][1]
                      [1][0]);

    println!("{}", BRAILLE_SINGLE[2][1]);

    println!("{}", BOX_SINGLE[1][1]);

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
}
