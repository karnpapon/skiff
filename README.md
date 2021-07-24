# skiff

a static site generator, written purely in Rust.
inspired by [Oscean](https://github.com/XXIIVV/oscean) from [Hundred Rabbits](https://100r.co/index.html)
to inherite Longtermism philosophy, solutioning for technological resilience and sustainability.
by taking out JavaScript and only using html/css and plaintext with custom database called [indental](https://wiki.xxiivv.com/site/indental.html). for more details please watch [this inspiring talks](https://www.youtube.com/watch?v=BW32yUEymvU).

## usage

the ideas is super simple, the database tables are in the human-readable plaintext (see: `./rustlib/databases` ) The lexicon body uses a simple markup language.

## build

- go to folder by `cd rustlib`.
- build by `cargo run`.
- skiff will generate static HTML file (eg. `/site/dynamic-static-page.html`) depends on data in `/database`

## technical Notes
