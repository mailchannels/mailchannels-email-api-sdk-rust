"""Apply reviewed SDK names and a batch-ID representation hint."""
import copy
import json
from pathlib import Path
import re
import sys
import yaml

root = Path(__file__).resolve().parent
source = yaml.safe_load((root / 'spec/email-api.yaml').read_text())
names = json.loads((root / 'operation-names.json').read_text())
methods = {'get', 'post', 'put', 'patch', 'delete', 'head', 'options', 'trace'}
operations = {f'{method.upper()} {path}': operation for path, item in source['paths'].items()
              for method, operation in item.items() if method in methods}
if set(operations) != set(names):
    raise SystemExit(f'Operation map mismatch: missing={set(operations)-set(names)}, stale={set(names)-set(operations)}')
if len(set(names.values())) != len(names) or any(not re.fullmatch(r'[a-z][A-Za-z0-9]*', name) for name in names.values()):
    raise SystemExit('Operation names must be unique camelCase identifiers')
original = copy.deepcopy(source)
for key, operation in operations.items():
    if operation.get('operationId', names[key]) != names[key]:
        raise SystemExit(f'Upstream operationId changed for {key}; review before overriding')
    operation['operationId'] = names[key]
# The API returns int64 batch IDs; an unformatted integer path parameter
# otherwise becomes i32 and cannot resend all returned IDs.
parameters = source['paths']['/webhook-batch/{batch_id}/resend']['post']['parameters']
identifier = next(p for p in parameters if p['in'] == 'path' and p['name'] == 'batch_id')
if identifier['schema'] != {'type': 'integer'}:
    raise SystemExit('Batch ID schema changed upstream; review representation hint')
identifier['schema']['format'] = 'int64'
# Verify the only differences are operationId and this explicit format hint.
round_trip = copy.deepcopy(source)
for path, item in round_trip['paths'].items():
    for method, operation in item.items():
        if method in methods:
            operation.pop('operationId', None)
            if 'operationId' in original['paths'][path][method]:
                operation['operationId'] = original['paths'][path][method]['operationId']
next(p for p in round_trip['paths']['/webhook-batch/{batch_id}/resend']['post']['parameters']
     if p['in'] == 'path' and p['name'] == 'batch_id')['schema'].pop('format')
assert round_trip == original
output = Path(sys.argv[1])
output.parent.mkdir(parents=True, exist_ok=True)
output.write_text(json.dumps(source, indent=2) + '\n')
print(f'Prepared {len(names)} explicitly named operations; routes/bodies unchanged; one int64 batch-ID format hint.')
