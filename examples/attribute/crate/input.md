Атрибут `crate_type` используется, чтобы сказать компилятору, какой контейнер является библиотекой (и каким типом библиотеки), а какой исполняемым файлом. Атрибут `crate_name` используется для указания имя контейнера.

{lib.rs}

Если мы используем атрибут `crate_type`, то нам больше нет необходимости передавать флаг `--crate-type` компилятору.

```
$ rustc lib.rs
$ ls lib*
library.rlib
```
