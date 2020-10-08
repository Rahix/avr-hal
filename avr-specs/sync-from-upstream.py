#!/usr/bin/env python3

import subprocess
import json
import copy
import os

SPECS = {
    "atmega32u4": {
        "cpu": "atmega32u4",
    },
    "atmega48p": {
        "cpu": "atmega48p",
    },
    "atmega168": {
        "cpu": "atmega168",
    },
    "atmega328p": {
        "cpu": "atmega328p",
    },
    "atmega1280": {
        "cpu": "atmega1280",
    },
    "atmega2560": {
        "cpu": "atmega2560",
    },
    "attiny85": {
        "cpu": "attiny85",
    },
    "attiny88": {
        "cpu": "attiny88",
    },
}


def main():
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
        spec['pre-link-args']['gcc'][0] = '-mmcu=%s' % mcu

        with open(f"avr-specs/avr-{mcu}.json", "w") as f:
            json.dump(spec, f, sort_keys=True, indent=2)
            f.write("\n")


if __name__ == "__main__":
    main()
