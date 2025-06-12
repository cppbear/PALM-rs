import argparse
import json
import os
from html import escape
from pathlib import Path
import sys

import numpy as np


def get_includes(target_dirs) -> set:
    includes = set()
    for target_dir in target_dirs:
        for root, _, files in os.walk(target_dir):
            for file in files:
                if file.endswith(".json"):
                    with open(os.path.join(root, file), "r") as f:
                        data = json.load(f)
                        if data["tests_run"] > 0:
                            includes.add(
                                (data["function_name"], data["lines"], data["branches"])
                            )
    return includes


def generate_html_report(target_dir, includes):
    all_tests_gen = 0
    all_tests_lines_list = []
    all_can_compile = 0
    all_tests_run = 0
    all_tests_pass = 0
    all_oracles_gen = 0
    all_oracles_can_compile = 0
    all_oracles_run = 0
    all_oracles_pass = 0
    line_cov_list = []
    branch_cov_list = []
    project_lines = 0
    project_lines_covered = 0
    project_branches = 0
    project_branches_covered = 0
    effective_lines = 0
    effective_lines_covered = 0
    effective_branches = 0
    effective_branches_covered = 0
    function_num = 0
    test_rate_infos = []

    for root, _, files in os.walk(target_dir):
        for file in files:
            if file.endswith(".json"):
                with open(os.path.join(root, file), "r") as f:
                    data = json.load(f)
                    function = data["function_name"]
                    function_num += 1

                    # Collect function details
                    test_rate_info = {
                        "function_name": data["function_name"],
                        "lines": data["lines"],
                        "lines_covered": data["lines_covered"],
                        "line_coverage": float(data["lines_coveraged_rate"]),
                        "branches": data["branches"],
                        "branches_covered": data["branches_covered"],
                        "branch_coverage": float(data["branches_coverage_rate"]),
                        "tests_gen": data["tests"],
                        "tests_compiled": data["tests_compiled"],
                        "tests_run": data["tests_run"],
                        "tests_passed": data["tests_passed"],
                        "oracles_gen": data["oracles"],
                        "oracles_compiled": data["oracles_compiled"],
                        "oracles_run": data["oracles_run"],
                        "oracles_passed": data["oracles_passed"],
                    }
                    test_rate_infos.append(test_rate_info)

                    # Aggregate statistics
                    all_tests_gen += data["tests"]
                    all_tests_lines_list.extend(data["tests_lines"])
                    all_can_compile += data["tests_compiled"]
                    all_oracles_gen += data["oracles"]
                    all_oracles_can_compile += data["oracles_compiled"]

                    if data["tests_run"] > 0:
                        all_tests_run += data["tests_run"]
                        all_tests_pass += data["tests_passed"]
                        all_oracles_run += data["oracles_run"]
                        all_oracles_pass += data["oracles_passed"]
                        line_cov_list.append(float(data["lines_coveraged_rate"]))
                        branch_cov_list.append(float(data["branches_coverage_rate"]))

                    project_lines += data["lines"]
                    project_lines_covered += data["lines_covered"]
                    project_branches += data["branches"]
                    project_branches_covered += data["branches_covered"]

                    # Calculate effective coverage
                    for include in includes:
                        if include[0] == function:
                            effective_lines += include[1]
                            effective_lines_covered += data["lines_covered"]
                            effective_branches += include[2]
                            effective_branches_covered += data["branches_covered"]
                            break

    # Calculate metrics
    compile_pass_rate = all_can_compile / all_tests_gen if all_tests_gen > 0 else 0.0
    tests_pass_rate = all_tests_pass / all_tests_run if all_tests_run > 0 else 0.0
    oracles_compile_pass_rate = (
        all_oracles_can_compile / all_oracles_gen if all_oracles_gen > 0 else 0.0
    )
    oracles_tests_pass_rate = (
        all_oracles_pass / all_oracles_run if all_oracles_run > 0 else 0.0
    )
    effective_lines_cov = (
        effective_lines_covered / effective_lines if effective_lines > 0 else 0.0
    )
    effective_branches_cov = (
        effective_branches_covered / effective_branches
        if effective_branches > 0
        else 0.0
    )
    project_line_cov = (
        project_lines_covered / project_lines if project_lines > 0 else 0.0
    )
    project_branch_cov = (
        project_branches_covered / project_branches if project_branches > 0 else 0.0
    )
    average_line_cov = np.mean(line_cov_list)
    average_branch_cov = np.mean(branch_cov_list)
    average_test_lines = np.mean(all_tests_lines_list)

    # Generate HTML
    html_output = [
        "<html>",
        "<head>",
        "<title>Coverage Report</title>",
        "<style>table {border-collapse: collapse; margin: 20px 0;} th, td {border: 1px solid #ddd; padding: 8px;}</style>",
        "</head>",
        "<body>",
        "<h1>Coverage Report</h1>",
        f"<h2>Directory: {escape(target_dir)}</h2>",
        # Summary Table
        "<table>",
        "<tr><th>Metric</th><th>Value</th></tr>",
        f"<tr><td>Number of Functions</td><td>{function_num}</td></tr>",
        f"<tr><td>Generated Tests</td><td>{all_tests_gen}</td></tr>",
        f"<tr><td>Average Tests Lines</td><td>{average_test_lines}</td></tr>",
        f"<tr><td>Compile Success Rate</td><td>{compile_pass_rate*100:.2f}%</td></tr>",
        f"<tr><td>Test Pass Rate</td><td>{tests_pass_rate*100:.2f}%</td></tr>",
        f"<tr><td>Oracles</td><td>{all_oracles_gen}</td></tr>",
        f"<tr><td>Oracles Compile Success Rate</td><td>{oracles_compile_pass_rate*100:.2f}%</td></tr>",
        f"<tr><td>Oracles Pass Rate</td><td>{oracles_tests_pass_rate*100:.2f}%</td></tr>",
        f"<tr><td>Average Line Coverage</td><td>{average_line_cov:.2f}%</td></tr>",
        f"<tr><td>Average Branch Coverage</td><td>{average_branch_cov:.2f}%</td></tr>",
        f"<tr><td>Project Line Coverage</td><td>{project_line_cov*100:.2f}%</td></tr>",
        f"<tr><td>Project Branch Coverage</td><td>{project_branch_cov*100:.2f}%</td></tr>",
        f"<tr><td>Effective Line Coverage</td><td>{effective_lines_cov*100:.2f}%</td></tr>",
        f"<tr><td>Effective Branch Coverage</td><td>{effective_branches_cov*100:.2f}%</td></tr>",
        "</table>",
        # Detailed Table
        "<h3>Function Details</h3>",
        "<table>",
        "<tr>",
        "<th>Function</th><th>Lines</th><th>Covered</th><th>Line Cov%</th>",
        "<th>Branches</th><th>Covered</th><th>Branch Cov%</th>",
        "<th>Tests</th><th>Passed</th><th>Pass Rate%</th>",
        "</tr>",
    ]

    for info in test_rate_infos:
        test_pass_rate = (
            info["oracles_passed"] / info["oracles_gen"] * 100
            if info["tests_run"] > 0
            else 0.0
        )
        html_output.append(
            f"<tr>"
            f"<td>{escape(info['function_name'])}</td>"
            f"<td>{info['lines']}</td><td>{info['lines_covered']}</td><td>{info['line_coverage']:.2f}</td>"
            f"<td>{info['branches']}</td><td>{info['branches_covered']}</td><td>{info['branch_coverage']:.2f}</td>"
            f"<td>{info['oracles_gen']}</td><td>{info['oracles_passed']}</td><td>{test_pass_rate:.2f}</td>"
            "</tr>"
        )

    html_output.extend(["</table>", "</body>", "</html>"])

    # Save report
    report_path = Path(target_dir) / "coverage_report.html"
    with open(report_path, "w") as f:
        f.write("\n".join(html_output))


def main():
    arg = sys.argv[1]
    crate_directory = arg
    target_dirs = [
        os.path.join(crate_directory, p)
        for p in [
            "rutgen_N/result",
            "rutgen_r/result",
            "rutgen_c/result",
            "rutgen_r_c/result",
            "rutgen_r_c/fixed_result",
            "rutgen_r_c_o/result",
            "rutgen_r_c_o/fixed_result",
        ]
    ]

    # Pre-calculate includes for all target directories
    includes = get_includes(target_dirs)

    for target_dir in target_dirs:
        if os.path.exists(target_dir):
            # print(target_dir)
            generate_html_report(target_dir, includes)
        else:
            print(f"Directory not found: {target_dir}")


if __name__ == "__main__":
    main()
