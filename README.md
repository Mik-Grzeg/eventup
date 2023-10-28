# Event UP

## Usage

As for the project command runner project [just](https://github.com/casey/just) is used. It has to be present on a host system to start the project.

docker compose environment uses profiles to enable additional functionalities.
* `log` profile - enables ClickHouse and RedPanda for Application activity loggin capabilities

### Instructions
In order to start transactional storage run from root location of the project:
```sh
just start
```

Further instructions can be found with `just --list`

### Dependencies
* docker compose >= 2.20.0
