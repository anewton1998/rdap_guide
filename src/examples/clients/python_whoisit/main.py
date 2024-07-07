import sys
import whoisit

whoisit.bootstrap()

needed_status = set(["client delete prohibited", "client transfer prohibited", "client update prohibited"])

domains = sys.argv[1:]
for domain in domains:
  results = whoisit.domain(domain)
  registrar = results["entities"]["registrar"][0]["name"]
  status = results["status"]
  if needed_status.difference(status):
    print(f"WARNING: {domain} is not properly protected. Contact {registrar}. Status: {status}")
  else:
    print(f"OK: {domain} Status: {status}")


