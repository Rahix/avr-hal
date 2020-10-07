#!/usr/bin/python3

import subprocess
import json
import os

ENCODING = 'utf-8'


def main():
    process = subprocess.Popen(['rustc', '--print', 'target-spec-json', '-Z', 'unstable-options', '--target', 'avr-unknown-gnu-atmega328'],
                               stdout=subprocess.PIPE,
                               stderr=subprocess.PIPE)

    ref_spec_string, stderr = process.communicate()

    if ref_spec_string is not None:
        ref_spec = json.loads(ref_spec_string)

        specs_path = 'avr-specs'
        paths = find_spec_paths(specs_path)

        for spec_path in paths:
            legacy_spec = read_spec(spec_path)
            new_spec = merge_specs(legacy_spec, ref_spec)
            write_spec(spec_path, new_spec)
    else:
        print(stderr)


def find_spec_paths(base_path):
    paths = []
    for i in os.listdir(base_path):
        if i.endswith('.json'):
            full_path = '%s/%s' % (base_path, i)
            paths.append(full_path)
    return paths


def read_spec(path):
    legacy_file = open(path, 'r', encoding=ENCODING)
    legacy_string = legacy_file.read()
    legacy_file.close()
    return json.loads(legacy_string)


def write_spec(spec_path, spec):
    legacy_file = open(spec_path, 'w',  encoding=ENCODING)
    json.dump(spec, legacy_file, sort_keys=True, indent=2)
    legacy_file.write('\n')
    legacy_file.close()


def merge_specs(legacy, ref):
    cpu = legacy['cpu']
    data_layout = legacy['data-layout']
    ref['cpu'] = cpu
    ref['data-layout'] = data_layout
    ref['pre-link-args']['gcc'][0] = '-mmcu=%s' % cpu
    return ref


if __name__ == "__main__":
    main()
