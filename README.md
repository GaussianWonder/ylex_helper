# Makefile Generator for YACC/LEX

This projects aims to provide a **simple** way to **run** and **test** program fragments generated from **YACC/LEX** files.

## Usage

```bash
# Help
cargo run help
# Help of run subcommand
cargo run run --help
```

**Expected file structure:**

```text
resources \
  resource_name
    input.txt
    resource_name.y
    resource_name.l
    <?Makefile.template>
<?Makefile.template>
```

> Files marked in <? > are **optional**, but at least one is required.

Now, each resource can be built and run using

```bash
cargo run resource_name
```

> Using the run -f [file] you can override the default input.txt file.

## Output

Normally, the desired build method will be setup (Make / CMake / ...).

The console output of the utility script is stripped of ANSI Escape Codes, so no colored output, however the output of the program is still releavant. It retains all runtime information, build errors, warnings, etc...

A `run.sh` script is generated, so you can still quickly run and view colored output.
