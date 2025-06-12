
import os


if __name__ == '__main__':
    # execute_one_fd(sys.argv[1])
    args = []
    for fd in os.listdir('.'):
        if not os.path.isdir(fd) or fd=="myvenv" or fd==".git":
            continue
        item_path=os.path.join(".",fd)
        item_path=os.path.join(item_path,"rutgen_r_c_o/generation/llm_fix")
        if not os.path.exists(item_path):
            print("{} wrong".format(fd))
            exit(1)
        num=0
        for f in os.listdir(item_path):
            if f.endswith(".json"):
                num+=1
        print("{} {}\n".format(fd,num))