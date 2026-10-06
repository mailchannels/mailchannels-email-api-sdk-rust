from pathlib import Path
root=Path(__file__).resolve().parents[1]
for path in [root/'README.md',*(root/'docs').glob('*.md')]:
    path.write_text('\n'.join(line.rstrip() for line in path.read_text().splitlines()).rstrip()+'\n')
