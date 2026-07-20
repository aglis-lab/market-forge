import json
import glob
import csv
import os
from collections import defaultdict

# Keep this in sync with benches/perf_order.rs so stale criterion folders do not
# get exported after benchmark sizes change.
ALLOWED_PARAMS = {
    "perf_order_cancel": {
        100_000,
        200_000,
        300_000,
        400_000,
        600_000,
        800_000,
        1_000_000,
        2_000_000,
        5_000_000,
        10_000_000,
    },
    "perf_order_replace": {
        100_000,
        200_000,
        300_000,
        400_000,
        600_000,
        800_000,
        1_000_000,
        2_000_000,
        5_000_000,
        10_000_000,
    },
    "perf_order_matching": {
        100_000,
        200_000,
        300_000,
        400_000,
        600_000,
        800_000,
        1_000_000,
        2_000_000,
        5_000_000,
        10_000_000,
        20_000_000,
        30_000_000,
    },
    "perf_order_combined": {
        100_000,
        200_000,
        300_000,
        400_000,
        600_000,
        800_000,
        1_000_000,
        2_000_000,
        5_000_000,
        10_000_000,
    },
}

# group -> list of row dicts
groups = defaultdict(list)

for path in glob.glob("target/criterion/**/new/estimates.json", recursive=True):
    parts = path.replace("\\", "/").split("/")
    # path: target/criterion/<group>/<param>/new/estimates.json
    if len(parts) < 5:
        continue

    group = parts[2]
    param = parts[3]

    # skip non-numeric params (e.g. "report")
    if not param.isdigit():
        continue

    with open(path) as f:
        est = json.load(f)

    mean_ns = est["mean"]["point_estimate"]
    std_ns  = est["std_dev"]["point_estimate"]
    n       = int(param)

    allowed_params = ALLOWED_PARAMS.get(group)
    if allowed_params is not None and n not in allowed_params:
        continue

    # Apply throughput multiplier based on benchmark type (must match Rust code)
    multiplier = 1.0
    if group == "perf_order_combined":
        multiplier = 1.3  # insert + 20% replace + 10% cancel

    total_ops = n * multiplier
    groups[group].append({
        "n":           n,
        "duration_ms": round(mean_ns / 1e6, 2),        # total bench duration
        "latency_ns":  f"{round(mean_ns / total_ops, 6):.6f}",           # per-operation latency
        "std_ns":      round(std_ns / total_ops, 3),             # per-operation std dev
        "throughput":  round(total_ops * 1e9 / mean_ns / 1e3, 3),  # Kelem/s
    })

os.makedirs("doc/stats", exist_ok=True)

fieldnames = ["n", "duration_ms", "latency_ns", "std_ns", "throughput"]

for group, rows in groups.items():
    rows.sort(key=lambda r: r["n"])

    # sanitize group name for use as filename
    safe_name = group.replace("/", "_").replace(" ", "_")
    out_path = f"doc/stats/{safe_name}.csv"

    with open(out_path, "w", newline="") as f:
        writer = csv.DictWriter(f, fieldnames=fieldnames)
        writer.writeheader()
        writer.writerows(rows)

    print(f"[{group}] → {out_path} ({len(rows)} rows)")
