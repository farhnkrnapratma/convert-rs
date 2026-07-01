# Security Policy

## Reporting a Vulnerability

**Do not open a public GitHub issue for security vulnerabilities.**

Report security issues privately via email:

**[contact@fkp.my.id](mailto:contact@fkp.my.id)**

Use the subject format:

```
[SECURITY] <Short Description>
```

---

## Steps to Report

1. **Identify** the vulnerability type (e.g., DoS, injection, information disclosure).
2. **Assess the impact** — what can an attacker achieve, and who is affected?
3. **Prepare a Proof of Concept (PoC)** — see the template below.
4. **Send an email** to [contact@fkp.my.id](mailto:contact@fkp.my.id) with the following structure:

```
Summary:
  A brief description of the vulnerability.

Affected Component:
  Which endpoint, module, or function is affected.

Severity:
  Critical / High / Medium / Low

Steps to Reproduce:
  1. ...
  2. ...

Proof of Concept:
  <see template below>

Expected Behavior:
  What should happen.

Actual Behavior:
  What actually happens.

Impact:
  What an attacker can achieve by exploiting this.
```

---

## Proof of Concept (PoC) Template

```bash
# PoC: <short description>
# Version: <convert-rs version>
# Date: <YYYY-MM-DD>

# 1. Start the server
cargo run

# 2. Send the crafted request
curl -v "http://127.0.0.1:8080/<endpoint>?<payload>"

# 3. Observe the response
# Expected: <normal behavior>
# Actual:   <vulnerable behavior>
```

---

## Response Timeline

| Stage              | Time         |
| ------------------ | ------------ |
| Acknowledgment     | Within 72 hours |
| Initial assessment | Within 14 days  |
| Fix & disclosure   | Within 30 days  |

Please do not disclose the vulnerability publicly until a fix has been released. Credit will be given to the reporter unless anonymity is preferred.
