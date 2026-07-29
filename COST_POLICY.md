# Zero-Incremental-Cost Development Policy

ExecLocus is developed with a default project budget of **0 JPY in incremental
spend**. A contribution must not create a new charge, consume pay-as-you-go
credit, or enable a billable service unless the repository owner explicitly
approves the exception before use.

This is a process and configuration policy, not a guarantee about a
contributor's existing internet, hardware, operating-system, or subscription
costs.

## Default classification

### Allowed without cost approval

- local tools and dependencies that can be installed and used without a usage
  fee, subject to their licenses;
- local, offline builds, tests, linters, and documentation generation;
- standard GitHub-hosted runners for this public repository, while GitHub's
  public-repository terms keep that runner usage free;
- read-only research using public documentation; and
- services whose account and project billing dashboards both confirm that the
  planned operation cannot create a charge.

### Requires explicit approval before use

- metered APIs, hosted AI model calls, cloud compute, paid runners, paid
  Marketplace actions, or any service capable of pay-as-you-go billing;
- API keys, cloud-provider credentials, billing accounts, usage credits, or a
  payment method used by project automation;
- trials, promotional credits, and quota included in a paid subscription;
- artifact, cache, package, or release storage that could exceed a free
  allowance; and
- any product whose current price or billing behavior cannot be verified.

Installing a free client does not authorize use of a paid backend. For example,
an agent CLI may be free to download while its model calls consume a paid
subscription quota or metered API credit.

## Approval record

Before an exception is used, record all of the following in a private approval
note or a public issue that contains no secrets:

1. the operation and why it is necessary;
2. the service and authentication route;
3. whether it consumes a subscription quota, credit, or pay-as-you-go balance;
4. the maximum possible incremental charge;
5. the data that leaves the machine; and
6. the approval scope and expiry.

Approval for one operation does not authorize later calls or unrelated
services. Never commit credentials or personal billing information.

## CI and repository guardrails

- Workflows must use standard runners. Larger GitHub-hosted runners require an
  approved exception.
- Workflows must not call a hosted AI model or paid external API.
- Workflows must not require billing credentials or paid-service secrets.
- Cache, artifact, package, and release-retention changes require a cost review.
- Pull requests must declare whether they change the project's cost exposure.

If pricing or authentication is unclear, stop before the operation and request
the repository owner's decision.
