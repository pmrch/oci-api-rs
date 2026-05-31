# OCI API Rust

## ⚡ About
This library is built to communicate with the Oracle Cloud Infrastructure (OCI).
Performance hasn't been benchmarked yet in any meaningful way, but it's not slow by any means.
Primary libraries involved are **reqwest**, **reqsign**, **serde** and **serde_json**.

Supports builder type for the **LaunchInstanceDetails** object, executed with **derive_builder**.
The structure of **LaunchInstanceDetails** is largely based off of the Python and Go source code of the
official OCI SDK, as well as API docs as referenced below.

## 🧰 Requirements

- Rust 1.84 or newer *(not a proven fact, was developed and tested on 1.98)*
- The `tokio` runtime with at least `rt-multi-thread` and `macros`
- A `.env` file properly configured

### Example `.env` file:
```
SSH_PUBLIC_KEY="your_ssh_public_key"
SUBNET_OCID="your_subnet_ocid"
IMAGE_OCID="the_ocid_of_the_image"
```

## 📥 Installation
### GitHub
Add the following lines to your `Cargo.toml` in your project root:

```toml
[dependencies]
oci-api-rs = { git = "https://github.com/pmrch/oci-api-rs.git", branch = "main" }
```

### crates.io (uncertain)
Once proven useful and necessary, I will publish on crates.io.

---

## 📜 License

MIT © 2026  
Free to use, modify, and adapt with attribution.

---

## References
- Python source code [OCI Python SDK](https://github.com/oracle/oci-python-sdk)
- Go source code reference for requests: [OCI Go SDK](https://github.com/oracle/oci-go-sdk)
- API reference for requests: [Oracle Cloud Docs](https://docs.oracle.com/en-us/iaas/api/#/en/iaas/20160918/LaunchInstanceDetails)

## 🤖 Acknowledgements
Built with the help of Claude (Anthropic) for design decisions, code review, and rubber duck debugging sessions that somehow 
turned into a full OCI signing implementation.