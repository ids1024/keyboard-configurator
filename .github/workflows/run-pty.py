#!/usr/bin/env python3

import os
import sys
import pty
import subprocess

pid, fd = pty.fork()
if pid == 0:
    sys.exit(subprocess.call(sys.argv[1:]))
else:
    try:
        for l in os.fdopen(fd):
            sys.stdout.write(l)
    except Exception:
        pass
    sys.exit(os.WEXITSTATUS(os.waitpid(pid, 0)[1]))
