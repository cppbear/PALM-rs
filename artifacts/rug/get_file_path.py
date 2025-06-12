import json
import os


def main():
    generations = [p for p in ["rutgen_N", "rutgen_r", "rutgen_c", "rutgen_r_c", "rutgen_r_c_o"]]
    for item in os.listdir("."):
        item_path = os.path.join(".", item)
        if item_path == "myvenv" or not os.path.isdir(item_path):
            continue
        for generation in generations:
            target_path = os.path.join(item_path, generation)
            generation_path = os.path.join(target_path, "generation/pre_fix")
            result_path = os.path.join(target_path, "result")
            print(generation_path)
            print(result_path)
            file_locs = {}
            # 构建文件名到loc的映射
            if os.path.exists(generation_path):
                for root, _, files in os.walk(generation_path):
                    for file in files:
                        if file.endswith(".json"):
                            with open(os.path.join(root, file), "r") as f:
                                data = json.load(f)
                                loc = data.get("loc")  # 使用get避免KeyError
                                loc = os.path.join(item_path,loc)
                                loc=os.path.abspath(loc)
                                # loc=loc.replace("/home/abezbm/rust-utgen-test-crates-rumono","/home/rumono")
                                # print(loc)
                                if loc is not None:
                                    file_locs[file] = loc.split(":")[0]
            # print(file_locs)
            # 处理result_path中的文件
            if os.path.exists(result_path):
                for root, _, files in os.walk(result_path):
                    for file in files:
                        if file.endswith(".json"):
                            result_file = os.path.join(root, file)
                            with open(result_file, "r") as f:
                                data = json.load(f)
                            # 检查并添加file_path
                            if file in file_locs:
                                data["file_path"] = file_locs[file]
                                with open(result_file, "w") as f_write:
                                    json.dump(data, f_write, indent=4)
            generation_path = os.path.join(target_path, "generation/llm_fix")
            result_path = os.path.join(target_path, "fixed_result")
            print(generation_path)
            print(result_path)
            file_locs = {}
            # 构建文件名到loc的映射
            if os.path.exists(generation_path):
                for root, _, files in os.walk(generation_path):
                    for file in files:
                        if file.endswith(".json"):
                            with open(os.path.join(root, file), "r") as f:
                                data = json.load(f)
                                loc = data.get("loc")  # 使用get避免KeyError
                                loc = os.path.join(item_path,loc)
                                loc=os.path.abspath(loc)
                                # loc=loc.replace("/home/abezbm/rust-utgen-test-crates-rumono","/home/rumono")
                                # print(loc)
                                if loc is not None:
                                    file_locs[file] = loc.split(":")[0]
            # print(file_locs)
            # 处理result_path中的文件
            if os.path.exists(result_path):
                for root, _, files in os.walk(result_path):
                    for file in files:
                        if file.endswith(".json"):
                            result_file = os.path.join(root, file)
                            with open(result_file, "r") as f:
                                data = json.load(f)
                            # 检查并添加file_path
                            if file in file_locs:
                                data["file_path"] = file_locs[file]
                                with open(result_file, "w") as f_write:
                                    json.dump(data, f_write, indent=4)
            

if __name__ == "__main__":
    main()