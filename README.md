## WARNING

This is a toy PostgreSQL extension that exposes basic libjq functionality to PostgreSQL.

Do not use for anything serious. It might crash your DB server. You have been warned.

This "project" depends on [the amazing pgx project.](https://github.com/zombodb/pgx)

To compile you must have libjq installed, often it is installed by installing jq itself.
Use your favorite package manager to install it.

If your package manager installs things outside of the default search paths for your default
compiler toolchain you'll need to tell the jq crates where to find the libraries using
the enviornment variable `JQ_LIB_DIR`. For example on macOS using MacPorts one would use `JQ_LIB_DIR=/opt/local/lib`.

Reference the pgx documentation for information how to work with extensions using pgx. Its documentation is really good.


### Installation Notes
This is purposefully empty. I don't want anyone to attempt to use this terrible thing in a real environment.
