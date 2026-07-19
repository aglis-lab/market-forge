import json
import glob
import csv
import os
from collections import defaultdict

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

    groups[group].append({
        "n":           n,
        "duration_ms": round(mean_ns / 1e6, 2),        # total bench duration
        "latency_ns":  f"{round(mean_ns / n, 6):.6f}",           # per-order latency
        "std_ns":      round(std_ns / n, 3),             # per-order std dev
        "throughput":  round(n / (mean_ns / 1e9) / 1e6, 3),  # Melem/s
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
