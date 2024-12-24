# Developing

## Translations
As new translation entries will be far and few between (due to the simplicity of the app), most people will just be improving the wording of certain existing entries or adding whole languages.

### Adding a new entry
Modify the source code, and anywhere where you need a translated string, use the `tr!` macro.

Then, install `xtr` with `cargo install xtr` and run:

```bash
xtr src/main.rs -o po/hello-rhino.pot
```

### Adding a new language
Add your language code to `po/LINGUAS`, and then run:

```bash
cp po/hello-rhino.pot po/<LANG>.po
```

And then you can work on that file.
