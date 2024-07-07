# Finding Protected Domains With Python

Let's say you work for an organization with many domain names, and you have
been tasked with making sure each of those domain names is safe from malicious
modification. That is, each domain needs to be "locked".

This can be accomplished using the [`status`](http://localhost:3000/protocol/common_data_structures.html#status)
array of a domain, and if the domain does not have the right set of status values
then the domain's registrar should be contacted.

Here is a simple Python script to do this (source code is [here](https://github.com/anewton1998/rdap_guide/tree/main/src/examples/clients/python_whoisit)):

```python
{{#include ../examples/clients/python_whoisit/main.py}}
```

How does this work? Using the [whoisit](https://github.com/meeb/whoisit) library,
the code first initializes its bootstrap information, and then loops over a list
of domain names given on the command line.

For each domain, it compares the status values to verify that each domain has the
"client delete prohibited", "client transfer prohibited", and "client update prohibited"
values. If the domain does not have those values, it prints a warning and the name
of the domain registrar to contact.

---

This simple example uses the client's built-in [bootstrapping](../bootstrapping/iana.md) feature,
however it does not save the bootstrapping information for later use as is best practice and
suggested by the [whoisit](https://github.com/meeb/whoisit) library.

---

Here is an example of running the command:

```
python main.py foo.com bar.com bar.net fake.com example.com
OK: foo.com Status: ['client delete prohibited', 'client transfer prohibited', 'client update prohibited']
WARNING: bar.com is not properly protected. Contact GoDaddy.com, LLC. Status: ['delete prohibited', 'transfer prohibited', 'renew prohibited', 'update prohibited']
WARNING: bar.net is not properly protected. Contact GoDaddy.com, LLC. Status: ['delete prohibited', 'transfer prohibited', 'renew prohibited', 'update prohibited']
WARNING: fake.com is not properly protected. Contact IONOS SE. Status: ['client transfer prohibited']
OK: example.com Status: ['client delete prohibited', 'client transfer prohibited', 'client update prohibited']
```
