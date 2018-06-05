import sys
import subprocess

if len(sys.argv) > 1:
    if sys.argv[1] == "start":
        print("start music")
        subprocess.Popen("rhythmbox-client --play", shell=True)
    if sys.argv[1] == "stop":
        print("stop music")
        subprocess.Popen("rhythmbox-client --stop", shell=True)
    if sys.argv[1] == "pause":
        print("pause music")
        subprocess.Popen("rhythmbox-client --pause", shell=True)
    if sys.argv[1] == "next":
        print("next music")
        subprocess.Popen("rhythmbox-client --next", shell=True)
    if sys.argv[1] == "previous":
        print("previous music")
        subprocess.Popen("rhythmbox-client --previous", shell=True)
        subprocess.Popen("rhythmbox-client --previous", shell=True)
