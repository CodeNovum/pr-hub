# Contributing

## Local development

### Dev machine setup

To start the app locally in dev mode, you need the following tooling installed:

- [rust and cargo](https://www.rust-lang.org/tools/install)
- [node](https://nodejs.org/en/download)
- [pnpm](https://pnpm.io/installation)

A more in depth guide on how to setup all required tooling to develop on a tauri
application can be found [here](https://v2.tauri.app/start/prerequisites/)

### Starting the app in dev mode

First ensure that the NPM dependencies are installed on your system:

```shell
pnpm i
```

Then you can start the app in development mode:

```shell
cargo tauri dev
```

If you want to start the app using pnpm, or have other issues, refer to the official
[tauri-docs](https://v2.tauri.app/start/create-project/#start-the-development-server).

## Pull Requests

All changes should be merged to the default branch only via pull requests.

The following should be enforced:

- the title is used as squash commit message
- the title should be a
  [conventional commit message](https://www.conventionalcommits.org/en/v1.0.0/)
- a review should be done by another developer
- the most recent push needs an approval
- an issue should be linked
