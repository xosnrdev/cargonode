Configuration precedence

- [x] Parse configuration as CLI arguments
- [x] Parse configuration from a configuration file likely from a key in the `package.json` file

Template Management

- [x] Local Template Source:
      Compressed .tar.gz file embedded in the binary using flate2.
      Extracted during project initialization.
- [ ] Optional Remote Template Source:
      Fetch latest from a remote source like git.
      Compare local and remote version metadata.
