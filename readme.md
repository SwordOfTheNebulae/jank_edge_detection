jank edge detection thing for the project for an art unit   
the edge detection itself isn't janky (apart from using an "outdated" algorithm for stylistic purposes)   
but everything else is   
it is very unpolished and kinda hacked together   
 
references-ish:   
https://www.sci.utah.edu/~cscheid/spr05/imageprocessing/project4/   
https://homepages.inf.ed.ac.uk/rbf/HIPR2/flatjavasrc/ZeroCrossing.java   

## How to use
have [cargo](https://www.rust-lang.org/tools/install) installed   
download/clone repo   
do `cargo build -r` in a terminal in this directory (the one with this file in it)   
`cd ./target/release/`   
### usage
`edge_detection <in_file> <out_file=out.png> <scale=8> <threshold=0> <sigma=2> <bg_red=0> <bg_green=0> <bg_blue=0>`   
or   
`edge_detection <in_file> <out_file=out.png> <scale=8> <threshold=0> <sigma=2> <use_random_background_colour=false>`   
if `in_file` is `camera` then it takes input from a webcam, ignores the out_file and opens a window that displays the result   
if using camera mode set scale to like 1, 2, or 3 otherwise its normally too small because it seems to always pick a low resolution for the camera feed   
   
in camera mode press `CTRL + s` (`CMD + s` i think on mac if you can even run this on mac idk) to save the current frame   

---
licence: GNU GPLv3
