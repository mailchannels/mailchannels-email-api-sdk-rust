"""Generate short-lived, fixture-only certificates; never add to system trust."""
from datetime import datetime, timedelta, timezone
from pathlib import Path
import os
import sys
from cryptography import x509
from cryptography.hazmat.primitives import hashes, serialization
from cryptography.hazmat.primitives.asymmetric import ec
from cryptography.x509.oid import NameOID, ExtendedKeyUsageOID

os.umask(0o077)
output = Path(sys.argv[1])
output.mkdir(parents=True, exist_ok=True)
now = datetime.now(timezone.utc)
key = ec.generate_private_key(ec.SECP256R1())
name = x509.Name([x509.NameAttribute(NameOID.COMMON_NAME, 'Ephemeral SDK fixture CA')])
ca = (x509.CertificateBuilder().subject_name(name).issuer_name(name).public_key(key.public_key())
      .serial_number(x509.random_serial_number()).not_valid_before(now-timedelta(days=1))
      .not_valid_after(now+timedelta(days=2)).add_extension(x509.BasicConstraints(ca=True,path_length=0),critical=True)
      .sign(key,hashes.SHA256()))
(output/'ca.der').write_bytes(ca.public_bytes(serialization.Encoding.DER))
for label, host, expired in [('valid','fixture.test',False),('wrong-host','other.test',False),('expired','fixture.test',True)]:
    leaf_key = ec.generate_private_key(ec.SECP256R1())
    subject = x509.Name([x509.NameAttribute(NameOID.COMMON_NAME,host)])
    cert = (x509.CertificateBuilder().subject_name(subject).issuer_name(name).public_key(leaf_key.public_key())
            .serial_number(x509.random_serial_number()).not_valid_before(now-timedelta(days=2))
            .not_valid_after(now-timedelta(days=1) if expired else now+timedelta(days=1))
            .add_extension(x509.BasicConstraints(ca=False,path_length=None),critical=True)
            .add_extension(x509.SubjectAlternativeName([x509.DNSName(host)]),critical=False)
            .add_extension(x509.ExtendedKeyUsage([ExtendedKeyUsageOID.SERVER_AUTH]),critical=False)
            .sign(key,hashes.SHA256()))
    (output/(label+'.der')).write_bytes(cert.public_bytes(serialization.Encoding.DER))
    (output/(label+'-key.der')).write_bytes(leaf_key.private_bytes(serialization.Encoding.DER,serialization.PrivateFormat.PKCS8,serialization.NoEncryption()))
