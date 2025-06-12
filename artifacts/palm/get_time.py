import subprocess
import time

start_time=time.perf_counter()
subprocess.run(
                        "rutgen gen -p . -r -c",
                        shell=True,
                        capture_output=True,
                        cwd="./bytes",
                    )
end_time=time.perf_counter()
elapsed = end_time - start_time
print(f"bytes: {elapsed:.6f} seconds")

start_time=time.perf_counter()
subprocess.run(
                        "rutgen gen -p . -r -c",
                        shell=True,
                        capture_output=True,
                        cwd="./fastrand",
                    )
end_time=time.perf_counter()
elapsed = end_time - start_time
print(f"fastrand: {elapsed:.6f} seconds")

start_time=time.perf_counter()
subprocess.run(
                        "rutgen gen -p . -r -c",
                        shell=True,
                        capture_output=True,
                        cwd="./hashbrown",
                    )
end_time=time.perf_counter()
elapsed = end_time - start_time
print(f"hashbrown: {elapsed:.6f} seconds")

start_time=time.perf_counter()
subprocess.run(
                        "rutgen gen -p . -r -c",
                        shell=True,
                        capture_output=True,
                        cwd="./http",
                    )
end_time=time.perf_counter()
elapsed = end_time - start_time
print(f"http: {elapsed:.6f} seconds")

start_time=time.perf_counter()
subprocess.run(
                        "rutgen gen -p . -r -c",
                        shell=True,
                        capture_output=True,
                        cwd="./indexmap",
                    )
end_time=time.perf_counter()
elapsed = end_time - start_time
print(f"indexmap: {elapsed:.6f} seconds")

start_time=time.perf_counter()
subprocess.run(
                        "rutgen gen -p . -r -c",
                        shell=True,
                        capture_output=True,
                        cwd="./itoa",
                    )
end_time=time.perf_counter()
elapsed = end_time - start_time
print(f"itoa: {elapsed:.6f} seconds")

start_time=time.perf_counter()
subprocess.run(
                        "rutgen gen -p . -r -c",
                        shell=True,
                        capture_output=True,
                        cwd="./json",
                    )
end_time=time.perf_counter()
elapsed = end_time - start_time
print(f"json: {elapsed:.6f} seconds")

start_time=time.perf_counter()
subprocess.run(
                        "rutgen gen -p . -r -c",
                        shell=True,
                        capture_output=True,
                        cwd="./once_cell",
                    )
end_time=time.perf_counter()
elapsed = end_time - start_time
print(f"once_cell: {elapsed:.6f} seconds")

start_time=time.perf_counter()
subprocess.run(
                        "rutgen gen -p . -r -c",
                        shell=True,
                        capture_output=True,
                        cwd="./quote",
                    )
end_time=time.perf_counter()
elapsed = end_time - start_time
print(f"quote: {elapsed:.6f} seconds")

start_time=time.perf_counter()
subprocess.run(
                        "rutgen gen -p . -w .,./rand_chacha,./rand_core -r -c",
                        shell=True,
                        capture_output=True,
                        cwd="./rand",
                    )
end_time=time.perf_counter()
elapsed = end_time - start_time
print(f"rand: {elapsed:.6f} seconds")

start_time=time.perf_counter()
subprocess.run(
                        "rutgen gen -p . -w .,./regex-capi,./regex-debug,./regex-syntax -r -c",
                        shell=True,
                        capture_output=True,
                        cwd="./regex",
                    )
end_time=time.perf_counter()
elapsed = end_time - start_time
print(f"regex: {elapsed:.6f} seconds")

start_time=time.perf_counter()
subprocess.run(
                        "rutgen gen -p . -r -c",
                        shell=True,
                        capture_output=True,
                        cwd="./rust-base64",
                    )
end_time=time.perf_counter()
elapsed = end_time - start_time
print(f"rust-base64: {elapsed:.6f} seconds")

start_time=time.perf_counter()
subprocess.run(
                        "rutgen gen -p . -r -c",
                        shell=True,
                        capture_output=True,
                        cwd="./ryu",
                    )
end_time=time.perf_counter()
elapsed = end_time - start_time
print(f"ryu: {elapsed:.6f} seconds")

start_time=time.perf_counter()
subprocess.run(
                        "rutgen gen -p . -w ./serde -r -c",
                        shell=True,
                        capture_output=True,
                        cwd="./serde",
                    )
end_time=time.perf_counter()
elapsed = end_time - start_time
print(f"serde: {elapsed:.6f} seconds")

start_time=time.perf_counter()
subprocess.run(
                        "rutgen gen -p . -r -c",
                        shell=True,
                        capture_output=True,
                        cwd="./strsim-rs",
                    )
end_time=time.perf_counter()
elapsed = end_time - start_time
print(f"strsim-rs: {elapsed:.6f} seconds")

start_time=time.perf_counter()
subprocess.run(
                        "rutgen gen -p . -r -c",
                        shell=True,
                        capture_output=True,
                        cwd="./thiserror",
                    )
end_time=time.perf_counter()
elapsed = end_time - start_time
print(f"thiserror: {elapsed:.6f} seconds")