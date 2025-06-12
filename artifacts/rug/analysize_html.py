import argparse
import json
import os
from html import escape
from pathlib import Path
import sys

import numpy as np
import xml.etree.ElementTree as ET


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
  
class BranchInfo:
    start_line = 0
    start_column = 0
    end_line = 0
    end_column = 0
    positive = False
    negative = False

    def __init__(
        self, start_line, start_column, end_line, end_column, positive, negative
    ):
        self.start_line = start_line
        self.start_column = start_column
        self.end_line = end_line
        self.end_column = end_column
        self.positive = positive
        self.negative = negative

    def __eq__(self, value):
        return (
            self.start_line == value.start_line
            and self.start_column == value.start_column
            and self.end_line == value.end_line
            and self.end_column == value.end_column
        )

    def __repr__(self):
        return f"start_line={self.start_line}, start_column={self.start_column}, end_line={self.end_line}, end_column={self.end_column}, positive={self.positive}, negative={self.negative}\n"

  
        
class Function:
    function_name=""
    file_path=""
    lines=[]
    lines_covered=[]
    branches = []
    branches_covered = []
    has_run=False
    
    def __init__(self, file_path, function_name, lines, branches,has_run):
        self.file_path = file_path
        self.function_name = function_name
        self.lines = lines
        self.lines_covered = []
        self.branches = branches
        self.branches_covered = []
        self.has_run=has_run
    
    def __eq__(self, other):
        if not isinstance(other, Function):
            return True
        return (    
            self.file_path == other.file_path
            and self.function_name == other.function_name
        )

    def __hash__(self):
        return hash(self.file_path + self.function_name)

    def __repr__(self):
        return f"file_path={self.file_path!r}, function_name={self.function_name!r}, lines={self.lines}, lines_covered={self.lines_covered}, branches={self.branches}, branches_covered={self.branches_covered}\n"


