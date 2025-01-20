# pr-hub

## Summary

A central entry point to view pull requests across repositories.
Currently this targets only the platform [Azure DevOps](https://dev.azure.com/)
and enables viewing pull request related data across multiple tenants,
projects and repositories.

## Installation

The prebuilt binaries can be downloaded using GitHub
[releases](https://github.com/CodeNovum/pr-hub/releases).

### macOS

Currently the applications are not signed. Therefore you may need to
do the following after the download, before you can run the app/the installer:

```shell
xattr -d com.apple.quarantine ./PATH_TO_DOWNLOADED_BIN
```

## Contributing

You can find guidelines on how to contribute to this repository
[here](https://github.com/CodeNovum/pr-hub/blob/main/CONTRIBUTING.md).

## License

This project is licensed under the
[MIT license](https://github.com/CodeNovum/pr-hub/blob/main/LICENSE).
