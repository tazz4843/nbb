# nbb
### no bullshit blogging

copy markdown files to your data directory,
and fire up the server: that's it to get started

if you want more reasons:
* blazing fast, but still insanely lightweight
* image/asset serving
* custom CSS + HTML supported on each page
* hidden blogposts

# installation

## standard
download the latest release from the [GitHub Releases page](
https://github.com/tazz4843/nbb/releases),
and download ``config.yaml`` from this repo

start with ``./nbb /path/to/config.yaml``

## docker
download ``docker-compose.yml``, and ``config.yaml`` and put both into the same directory

then run ``docker-compose up -d`` to start up

this binds to port 8080 by default: edit ``docker-compose.yml`` to change this port

## configuration

open up ``config.yaml`` in a text editor

everything is heavily commented with an explanation

## usage

put new ``.md`` files into ``./blog`` (by default)

then visit ``http://localhost:8080/blog/name`` where name is the name of the
file on disk, minus the ``.md`` extension

if you need static assets, place them in a new folder with the same name as the file,
minus extension

these assets will be available at ``/blog/name/:assets``

if you need *global* static assets, put them into ``/static``
where they will be available at ``/static/:assets``