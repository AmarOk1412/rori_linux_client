#!/usr/bin/env python3
import speech_recognition as sr
import requests

r = sr.Recognizer()
m = sr.Microphone()

try:
    print("A moment of silence, please...")
    with m as source: r.adjust_for_ambient_noise(source)
    print(f"Set minimum energy threshold to {r.energy_threshold}")
    while True:
        requests.get(url = "http://localhost:3000/startListen")
        print("Say something!")
        with m as source: audio = r.listen(source)
        requests.get(url = "http://localhost:3000/stopListen")
        print("Got it! Now to recognize it...")
        try:
            value = r.recognize_sphinx(audio)
            print(f"You said {value}")
            print(requests.post(url = "http://localhost:3000/say", json = { "say" : value }))
        except sr.UnknownValueError:
            print("Oops! Didn't catch that")
        except sr.RequestError as e:
            print(f"Uh oh! Couldn't request results from Sphinx service; {e}")
        except:
            print("Bye.")
            break
except KeyboardInterrupt:
    pass

