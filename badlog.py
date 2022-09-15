#! /usr/bin/python3
# test python writing to a file

import random
import time
from datetime import datetime

f = open('/var/log/bad.log', 'a')
while True:
  r = random.randrange(2147483647)
  f.write(str(datetime.now()) + ' token: ' + str(r) + '\n')
  f.flush()
  time.sleep(1)
