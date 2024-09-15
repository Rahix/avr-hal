#!/usr/bin/env python3
import copy
import json
import subprocess

SPECS = {
    "atmega32u4": {
        "cpu": "atmega32u4",
    },
    "atmega48p": {
        "cpu": "atmega48p",
    },
    "atmega164pa": {
        "cpu": "atmega164pa",
    },
    "atmega168": {
        "cpu": "atmega168",
    },
    "atmega32a": {
        "cpu": "atmega32a"
    },
    "atmega328": {
        "cpu": "atmega328",
    },
    "atmega328p": {
        "cpu": "atmega328p",
    },
    "atmega128a": {
        "cpu": "atmega128a",
    },
    "atmega1280": {
        "cpu": "atmega1280",
    },
    "atmega2560": {
        "cpu": "atmega2560",
    },
    "atmega1284p": {
        "cpu": "atmega1284p",
    },
    "atmega8": {
        "cpu": "atmega8",
    },
    "attiny85": {
        "cpu": "attiny85",
    },
    "attiny88": {
        "cpu": "attiny88",
    },
    "attiny167": {
        "cpu": "attiny167",
    },
    "attiny2313": {
        "cpu": "attiny2313",
    },
}

COMMON = {
    # needed because we currently rely on avr-libc
    "no-default-libraries": False,
    # 8-bit operations on AVR are atomic
    # LLVM also supports 16-bit atomics by disabling interrupts
    # see also https://github.com/rust-lang/rust/pull/114495
    "max-atomic-width": 16,
}


def main():
    rustc_version = subprocess.run(
        ["rustc", "--version"],
        check=True,
        stdout=subprocess.PIPE,
    ).stdout.decode()

    if "nightly" not in rustc_version:
        raise Exception("You need nightly rustc!")

    upstream_spec_string = subprocess.run(
        [
            "rustc",
            "--print",
            "target-spec-json",
            "-Z",
            "unstable-options",
            "--target",
            "avr-unknown-gnu-atmega328",
        ],
        check=True,
        stdout=subprocess.PIPE,
    ).stdout

    upstream_spec = json.loads(upstream_spec_string)

    # our targets are of course not built into rustc
    del upstream_spec["is-builtin"]

    for mcu, settings in SPECS.items():
        spec = copy.deepcopy(upstream_spec)
        spec.update(COMMON)
        spec.update(settings)

        for pre_link_args in spec["pre-link-args"].values():
            pre_link_args[0] = f"-mmcu={settings['cpu']}"
            pre_link_args.append("-Wl,--as-needed,--print-memory-usage")

        with open(f"avr-specs/avr-{mcu}.json", "w") as f:
            json.dump(spec, f, sort_keys=True, indent=2)
            f.write("\n")


if __name__ == "__main__":
    main()
