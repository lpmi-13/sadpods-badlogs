#! /usr/bin/python3
# test python writing to a file

import os
import random
import time
from datetime import datetime

VERBOSE_LOGS = os.getenv('PYTHON_LOG', '0')

if VERBOSE_LOGS == '1':
    f = open('/var/log/badpython.log', 'a')
    while True:
        r = random.randrange(2147483647)
        f.write(str(datetime.now()) + ' token: ' + str(r) + '\n')
        f.flush()
        time.sleep(1)
else:
    while True:
        time.sleep(1)

