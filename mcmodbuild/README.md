# mcmodbuild

McModBuild is a format for describing how mods should be built and automatically building them from a file.

## Format

```yaml
# yourmod.yml
id: "yourmod"
name: "Test mod"
git: "https://github.com/you/yourmod.git"
branch: "1.21.7"
build: Cmd # Can be Std
cmd: "./build.sh" # If build is set to Std, cmd isn't needed as it'll run ./gradlew build
out: "dir:@/build/libs" # Directory to find the jar, can be file:... to specify a file, @ is the build root
exclude:
  # Files to exclude if out is a directory, if it's a file, set it to []
  - type: Ends
    value: "-sources.jar"
  - type: Starts
    value: "tmp-"
  - type: Contains
    value: "dev"
```

## Commands

### `mcmodbuild build <path-to-build-file>`

Converts a build file into an installation file named "\<id>.mcmodbuild".
Optional arguments:

- `-d`: Destination. Example: `mcmodbuild build testmod.yml -d dist/mod.mcmodbuild`

### `mcmodbuild install <path-to-binary-file>`

Builds a mod from an installation file.
Optional arguments:

- `-d`: Destination. Example: `mcmodbuild install testmod.mcmodbuild -d mods/`
