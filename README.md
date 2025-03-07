# cd-allow

## Table of Contents

* [Motivation](#motivation)
* [Requirements](#requirements)
* [Installation](#Installation)
* [Usage](#Usage)
* [Contributing](#Contributing)
* [License](#License)


## Motivation

In the cargo deny [0.16.0](https://github.com/EmbarkStudios/cargo-deny/blob/main/CHANGELOG.md#0160---2024-08-02) the `allow-osi-fsf-free` attribute was removed from the `[licenses]` section, making mandatory to list all the allowed licenses in the `deny.toml` file. This project just make that listing in the toml part for you.

## Requirements

It's not mandatory, but this CLI is created to help with the `deny.toml` required by [cargo-deny](https://github.com/EmbarkStudios/cargo-deny)

## Installation

To install you just need to execute

```
cargo install --locked cd-allow
```

## Usage

After you've installed the cd-allow, you just need to execute the `cd-allow` with the desired flags `use the --help` to more information about it

### Adding all OSI-approved licenses to `deny.toml`
``` 
> cd my-project/
> cd-allow --file deny.toml --osi
```

## Contributing

If you feel like something is missing/broken, feel free to create an issue or submit a PR.

## License

This project is under the `MIT License`. See the [LICENSE](LICENSE.md) file for more details.
