#!/bin/bash

for example in $(ls examples); do
  if [[ -d examples/${example} ]]; then
    new_html=https://gordon-f.github.io/rust-by-example-ru/${example}.html

    mkdir -p stage/_book/examples/${example}
    echo '<!DOCTYPE HTML>
<html lang="ru-RU">
  <head>
    <script type="text/javascript">
      window.location = "'${new_html}'"
    </script>
  </head>
  <body>
  </body>
</html>' > stage/_book/examples/${example}/README.html
  fi
done
