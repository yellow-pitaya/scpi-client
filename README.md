# Redpitaya SCPI

[![Build Status](https://travis-ci.org/sanpii/redpitaya-scpi.svg?branch=master)](https://travis-ci.org/sanpii/redpitaya-scpi)
[![.github/workflows/ci.yml](https://github.com/yellow-pitaya/scpi-client/actions/workflows/ci.yml/badge.svg)](https://github.com/yellow-pitaya/scpi-client/actions/workflows/ci.yml)
[![pipeline status](https://gitlab.com/yellow-pitaya/scpi-client/badges/main/pipeline.svg)](https://gitlab.com/yellow-pitaya/scpi-client/-/commits/main)

Controlling your redpitaya via SCPI commands in rust.

# Installation

In your Cargo.toml:

```toml
[dependencies]
redpitaya-scpi = "0.22"
```

# Usage

## Setup

Firt, you need to enable the SCPI server on your redpitaya, vi ssh:

```
# systemctl start redpitaya_scpi
```

You can permantly enable this service:

```
# systemctl enable redpitaya_scpi
```

You can also enable it via the web interface, see the [official
documentation](https://redpitaya.readthedocs.io/en/latest/doc/appsFeatures/remoteControl/remoteControl.html#quick-start)
for instructions.

## Examples

Build examples with cargo:

```
cargo run --release --example k2000
```
