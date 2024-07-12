# Rust Flac Meta Reader
Simple Rust program to recursively detect flac files in your folders and read out its metadata. Can be used on huge data collections




## Prepare Database
Sqlite is used here along with an ORM called [Diesel](https://diesel.rs/).


```
# Linux/MacOS
curl --proto '=https' --tlsv1.2 -LsSf https://github.com/diesel-rs/diesel/releases/download/v2.2.1/diesel_cli-installer.sh | sh

# Windows
powershell -c "irm https://github.com/diesel-rs/diesel/releases/download/v2.2.1/diesel_cli-installer.ps1 | iex"
```

open new terminal in your root folder and dropping:


```
diesel setup
diesel migration generate create_posts
```

After this, a folder called migrations should be created and containing a file up.sql.

## Migration Schema

Put this into the up.sql file

```
CREATE TABLE posts (
  id INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT,
  title VARCHAR NOT NULL,
  body TEXT NOT NULL,
  published BOOLEAN NOT NULL DEFAULT 0
)
``` 

and this into down.sql

```
DROP TABLE posts
``` 

then create the table by:

``` 
diesel migration run
```



## Build and Run

```
cargo build
cargo run <Path to your Flac Music Collection>
```

