#!/bin/bash

mkdir -p app

hash=$(ls dist/geojson-source*.js | sed -e 's/.*geojson-source-//g' -e 's/.js//g')

cat << EOF >dist/app.js
import init from './geojson-source-$hash'
init()
EOF

cat << EOF >app/index.html
<!DOCTYPE html>
<html>
  <head>
    <meta charset="utf-8">
    <title>App</title>
    <link href='https://api.mapbox.com/mapbox-gl-js/v2.10.0/mapbox-gl.css' rel='stylesheet' />
    <meta name="viewport" content="width=device-width, initial-scale=1">
    <script defer src="main.js"></script>
  </head>
  <body style="margin: 0; padding: 0">
  </body>
</html>
EOF
