# quickcfg
[![Build Status](https://travis-ci.org/udoprog/quickcfg.svg?branch=master)](https://travis-ci.org/udoprog/quickcfg)

Apply a base configuration to a system, quickly!

It reads a configuration and template structure from a [dotfiles] directory and tries to normalize
the machine that it is run base on this configuration.

Until Rust Edition 2018 is released, this crate is _Nightly Only_.

**WARNING**:
This project is currently in development.
I've tried my best to make all operations non-destructive, but beware of bugs!

[dotfiles]: https://github.com/udoprog/dotfiles

![Example](gfx/example1.gif)

## Features

**Zero dependencies**, All you need is the `quickcfg` binary and your configuration repo.

**Blazingly fast**, multi-threaded and uses a simple dependency graph to determine when things can
run in parallel.

**Flexible but opinionated manifests**, There are a couple of powerful primitives available
(e.g. `copy-dir`), which does _a lot_ of work with very little configuration.

**Uses fast checksumming**, to reduce the amount of unnecessary work. Only applies changes when it
has to.

## Automatically applying updates

If you want quickcfg to periodically check your git repositories for updates, you can add the
following to your `.zshrc` or `.bashrc`:

```bash
if command -v quickcfg > /dev/null 2>&1; then
    quickcfg --updates-only --root $HOME/.dotfiles
    alias upd="quickcfg --root $HOME/.dotfiles"
fi
```

Every time you open a shell quickcfg will not check if your dotfiles are up-to-date.

You control how frequently by setting the `git_refresh` option in `quickcfg.yml`:

```
git_refresh: 3d
```

## Configuration

Create a repository with a `quickcfg.yml` in its root:

```
git_refresh: 1d

hierarchy:
  - secrets.yaml
  - db/common.yaml
  - db/{distro}.yaml

systems:
  # System to ensure that a set of packages are installed.
  - type: install-packages
```

The [`hierarchy`] specifies a set of files that should be looked for.
These can use variables like `{distro}`, which will be expanded based on the facts known of the
system you are running on.

You can use my [dotfiles](https://github.com/udoprog/dotfiles) repository as inspiration.

The following section will detail all the systems which are available.

[`hierarchy`]: #hierarchy

## Hierarchy

The hierarchy is a collection of files which contain data.

Some systems query the hierarchy for information, like the `key` setting in [`install-packages`].
This then determines which packages should be installed.

Hierarchy variables can also be made available in [`templates`] by adding a `quickcfg:` tag at the
top of the template.

[`install-packages`]: #install-packages
[`templates`]: #templating

## Systems

#### `copy-dir`

Copies a directory recursively.

```yaml
type: copy-dir
from: ./some/dir
to: home:some/dir
templates: false
```

Will copy a directory recursively.

#### `link-dir`

Links a directory recursively.

```yaml
type: link-dir
# Directory to link from.
from: ./some/dir
# Directory to link towards.
to: home:some/dir
```

Will create the corresponding directory structure, but all files will be symbolic links.

#### `git-sync`

System that syncs a single git repository to some path.

```yaml
type: git-sync
# Where to clone.
path: home://.oh-my-zsh
# Remote to clone.
remote: https://github.com/robbyrussell/oh-my-zsh.git 
# Refresh once per day.
refresh: 1d
```

#### `install-packages`

Compares the set of installed packages, with a set of packages from the hierarchy to install and
installs any that are missing.

Will use `sudo` if needed to install packages.

```yaml
type: install-packages
# The provider of the package manager to use.
provider: pip3
# Hierarchy key to lookup for packages to install.
key: pip3::packages
```

The simplest example of this system is the one that uses the primary provider:

```yaml
systems:
  - type: install-packages
```

This will look up packages under the `packages` key and install it using the primary provider for
the system that you are currently running.

These are the supported providers:

 * `debian`: For Debian-based systems. This is a _primary_ provider.
 * `pip`: The Python 2 package manager.
 * `pip3`: The Python 3 package manager.
 * `gem`: The Ruby package manager.

By default, any _primary_ provider will be the default provider of the system if it can be
detected.

Explicitly configured providers look up packages based on the hierarchy key `<provider>::packages`.
Default providers use the key `packages`.

#### `download-and-run`

Downloads a script of the internet and runs it once.

```yaml
type: download-and-run
id: install-oh-my-zsh
url: https://raw.githubusercontent.com/robbyrussell/oh-my-zsh/master/tools/install.sh
shell: true
```

The `id` is to uniquely identify that this system has only been run once.

#### `link`

Creates a symlink.

```yaml
type: link
path: home:.vimrc
link: .vim/vimrc
```

This creates a symbolic link at `path` which contains whatever is specified in `link`.

## Templating

Some systems treats files as templates, like [`copy-dir`] when the `templating` option is enabled.
Any file being copied is then treated as a [`handlebars`] template.

Any template file can make use of hierarchy data, by specifying their dependencies using
a `quickcfg:` tag at the top of the file, like this:

```
# quickcfg: name, hobbies:array

Hi, my name is {{name}}

My hobbies are:
{{#each hobbies}}
- {{this}}
{{/#each}}
```

This will load the `name` and `hobbies` variables out of the [`hierarchy`].
`hobbies` will be loaded as an array, causing all values in the hierarchy for that value to be
loaded.

[`copy-dir`]: #copy-dir
[`handlebars`]: https://handlebarsjs.com/
