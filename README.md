# ness

# Dependencies

* MariaDb
* openssl
* cargo
* Alsa libs
* pocketsphinx
* gcc
* notify-send

# Environment Variable

```shell
echo "set -x NESS_USERNAME ness" >> $HOME/.config/fish/config.fish
```   

```shell
echo "set -x NESS_DBNAME ness" >> $HOME/.config/fish/config.fish
```

```shell
echo "set -x NESS_PASSWORD ness" >> $HOME/.config/fish/config.fish
```

```shell
echo "set -x EDITOR vim" >> $HOME/.config/fish/config.fish
```

# Init the database

```bash
ness init 
```

# Re init the database

```bash
ness --re-init 
```

### Indexes Music directory content

```bash
ness --save-albums
```

### Playing music

```bash
ness --listen-track [name]
```

```bash
ness --listen-album [name]
```
