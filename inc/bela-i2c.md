for safety parsing [OSC](http://opensoundcontrol.org/) through i2c a [logic level converter](https://www.sparkfun.com/products/12009) to use with Eurorack level (5V) is needed.

![img-1](/media/images/bela-i2c/04.jpg)

## Quick Start
make sure you're already on Bela board (root@bela). and Bela's WIFI is setup, details [here](https://learn.bela.io/using-bela/bela-techniques/connecting-to-wifi/).

```
# download this repo to Bela.
git clone https://github.com/karnpapon/bela-i2c && cd bela-i2c

# run bela-osc
./build.sh

# build c++ file on Bela (more handy, if you don't want to open Bela user-interface and click build button.)
make -C ~/Bela PROJECT=render.cpp run CL="-p16"

# open another window
cargo run
```

## Usage
osc pattern
`module/module_number/command/output_port value`

| msg           | description                                                                                | type   | example               |
|---------------|--------------------------------------------------------------------------------------------|--------|-----------------------|
| module        | name of connecting module, currently support only   ER301                                  | String | er301                 |
| module_number | the order number of connecting module                                                      | Int    | 1                     |
| command       | command to send to module available command for ER301 documentation please refer to   this | String | cv_slew, tr_pulse, tr |
| output_port   | output port number                                                                         | Int    | 1                     |
| value         | value to send to                                                                           | Float  | 1000                  |

please refer to module documentation
use same configuration as [Teletype](https://monome.org/docs/teletype/) (value = unsigned 14bits integer (16,384 = 10v))

## [Example] sending data via WIFI (eg. use TouchOSC (Mobile) -&gt; Bela)

<div style="width: 100%; height: 0; padding-top: 56.25%; position: relative;">
	<iframe width="100%" height="100%" style="border:none;overflow:hidden;position:absolute;top:0;" src="https://www.youtube.com/embed/BFHCymXCxAw" title="YouTube video player" frameborder="0" allow="accelerometer; autoplay; clipboard-write; encrypted-media; gyroscope; picture-in-picture" allowfullscreen></iframe>
</div>

![img-2](/media/images/bela-i2c/02.png)

- setup WIFI on Bela first, details [here](https://learn.bela.io/using-bela/bela-techniques/connecting-to-wifi/).
- make sure you&#39;re in same WIFI as Bela.
- on Bela
- get wlan IP by `ip a` on Bela board.
- on TouchOSC:
  -  set HOST to IP obtained by previous command (eg. `192.168.1.115` as shown above).<
  - set Outgoing to `7562` (default Bela&#39;s listening port).

## Disclaimer
this have been successfully tested on
Bela image: v0.3.2, (released 13 March 2018).
OSX: 10.13.6 (High Sierra)
MacBook Pro (Retina, 13-inch, Early 2013)

inspired by [hans](https://llllllll.co/t/hans/36455/14)