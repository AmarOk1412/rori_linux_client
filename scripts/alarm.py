import time
import sys
import datetime
import subprocess

arg = sys.argv[1].split(":")
hour = int(arg[0])
minutes = int(arg[1])
seconds = hour*3600 + minutes*60

currenttime = datetime.datetime.time(datetime.datetime.now())
currentsec = currenttime.hour*3600 + currenttime.minute*60

if currentsec > seconds:
    seconds += 24*3600

seconds -= currentsec

print("sleep " + str(seconds))
time.sleep(seconds)

to_say = "Hei! Good morning here! It's time to begin new tests!"

subprocess.call("./mimic/bin/mimic -t \"" + to_say + "\"  -voice mimic/voices/cmu_us_slt.flitevox -pw --setf int_f0_target_mean=200 --setf duration_stretch=0.8", shell=True)
subprocess.call("python3 scripts/music.py start", shell=True)
