# updlockfiles

Manage lockfiles for packages that don't ship any upstream. Like `updpkgsums`
but for 3rd party dependency trees.

If you're not actively maintaining Arch Linux packages you likely don't need
this tool.

## Getting started

Add a function like this to your PKGBUILD:
```sh
updlockfiles() {
  cd ${pkgname}
  rm -f composer.lock
  composer update
  cp composer.lock "${outdir}/"
}
```

This works for arbitrary files, just make sure the files you want copied back
need to be copied into `$outdir`.

Next run this command (no arguments needed, the default should _just work_):
```
updlockfiles
```

Finally add the new file to your source array (and make sure it's part of your next commit!):
```
source=("git+https://github.com/vimeo/psalm.git#commit=${_commit}"
        "composer.lock")
```

Update the checksums for content pinning:
```
updpkgsums
```

## Update a lockfile

If the initial setup was done before you can generate a new lockfile of the latest patch level like this:
```
updlockfiles
```

## License

GPLv3+
