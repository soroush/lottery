# 🎰 True Entropy Lottery

TL;DR **This isn't a serious project. If you mean business, look somewhere else.**

This is a toy project, implementing a random number generator that provides cryptographically secure, bias-free random numbers for lottery games. Not useful for anything really.

## FAQ

**Q: Why use this lottery random number generator?**<br>
A: You shouldn't. It's just a fun project to test Rust and cryptographic randomness.

**Q: Why did you make this?**<br>
A: I wanted to see what's the fuss about Rust and try out some cryptographic randomness libraries.

**Q: Is this suitable for production use?**<br>
A: What do you think? Use at your own risk.

**Q: Will this increase my chances of winning the lottery?**<br>
A: Don't be stupid. No, it won't. Lottery games are designed to be random, and this tool simply provides a way to generate random numbers.

## Features

- 🎲 **Cryptographically Entropic Randomness** - Use PRNGs with high levels of entropy
- 🔒 **Bias-Free Generation** - Eliminates statistical bias in number selection (Remove [Modulo Bias](https://en.wikipedia.org/wiki/Random_number_generation), [Representativeness Heuristic Fallacy](https://en.wikipedia.org/wiki/Representativeness_heuristic), [Gambler's Fallacy](https://en.wikipedia.org/wiki/Gambler%27s_fallacy) and such)
- ⚡ **High Performance** - Built with Rust for maximum speed. Not that it really matters though
- 🎯 **Multiple Lottery Formats** - Supports various lottery game configurations
- 📊 **Configurable Parameters** - Customize number ranges and draw sizes

## Quick Start

```bash
# Clone the repository
git clone https://github.com/soroush/lottery.git

# Navigate to the project directory
cd lottery

# Build the project
cargo build --release

# Run with default settings
cargo run -- --game euromillions --tickets 5
```

## Installation

### Prerequisites

- [Rust](https://www.rust-lang.org/tools/install)
- [Cargo](https://doc.rust-lang.org/cargo/getting-started/installation.html)

### Setup

```bash
# Build the project
cargo build --release
# The binary will be available at target/release/lottery
```

## Command Line Usage

```bash
$ lottery --game euromillions --tickets 2
1: [20 26 31 41 47] [02 08]
2: [01 22 26 37 40] [05 12]
```

## Configuration

You can create a `games.json` configuration file to define custom lottery games. There is a default `games.json` file included in the repository. It conitains definitions for several popular lottery games and is shipped with the binary. You can change it or add your own games. The application will look for this file in the following locations (in order):

1. `./games.json`
2. `$XDG_CONFIG_HOME/lottery/games.json` (usually `~/.config/lottery/games.json`)
3. `/etc/lottery/games.json`

In Windows systems:

1. `.\games.json`
2. `{FOLDERID_RoamingAppData}\lottery\games.json` (usually `C:\Users\<User>\AppData\Roaming\lottery\games.json`)
3. `C:\ProgramData\lottery\games.json`

The structure of the `games.json` file is as follows:

```json
{
    {
      "name": "EuroMillions",
      "id": "euromillions",
      "description": "EuroMillions is a transnational lottery ...",
      "country": "Multiple European countries",
      "draw_days": ["Tuesday", "Friday"],
      "pools": [
        {
          "count": 5,
          "range": [1, 50]
        },
        {
          "count": 2,
          "range": [1, 12]
        }
      ],
      "format": "[ {0:02} {1:02} {2:02} {3:02} {4:02} ]  [ {5:02} {6:02} ]"
    }
}
```

The important fields are:

- `name`: The name of the lottery game. This is just for display purposes.
- `id`: A unique identifier for the lottery game. This is used to specify the game when running the application in the command line.
- `pools`: An array of number pools. Each pool has a `count` (how many numbers to draw) and a `range` (the inclusive range of numbers to choose from).
- `format`: A format string to display the drawn numbers. The numbers are indexed starting from 0.

## Future Plans

So many useless features to add to this useless toy. Here are some:

- [ ] **Integrate Quantum Entryropy APIs** - Use quantum entropy sources for randomness to remove all possible bias in electronics. Candidates are [ANU Quantum Random Numbers Server](https://qrng.anu.edu.au/) and [Quantum Random Bit Generator Service](https://www.idquantique.com).
- [ ] **Concurrency** - Because why not
- [ ] **More Lottery Games** - Add more predefined lottery games
- [ ] **Better Output Formatting** - More options for formatting the output
- [ ] **Batch Generate** - Generate multiple tickets of multiple games at one go
- [ ] **File Output** - Save generated tickets to a file (CSV, JSON, etc.
- [ ] **API Service** - Expose functionality via a RESTful API
- [ ] **API Integration** - Fetch latest lottery results from online APIs)

## 🤝 Contributing

We love contributions! Here's how you can help:

1. 🍴 Fork the repository
2. 🌱 Create a feature branch
3. 💻 Make your changes
4. ✅ Add tests
5. 🚀 Submit a pull request

## 📜 License

This project is licensed under the WTFPL license. see the [LICENSE](./LICENSE) file for details.

<a href="http://www.wtfpl.net/">
  <img src="https://www.wtfpl.net/wp-content/uploads/2012/12/wtfpl.svg" 
       width="125"  alt="WTFPL" />
</a>
