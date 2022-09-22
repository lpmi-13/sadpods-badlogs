#! /usr/bin/python3
# test python writing to a file

import os
import random
import time
from datetime import datetime

VERBOSE_LOGS = os.getenv('LOG_LEVEL', '0')

if VERBOSE_LOGS == '1':
    f = open('/var/log/bad.log', 'a')
    while True:
        r = random.randrange(1111111111, 9999999999)
        f.write(str(datetime.now()) + ' token: ' + str(r) + '\n')
        f.flush()
        time.sleep(1)
else:
    while True:
        time.sleep(1)

