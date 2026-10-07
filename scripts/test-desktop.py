"""Run existing loopback fixtures on a host-native TLS stack; no system trust edits."""
import argparse
import platform
import subprocess
import sys
from pathlib import Path

parser = argparse.ArgumentParser()
parser.add_argument('--toolchain', required=True)
args = parser.parse_args()
root = Path(__file__).resolve().parents[1]

def run(*command):
    subprocess.run(command, cwd=root, check=True)

print('RUST_DESKTOP_PLATFORM', platform.platform(), platform.machine(), flush=True)
run('rustc', '+' + args.toolchain, '--version')
run(sys.executable, 'scripts/tls-certificates.py', 'tests/tls-fixtures')
run('cargo', '+' + args.toolchain, 'fetch', '--locked')
features = subprocess.check_output(
    ['cargo', '+' + args.toolchain, 'tree', '--offline', '--locked',
     '--no-default-features', '--features', 'native-tls', '-e', 'features', '-i', 'reqwest'],
    cwd=root, text=True,
)
print(features, flush=True)
assert 'reqwest feature "__native-tls"' in features
assert 'reqwest feature "__rustls"' not in features, 'Unexpected client TLS backend'
run('cargo', '+' + args.toolchain, 'test', '--offline', '--locked',
    '--no-default-features', '--features', 'native-tls', '--tests')
print('RUST_DESKTOP_COMPLETE', args.toolchain, 'native-tls', flush=True)
