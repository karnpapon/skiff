![image 1](/media/images/anu/thumb-000.png)

as the name suggests "anu" (or "อนุ"), which in Thai grammar denotes a prefix meaning "small", "sub" or "minor". It can be prefixed(compatible) with any others software/hardware that support [ OSC ](https://en.wikipedia.org/wiki/Open_Sound_Control) or [MIDI](https://en.wikipedia.org/wiki/MIDI) protocol (more to be implemented).
 
unlike others conventional step-sequencers, "anu" explore a new musical expressions and territories, while still balancing deterministic and stochastic processes (previously, this project have been developed under the name "seeq").

written in plain JavaScript with dependencies as less as possible in minds. powered by [Tauri](https://tauri.app/), a framework for building tiny, blazing fast binaries for all major desktop platforms.

<div style="width: 100%; height: 0; padding-top: 56.25%; position: relative;">
	<iframe width="100%" height="100%" style="border:none;overflow:hidden;position:absolute;top:0;" src="https://www.youtube.com/embed/kXfi4FhzCi8" title="YouTube video player" frameborder="0" allow="accelerometer; autoplay; clipboard-write; encrypted-media; gyroscope; picture-in-picture" allowfullscreen></iframe>
</div>

*Talk: "Regular Expression as an Irregular Musical Expression" @ CreativeCoding meetup 2023, Thailand (organized by [Creatorsgarten](https://creatorsgarten.org/))*

&nbsp;
&nbsp;

## screenshots
![image 1](/media/images/anu/anu-ss.png)
![image 1](/media/images/anu/anu-console.gif)
![image 1](/media/images/anu/anu-ss-1.gif)
![image 1](/media/images/anu/anu-ss-2.gif)
![image 1](/media/images/anu/anu-ss-3.gif)

## usages
- [OSC]: sending OSC message (based-on [`oscd`](https://github.com/karnpapon/oscd))
  - in case of sending sequencial messages (like Arpeggiator), use `|` as a delimiter, eg. `"msg1" 440.0 | "msg2" 450.0` will send `"msg1" 440.0` and `"msg2" 450.0`, respectively when triggering. 
  - within sequencial messages, you can have any type that OSC is supported including an Array ([see complete support list here](https://github.com/karnpapon/oscd#usage)), which means you can send something like `"msg1" 440.0 [12,44,true] | "msg2" 450.0 [30,20.1,"msg inside an array"]`

## features
- lightweight and cross-platform (application size only ~9mb)
- support sending OSC
- support sending MIDI
- precise clock scheduling
- mutable marker
- reversable marker
- adjustable BPM (without jittery)
- fault-tolerance regex
- live-performance oriented
- adjustable note-ratio per marker
- mono step (when finish running current marker it'll automatically run the next marker, and so on, basically the marker will run one-by-one)
- [!experimental] [ratcheting](https://learningmodular.com/glossary/ratcheting/)

# download
[ download ](https://github.com/karnpapon/anu/releases) latest installer at release page, support major platforms(Win/OSX/Linux)

## inspirations
draw an inspirations from Xenakis's work [Achorripsis](https://muse.jhu.edu/article/7871/summary)(1956) and Esoteric Environment like [Orca](https://hundredrabbits.itch.io/orca) also others obsoleted music software.
