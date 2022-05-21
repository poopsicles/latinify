# latinify

Just another super-typical Pig Latin translator...you already know what to expect 😂<br><br>

## Description

A pretty simple bare-bones [Pig Latin](https://en.wikipedia.org/wiki/Pig_Latin) translator...that's literally it.

Pig Latin is a fun little "language" of sorts, where the meaning of words are obsufcated through bleeding-edge encryption techniques, rendering the sentence intelligible to the untrained eye.

But you wouldn't dare do that by hand, would you?
<br><br>
## Getting Started

### Dependencies

* Install the [Rustup](https://rustup.rs/) toolchain for your system.

### Installing

* Add Cargo's binary directory to your path environment variables.<br>The rustup installer should do this for you, but if not, it should be located at `~/.cargo/bin` (Linux) or `%USERPROFILE\.cargo\bin` (Windows).
* Simply run `cargo install latinify` at a terminal and it'll be installed from crates.io.

### Executing program

* Type `latinify` at a prompt

```
$ latinify
Welcome to latinify! Type a sentence and press Return/Enter to get started...
> _
```
* You can then simply type the sentence you want to be translated, and hit the Return/Enter key on your keyboard to translate it.
```
> Hello, World! It's a beautiful day today...
Ellohay, Orldway! Ithay'say ahay beautifulay dayay todayay...

> _
```
<br>Typing `:exit` will quit the interpreter.<br><br>
## Building from source

* Clone this repository
```
$ git clone https://github.com/poopsicles/latinify
```

* Switch to the newly created directory 
```
$ cd ./latinify
```

* Compile using cargo
```
$ cargo build
```

* Cargo will grab the required dependencies and create the binary at `./target/debug/latinify`<br><br>

## Version History

* 1.0
    * Initial release<br><br>

## License

This project is licensed under the MIT License, more details [here](LICENSE).