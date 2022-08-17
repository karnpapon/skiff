> String is  traditionally a sequence of letters.

Seeq is yet-another-live-coding-environment-alike sequencer. It's heavily inspired by [Orca](https://hundredrabbits.itch.io/orca).
Initially, the idea coming from being tired of keep remembering new available functions from [TidalCycles](https://tidalcycles.org/), a language for creating complex pattern purposely for <a target="_blank" href="https://toplap.org/">live-coding</a> performance.  

Don't get me wrong. TidalCycles is hands-down obviously a great language out there. 
But for non-programming person like me. I cannot utilize the true power of TidalCycles without knowing [Haskell](https://www.haskell.org/) language.

Thus, for the sake of understanding programming in general. I decided to roll my own.
JavaScript is a language of choice since it's personally more friendly for newbie. 
Creating programming language is considered advanced topic. 
So, I decided to looking for practical way like sequencer instead. and utilize from existing "language" like [RegEx](https://en.wikipedia.org/wiki/Regular_expression) to create a pattern (I know, I know you may immediately argue that "but RegEx is not even considered a programming language!". well, you're right but lemme call it language in this context anyway).

![img1](/media/images/seeq/05.gif)

<div style="width: 100%; height: 0; padding-top: 56.25%; position: relative;">
	<iframe width="100%" height="100%" style="border:none;overflow:hidden;position:absolute;top:0;" src="https://www.youtube.com/embed/DGaakhSvYOg" title="YouTube video player" frameborder="0" allow="accelerometer; autoplay; clipboard-write; encrypted-media; gyroscope; picture-in-picture" allowfullscreen></iframe>
</div>

![img-2](/media/images/seeq/diagram.svg)
![img-3](/media/images/seeq/04.gif)

## Usage
to play with it, just put any words, sentences, just like searching from Google or any search engine. Seeq will manage to find the descriptions for that keywords and use it as a step sequencer (using API from WikiPedia).
a “trigger” is simply assigned by typing any words/letters in provided  `find` input,  **Regex is also supported.** in `regex` input.
in order to move selection, the console has to be toggled off ( `cmd` + `i`).

### movement
| Operation    | keybinding      |
|--------------|-----------------|
| start / stop | spacebar        |
| move         | arrow           |
| leap         | `cmd` + `arrow` |

### selection ( cursor )
| Operation         | keybinding                  |
|-------------------|-----------------------------|
| range             | shift   +   arrow           |
| large range       | shift   +   arrow   +   cmd |
| add               | cmd   +   n                 |
| delete            | cmd   +   backspace         |
| rename            | cmd   +   e                 |
| show current name | option   +   e              |
| switch between    | option   +   tab            |
| get step within   | cmd   +   return (enter)    |
| focused           | cmd   +   f                 |

### step ( within selection )
| Operation | keybinding    |
|-----------|---------------|
| add step  | shift   +   + |

### console -> input
| Operation     | keybinding  |
|---------------|-------------|
| toggle insert | cmd   +   i |
| eval input    | enter       |

### console -> status
| Operation | keybinding  |
|-----------|-------------|
| BPM up    | cmd   +   > |
| BPM down  | cmd   +   < |

### config
| Operation | keybinding  |
|-----------|-------------|
| set MIDI  | cmd   +   m |