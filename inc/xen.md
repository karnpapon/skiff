an implementation on [ De Casteljau's ](https://en.wikipedia.org/wiki/De_Casteljau%27s_algorithm) algorithm, capable of sending MIDI (trigger), [OSC](https://en.wikipedia.org/wiki/Open_Sound_Control) to target client/server  eg.[SuperCollider](https://supercollider.github.io/) (OSC only works in repl mode), NOTE: xen is backend-agnostic, Intentionally used for live performance. thus, no built-in audio engine is implemented.

![image 1](/media/images/xen/ss3.gif)

<div style="width: 100%; height: 0; padding-top: 56.25%; position: relative;">
	<iframe width="100%" height="100%" style="border:none;overflow:hidden;position:absolute;top:0;" src="/media/images/xen/xen-live.mp4" title="xenlive" frameborder="0" allow="accelerometer; clipboard-write; autoplay=0; encrypted-media; gyroscope; picture-in-picture"  allowfullscreen sandbox></iframe>
</div>

video taken by [Tentacle Workshop Bkk](https://www.facebook.com/TentaclesN22/)

## Run

```
git clone https://github.com/karnpapon/xen.git
cd xen
npm install
npm run start

# in order to send OSC
# open new terminal tab.
cd bridge  
npm install
node index.js

```

## Keybinding
- **play/pause**: `spacebar`
- **add new point**: `cmd` + `left click`
- **add point group**: `Shift` + `n` = spawn new point group.
- **remove point**: `right click` at target point (make sure group is selected).
- **trigger first/last line** `t`, since cubic bezier calculated by four points. by nature the distance between point is basically a straight line. thus moving point cannot avoid colliding with first line(start) and last line(end). use this option to avoid trigger first/last line.
- **switch between point group**: `tab`, selected group will be highlighted in `BLUE` color `rgb(0,0,255)`
- **toggle control-line**: `c` = only current selected group, `Shift` + `C` = toggle all group.  
- **toggle L-Point**: `l` = toggle even recursived points (collision in `RED`), `Shift` + `L` = toggle all points.
- **toggle R-point**: `r` = toggle odd recursive points (collision in `BLUE`),  `Shift` + `R` = toggle all points.

## File/Folder

- **/src**: main xen's sourcecode.
- **/tool**: for test sending msg(osc).
- **/bridge**: for receiving msg(osc) from browser and forward to target server.


### IO

- **midi**.
- **osc**: Sends OSC message, **NOTE** run `node bridge/index.js` first. in order to send OSC out to host.

# Resources
- https://pomax.github.io/bezierinfo/#decasteljau
- https://www.khanacademy.org/computing/pixar/animate/parametric-curves/a/equations-from-de-casteljaus-algorithm
- https://www.youtube.com/watch?v=aVwxzDHniEw
- https://webmidijs.org/docs/getting-started/basics/