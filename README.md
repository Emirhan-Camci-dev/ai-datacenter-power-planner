# AIGrid-Planner (HyperScale-Twin-Planner) 🚀

Enterprise-grade AI Data Center Power & Cooling Bottleneck Capacity Planning Engine.

Designed for **AI Hyperscale Data Center Operators** (AWS, Microsoft, Google, Equinix, Meta), **Engineering EPCs**, and **Critical Cooling OEMs**. Deterministically model dynamic multi-megawatt AI training/inference load spikes, stress-test Substation Transformer & Cable headroom (IEEE C57.91, IEC 60287), and execute N+1/2N redundancy simulations.

## ⚡ Quickstart (Python SDK)

Easily integrate the high-performance Rust core into your Python workflows with just 3 lines of code:

```python
import aigrid_planner as agp

# 1. Load 24h AI training power trace and define infrastructure constraints
facility = agp.Facility(it_load_mw_trace=[12.4, 15.8, ...], cooling_type="D2C_Liquid")

# 2. Stress-test N+1 transformer headroom and evaluate hot-spot rises (IEEE C57.91)
bottlenecks = facility.run_dynamic_simulation(ambient_temp_c=35.0, fail_unit_mw=5.0)

# 3. Export compliance metrics (PUE/WUE) and safety margins
agp.report.export_pdf(bottlenecks, filename="capacity_audit.pdf")
```

## 📊 Performance & Benchmarks

The core multi-physics solver is written in **Rust** with PyO3 bindings, guaranteeing memory safety and extreme performance:
- **Execution Speed:** <100ms for a full 24-hour dynamic profile at 1-second resolution.
- **Memory Profiling:** Zero-allocation hot loops and strictly scoped data structures ensure **no memory leaks** during long Monte Carlo sensitivity sweeps.

## 🛡️ Compliance Matrix
- **Transformer Thermal:** IEEE C57.91 / IEC 60076-7
- **Cable Ampacity:** IEC 60287
- **Thermal Envelopes:** ASHRAE TC 9.9
- **Redundancy:** N+1, 2N Tier III/IV Failure modeling

## ⚖️ Open-Core & Licensing

AIGrid-Planner follows a **Dual-Licensing** Open-Core model:

| Feature | Community Edition (Free) | Enterprise Edition (Pro) |
| :--- | :--- | :--- |
| **License** | AGPLv3 | Proprietary / Commercial |
| **Simulation** | Static Steady-State | Full Time-Series Dynamic MW Load |
| **Thermal** | Basic Fixed-PUE | Coupled IEEE C57.91 Hot-Spot & Liquid DAE Solver |
| **Redundancy** | Single Transformer Overload | N+1 / 2N Emergency Failure Analysis |
| **Reporting** | Basic CLI Report (<10 MW) | Automated PDF Engineering Audit (500+ MW) |

### 💼 Get the Enterprise Edition

The Enterprise license is designed for Facility Planners & EPCs ($18,000 – $48,000/year per facility/project). 
It includes an **Offline Ed25519 Cryptographic License Validator**, allowing secure on-device execution without internet dependency.

👉 [**Purchase Enterprise License via Polar.sh**](https://polar.sh/aigrid-planner)

---
*Author: Emirhan CAMCI (@byemir) | © 2026*
