import sys, glob, os
import hashlib

# BUF_SIZE is totally arbitrary, change for your app!
BUF_SIZE = 65536  # lets read stuff in 64kb chunks!

dirname = sys.argv[1]

for n in glob.glob(dirname+"/*"):
    sha1 = hashlib.sha1()

    with open(n, 'rb') as f:
        while True:
            data = f.read(BUF_SIZE)
            if not data:
                break
            sha1.update(data)

    h = str(sha1.hexdigest())

    print(n + " -> " + h)

    newfilename = os.path.basename(n) + ".sha256"

    with open(os.path.join(dirname, newfilename), "w") as k:
        k.write(h)