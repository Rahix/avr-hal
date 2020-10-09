#!/usr/bin/env python3

import subprocess
import json
import copy
import os

SPECS = {
    "atmega32u4": {
        "cpu": "atmega32u4",
        "no-default-libraries": False,
    },
    "atmega48p": {
        "cpu": "atmega48p",
        "no-default-libraries": False,
    },
    "atmega168": {
        "cpu": "atmega168",
        "no-default-libraries": False,
    },
    "atmega328p": {
        "cpu": "atmega328p",
        "no-default-libraries": False,
    },
    "atmega1280": {
        "cpu": "atmega1280",
        "no-default-libraries": False,
    },
    "atmega2560": {
        "cpu": "atmega2560",
        "no-default-libraries": False,
    },
    "attiny85": {
        "cpu": "attiny85",
        "no-default-libraries": False,
    },
    "attiny88": {
        "cpu": "attiny88",
        "no-default-libraries": False,
    },
}


def main():
    rustc_version = subprocess.run(
        ['rustc', '--version'],
        check=True,
        stdout=subprocess.PIPE,
    ).stdout.decode()

    if 'nightly' not in rustc_version:
        raise RuntimeError('You need nightly rustc!')

    upstream_spec_string = subprocess.run(
        ['rustc', '--print', 'target-spec-json', '-Z',
            'unstable-options', '--target', 'avr-unknown-gnu-atmega328'],
        check=True,
        stdout=subprocess.PIPE,
    ).stdout

    if upstream_spec_string is not None:
        upstream_spec = json.loads(upstream_spec_string)

    for mcu, settings in SPECS.items():
        spec = copy.deepcopy(upstream_spec)
        spec.update(settings)
        spec['pre-link-args']['gcc'][0] = f"-mmcu={mcu}"

        with open(f"avr-specs/avr-{mcu}.json", "w") as f:
            json.dump(spec, f, sort_keys=True, indent=2)
            f.write("\n")


if __name__ == "__main__":
    main()