def main():
    generations = [p for p in ["rutgen_N/result", "rutgen_r/result", "rutgen_c/result", "rutgen_r_c/result", "rutgen_r_c/fixed_result", "rutgen_r_c_o/result", "rutgen_r_c_o/fixed_result"]]
    for item in os.listdir("."):
        item_path = os.path.join(".", item)
        if item == "myvenv" or not os.path.isdir(item_path):
            continue
        print(item_path)
        functions=set()
        useful_functions=set()
        for generation in generations:
            if generation == "rutgen_r_c_o/result":
                target_path = os.path.join(item_path, generation)
                if os.path.exists(target_path):
                    for root, _, files in os.walk(target_path):
                        for file in files:
                            if file.endswith(".json"):
                                with open(os.path.join(root, file), "r") as f:
                                    data = json.load(f)
                                    function_name=data["function_name"]
                                    file_path=data["file_path"]
                                    codes_lines=data["codes_lines"]
                                    codes_branches=data["codes_branches"]
                                    branches = []
                                    for codes_branch in codes_branches:
                                        branch = BranchInfo(
                                            codes_branch["start_line"],
                                            codes_branch["start_column"],
                                            codes_branch["end_line"],
                                            codes_branch["end_column"],
                                            codes_branch["positive"],
                                            codes_branch["negative"],
                                        )
                                        branches.append(branch)
                                    oracles_fun=int(data["oracles_run"])
                                    has_run=True if oracles_fun>0 else False
                                    functions.add(Function(file_path,function_name,codes_lines,branches,has_run))
                                    if oracles_fun>0:
                                        useful_functions.add(Function(file_path,function_name,codes_lines,branches,True))
            else:
                target_path = os.path.join(item_path, generation)
                if os.path.exists(target_path):
                    for root, _, files in os.walk(target_path):
                        for file in files:
                            if file.endswith(".json"):
                                with open(os.path.join(root, file), "r") as f:
                                    data = json.load(f)
                                    function_name=data["function_name"]
                                    file_path=data["file_path"]
                                    codes_lines=data["codes_lines"]
                                    codes_branches=data["codes_branches"]
                                    branches = []
                                    for codes_branch in codes_branches:
                                        branch = BranchInfo(
                                            codes_branch["start_line"],
                                            codes_branch["start_column"],
                                            codes_branch["end_line"],
                                            codes_branch["end_column"],
                                            codes_branch["positive"],
                                            codes_branch["negative"],
                                        )
                                        branches.append(branch)
                                    oracles_fun=int(data["oracles_run"])
                                    if oracles_fun>0:
                                        useful_functions.add(Function(file_path,function_name,codes_lines,branches,True))
                                        
        # print(useful_functions)
        # print(functions)
        if not os.path.exists(os.path.join(item_path, "coverage.xml")):
            continue
        for function in functions:
            if not os.path.exists(os.path.join(item_path, "coverage.xml")):
                continue
            tree = ET.parse(os.path.join(item_path, "coverage.xml"))
            root = tree.getroot()
            sources = [s.text for s in root.findall(".//source")]
            if len(sources) != 1:
                print("There are sources more than one")
            classes = root.findall(".//class")
            for class_elem in classes:
                file_path = class_elem.get("filename")
                if file_path == function.file_path or os.path.abspath(os.path.join(item_path, file_path)) == function.file_path:
                    lines = class_elem.find("lines").findall(".//line")
                    for line in lines:
                        line_number = int(line.get("number"))
                        # print(function)
                        if (
                            line_number >= function.lines[0]
                            and line_number
                            <= function.lines[-1]
                        ):
                            # if line_number in original_file_coverage_information.lines:
                            hits = int(line.get("hits"))
                            if hits > 0:
                                if (
                                    line_number
                                    not in function.lines_covered
                                ):
                                    function.lines_covered.append(
                                        line_number
                                    )
                                    function.lines_covered.sort()
            if not os.path.exists(os.path.join(item_path, "coverage.json")):
                continue
            with open(os.path.join(item_path, "coverage.json"), "r") as f:
                data = json.load(f).get("data")
                files = data[0]["files"]
                for file in files:
                    file_path = file["filename"]
                    if function.file_path == file_path or os.path.abspath(os.path.join(item_path, file_path)) == function.file_path:
                        branches = file["branches"]
                        for branch in branches:
                            branch_info = BranchInfo(
                                branch[0],
                                branch[1],
                                branch[2],
                                branch[3],
                                True if branch[4] > 0 else False,
                                True if branch[5] > 0 else False,
                            )
                            if (
                                branch_info.start_line
                                >= function.lines[0]
                                and branch_info.end_line
                                <= function.lines[-1]
                            ):
                                has_branch_covered = False
                                for (
                                    branch_covered
                                ) in (
                                    function.branches_covered
                                ):
                                    if branch_info == branch_covered:
                                        if branch_info.positive == True:
                                            branch_covered.positive = True
                                        if branch_info.negative == True:
                                            branch_covered.negative = True
                                        has_branch_covered = True
                                        break
                                if not has_branch_covered:
                                    function.branches_covered.append(
                                        branch_info
                                    )
        
        lines_cov=[]
        branches_cov=[]
        lines=0
        lines_covered=0
        branches=0
        branches_covered=0
        effective_lines=0
        effective_lines_covered=0
        effective_branches=0
        effective_branches_covered=0
        for function in functions:
            if function.branches == []:
                function.branches = [BranchInfo(0, 0, 0, 0, False, False)]
                if not function.lines_covered == []:
                    function.branches_covered = [
                        BranchInfo(0, 0, 0, 0, True, True)
                    ]
            if len(function.lines_covered)>0:
                lines_cov.append(len(function.lines_covered)/len(function.lines)*100)
                bc=0
                for b in function.branches_covered:
                    if b.positive:
                        bc+=1
                    if b.negative:
                        bc+=1
                if len(function.branches)>0:    
                    branches_cov.append(bc/len(function.branches)/2*100 if len(function.branches)>0 else 0)
            lines+=len(function.lines)
            lines_covered+=len(function.lines_covered)
            branches+=len(function.branches)*2
            bc=0
            for b in function.branches_covered:
                if b.positive:
                    bc+=1
                if b.negative:
                    bc+=1
            branches_covered+=bc
            
            if function in useful_functions:
                effective_lines+=len(function.lines)
                effective_lines_covered+=len(function.lines_covered)
                effective_branches+=len(function.branches)*2
                bc=0
                for b in function.branches_covered:
                    if b.positive:
                        bc+=1
                    if b.negative:
                        bc+=1
                effective_branches_covered+=bc
                
        average_line=np.mean(lines_cov)
        average_branch=np.mean(branches_cov)
        effective_line=effective_lines_covered/effective_lines*100
        effective_branch=effective_branches_covered/effective_branches*100
        project_line=lines_covered/lines*100
        project_branch=branches_covered/branches*100
        
        print(f"{item}\n")
        print(f"average line:{average_line}\t average branch:{average_branch}\n")
        print(f"effective line:{effective_line}\t effective branch:{effective_branch}\n")
        print(f"project line:{project_line}\t project branch:{project_branch}\n")
        print("\n")
        


if __name__ == "__main__":
    main()
