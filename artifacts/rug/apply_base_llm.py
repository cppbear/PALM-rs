import subprocess
import os
import sys
import json
import copy
import time
import multiprocessing


def load_analysis(f: str):
    ans = None
    with open(f, 'r') as fp:
        ans = json.load(fp)
    return ans


def execute_one_fd(fd:str):
    fin = subprocess.run('~/.cargo/bin/ws', shell=True, cwd=fd, capture_output=True)
    uid = 0
    total = 0
    succeed = 0
    skip = 0
    with open(fd+'/inject.log', 'w') as sys.stdout:
        for l in fin.stdout.decode('utf-8').splitlines():
            ls = l.split(' ')
            crate = ls[0].strip()
            path = ls[-1]
            ret = subprocess.run("cargo test --no-run", shell=True, cwd=fd+'/'+path, capture_output=True)
            if ret.returncode != 0:
                print('err cargo test', fd, crate, path)
                continue
            if os.path.exists('{}/{}.json'.format(fd, crate)) and os.path.exists("{}/{}.gpt.json".format(fd, crate)):
                data = load_analysis('{}/{}.json'.format(fd, crate))
                llm = load_analysis('{}/{}.gpt.json'.format(fd, crate))
                targets = data['targets']
                dependencies = data['dependencies']
                srcs = data['srcs']
                struct_to_trait = data['struct_to_trait']
                trait_to_struct = data['trait_to_struct']
                self_to_fn = data['self_to_fn']
                crate = crate.replace('-','_')
                for target, meta in targets.items():
                    uid += 1
                    func_name = meta[0]
                    filename = meta[1][meta[1].find('"')+1:meta[1].rfind('"')]
                    if target not in llm:
                        continue
                    program = llm[target]
                    program = program.replace('```rust', '').replace('```Rust', '').replace('```', '')
                    ls = program.splitlines(keepends=True)
                    for i in range(len(ls)):
                        l = ls[i]
                        if 'mod tests' in l:
                            ls[i] = l.replace('mod tests', 'mod tests_llm_16_{}'.format(uid))
                        if '{}::'.format(crate) in l:
                            idx = ''.find('{}::'.format(crate))
                            if idx > 0 and l[idx-1] in 'qazwsxedcrfvtgbyhnujmikolp:':
                                continue
                            ls[i] = l.replace('{}::'.format(crate), 'crate::')
                        if 'use super::*' in l:
                            ls[i] = l+'\nuse crate::*;\n'
                        elif 'use super::' in l:
                            ls[i] = l.replace('use super::', 'use crate::')
                    # print("".join(ls))
                    # try inject
                    if not os.path.exists(fd+'/'+filename):
                        print('file not found', target, fd, filename)
                        continue
                    total += 1
                    with open(fd+'/'+filename, 'r+') as fp:
                        origins = fp.readlines()
                        mutate = copy.deepcopy(origins)
                        mutate.extend(ls)
                        fp.truncate(0)
                        fp.seek(0)
                        fp.writelines(mutate)
                        fp.flush()
                        ret = subprocess.run("cargo test --message-format=json --no-run", shell=True, cwd=fd+'/'+path, capture_output=True)
                        if ret.returncode == 0:
                            print('inject succeed', fd, crate, target)
                            succeed += 1
                        else:
                            fp.truncate(0)
                            fp.seek(0)
                            fp.writelines(origins)
                            fp.flush()
                            print('='*40)
                            print('inject err', target, filename, func_name)
                            print(''.join(ls))
                            print(ret.stdout.decode('utf-8'))
        print(fd, succeed, total)


if __name__ == '__main__':
    # execute_one_fd(sys.argv[1])
    args = []
    for fd in os.listdir('.'):
        if not os.path.isdir(fd):
            continue
        ret = subprocess.run("cargo test --no-run", shell=True, cwd=fd, capture_output=True)
        if ret.returncode == 0:
            args.append(fd)
    print(args)
    with multiprocessing.Pool(16) as p:
        p.map(execute_one_fd, args)
