Некоторые условия, например, `target_os` предоставляются компилятором. Если мы хотим создать собственные условия, то их необходимо передать компилятору используя флаг `--cfg`.  

{custom.rs}

Без указания флага `cfg`:

{custom.out}

С указанием флага `cfg`:

```
$ rustc --cfg some_condition custom.rs && ./custom
condition met!
```
